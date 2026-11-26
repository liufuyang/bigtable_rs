//! Connections whose channels are managed in the background.
//!
//! The Bigtable service closes connections that have been open for an hour, and connections
//! that sit idle go cold, so the next request through them is slow. The connections built
//! here avoid both, the way the official clients do: every channel is replaced before the
//! server closes it, and every channel periodically receives a `PingAndWarm` request.

use std::convert::Infallible;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Stream;
use gcp_auth::TokenProvider;
use http::Request;
use log::{debug, info, warn};
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tonic::body::Body;
use tonic::metadata::MetadataMap;
use tonic::transport::{Channel, Endpoint};
use tonic::Extensions;
use tower::balance::p2c::Balance;
use tower::buffer::Buffer;
use tower::discover::Change;
use tower::load::{CompleteOnResponse, PendingRequests};
use tower::Service;

use super::{
    create_client, create_emulator_endpoint, create_endpoint, insert_instance_routing_header,
    BigTableConnection, Error, Result,
};
use crate::auth_service::AuthSvc;
use crate::google::bigtable::v2::{bigtable_client::BigtableClient, PingAndWarmRequest};
use crate::transport::Transport;

/// The server closes connections after an hour, so replace them comfortably before that.
const DEFAULT_MAX_CHANNEL_AGE: Duration = Duration::from_secs(50 * 60);
const DEFAULT_PING_AND_WARM_INTERVAL: Duration = Duration::from_secs(30);

/// Builds a [`BigTableConnection`] whose channels are managed in the background.
///
/// Created with [`BigTableConnection::managed`]. Compared to the connections created
/// by [`BigTableConnection::new`], a managed connection:
///
/// - replaces every channel once it is older than [`max_channel_age`](Self::max_channel_age),
///   before the server closes it;
/// - sends a `PingAndWarm` request through every channel each
///   [`ping_and_warm_interval`](Self::ping_and_warm_interval), so that no channel goes cold
///   when traffic is low;
/// - [primes](Self::prime_channels) every channel with a `PingAndWarm` request before it
///   receives traffic.
///
/// The background tasks doing this stop once the connection and all of the clients created
/// from it are dropped.
///
/// ```rust,no_run
/// use bigtable_rs::bigtable::BigTableConnection;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connection = BigTableConnection::managed("project-1", "instance-1")
///     .channel_size(4)
///     .timeout(Duration::from_secs(10))
///     .build()
///     .await?;
/// let mut bigtable = connection.client();
/// # Ok(())
/// # }
/// ```
pub struct ManagedConnectionBuilder {
    project_id: String,
    instance_name: String,
    is_read_only: bool,
    channel_size: usize,
    timeout: Option<Duration>,
    token_provider: Option<Arc<dyn TokenProvider>>,
    max_channel_age: Option<Duration>,
    ping_and_warm_interval: Option<Duration>,
    prime_channels: bool,
    ping_and_warm_app_profile_id: String,
    emulator_endpoint: Option<String>,
}

impl ManagedConnectionBuilder {
    pub(super) fn new(project_id: &str, instance_name: &str) -> Self {
        Self {
            project_id: project_id.to_owned(),
            instance_name: instance_name.to_owned(),
            is_read_only: false,
            channel_size: 1,
            timeout: None,
            token_provider: None,
            max_channel_age: Some(DEFAULT_MAX_CHANNEL_AGE),
            ping_and_warm_interval: Some(DEFAULT_PING_AND_WARM_INTERVAL),
            prime_channels: true,
            ping_and_warm_app_profile_id: String::new(),
            emulator_endpoint: None,
        }
    }

    /// Request the read-only OAuth2 scope instead of the read/write one. Defaults to `false`.
    pub fn read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    /// Set the number of channels that requests are load balanced onto. Defaults to 1, and
    /// zero is treated as 1.
    ///
    /// See [`BigTableConnection::new`] for guidance on choosing a value.
    pub fn channel_size(mut self, channel_size: usize) -> Self {
        self.channel_size = channel_size;
        self
    }

    /// Set a timeout for every request, including the `PingAndWarm` requests sent in the
    /// background. There is no timeout by default.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Use the given token provider instead of discovering credentials from the environment.
    pub fn token_provider(mut self, token_provider: Arc<dyn TokenProvider>) -> Self {
        self.token_provider = Some(token_provider);
        self
    }

    /// Set the age at which channels are replaced. Defaults to 50 minutes, as the server
    /// closes connections after an hour. `None` disables replacing channels.
    ///
    /// If a replacement channel cannot be created, the existing channel stays in use and is
    /// retried the next time around.
    pub fn max_channel_age(mut self, max_channel_age: Option<Duration>) -> Self {
        self.max_channel_age = max_channel_age;
        self
    }

    /// Set how often a `PingAndWarm` request is sent through every channel. Defaults to 30
    /// seconds. `None` disables these requests.
    pub fn ping_and_warm_interval(mut self, ping_and_warm_interval: Option<Duration>) -> Self {
        self.ping_and_warm_interval = ping_and_warm_interval;
        self
    }

    /// Set whether a channel is connected and sent a `PingAndWarm` request before it receives
    /// any traffic. Defaults to `true`.
    ///
    /// When enabled, [`build`](Self::build) fails if any of the initial channels cannot be
    /// connected or primed. When disabled, channels connect lazily on their first request.
    pub fn prime_channels(mut self, prime_channels: bool) -> Self {
        self.prime_channels = prime_channels;
        self
    }

    /// Set the app profile that `PingAndWarm` requests are sent with. Defaults to the
    /// instance's default app profile.
    ///
    /// This only applies to the `PingAndWarm` requests sent by the connection itself: every
    /// other request carries the app profile set on that request.
    pub fn ping_and_warm_app_profile_id(mut self, app_profile_id: impl Into<String>) -> Self {
        self.ping_and_warm_app_profile_id = app_profile_id.into();
        self
    }

    /// Connect to a Bigtable emulator at the given endpoint, either `host:port` or
    /// `unix:///path/to/unix.sock`, instead of the Bigtable service. This takes precedence over
    /// the `BIGTABLE_EMULATOR_HOST` environment variable.
    ///
    /// The connection to an emulator is unauthenticated, and otherwise managed in the same
    /// way.
    pub fn emulator_endpoint(mut self, emulator_endpoint: impl Into<String>) -> Self {
        self.emulator_endpoint = Some(emulator_endpoint.into());
        self
    }

    /// Establish the connection.
    ///
    /// Unless a [token provider](Self::token_provider) was given, credentials are discovered
    /// from the environment, as in [`BigTableConnection::new`].
    ///
    /// The `BIGTABLE_EMULATOR_HOST` environment variable is also respected, and has the same
    /// effect as [`emulator_endpoint`](Self::emulator_endpoint).
    pub async fn build(self) -> Result<BigTableConnection> {
        for (name, interval) in [
            ("max_channel_age", self.max_channel_age),
            ("ping_and_warm_interval", self.ping_and_warm_interval),
        ] {
            if interval.is_some_and(|interval| interval.is_zero()) {
                return Err(Error::InvalidArgument(format!(
                    "{name} must not be zero; use None to disable it"
                )));
            }
        }

        let instance_prefix = format!(
            "projects/{}/instances/{}",
            self.project_id, self.instance_name
        );
        let table_prefix = format!("{instance_prefix}/tables/");
        // Fail now, rather than on every request sent in the background, if the app profile
        // cannot be sent as a header.
        let ping_and_warm = PingAndWarm::new(&instance_prefix, &self.ping_and_warm_app_profile_id)?;

        let emulator_endpoint = self
            .emulator_endpoint
            .or_else(|| std::env::var("BIGTABLE_EMULATOR_HOST").ok());
        let (endpoint, token_provider) = match emulator_endpoint {
            Some(emulator_endpoint) => {
                info!("Connecting to bigtable emulator at {emulator_endpoint}");
                let endpoint = create_emulator_endpoint(&emulator_endpoint, self.timeout)?;
                (endpoint, None)
            }
            None => {
                let token_provider = match self.token_provider {
                    Some(token_provider) => token_provider,
                    None => gcp_auth::provider().await?,
                };
                (create_endpoint(self.timeout)?, Some(token_provider))
            }
        };
        let factory = ChannelFactory {
            endpoint,
            token_provider: token_provider.clone(),
            is_read_only: self.is_read_only,
            prime: self.prime_channels.then(|| ping_and_warm.clone()),
        };
        let pool = PoolSettings {
            channel_size: self.channel_size.max(1),
            max_channel_age: self.max_channel_age,
            ping_and_warm_interval: self.ping_and_warm_interval,
        };
        let transport = ManagedTransport::connect(factory, ping_and_warm, pool).await?;

        Ok(BigTableConnection {
            client: create_client(transport, token_provider, self.is_read_only),
            table_prefix: Arc::new(table_prefix),
            instance_prefix: Arc::new(instance_prefix),
            timeout: Arc::new(self.timeout),
        })
    }
}

/// A `PingAndWarm` request for the instance of the connection, with its routing header.
#[derive(Clone)]
struct PingAndWarm {
    message: PingAndWarmRequest,
    metadata: MetadataMap,
}

impl PingAndWarm {
    /// Fails if the app profile cannot be sent as a header.
    fn new(instance_prefix: &str, app_profile_id: &str) -> Result<Self> {
        let mut metadata = MetadataMap::new();
        insert_instance_routing_header(&mut metadata, instance_prefix, app_profile_id)?;
        Ok(Self {
            message: PingAndWarmRequest {
                name: instance_prefix.to_owned(),
                app_profile_id: app_profile_id.to_owned(),
            },
            metadata,
        })
    }

    async fn send(&self, client: &mut BigtableClient<AuthSvc>) -> Result<()> {
        let request = tonic::Request::from_parts(
            self.metadata.clone(),
            Extensions::default(),
            self.message.clone(),
        );
        client.ping_and_warm(request).await?;
        Ok(())
    }
}

/// Opens the channels of a [`ManagedTransport`].
struct ChannelFactory {
    endpoint: Endpoint,
    token_provider: Option<Arc<dyn TokenProvider>>,
    is_read_only: bool,
    /// Sent through every channel before it is used. `None` disables priming.
    prime: Option<PingAndWarm>,
}

impl ChannelFactory {
    /// Opens a channel, priming it if configured to.
    ///
    /// Also returns a client that sends requests through this channel alone, rather than
    /// through whichever channel the load balancer picks.
    async fn open(&self) -> Result<(Channel, BigtableClient<AuthSvc>)> {
        let Some(prime) = &self.prime else {
            let channel = self.endpoint.connect_lazy();
            return Ok((channel.clone(), self.client(channel)));
        };
        let channel = self.endpoint.connect().await?;
        let mut client = self.client(channel.clone());
        prime.send(&mut client).await?;
        Ok((channel, client))
    }

    fn client(&self, transport: impl Into<Transport>) -> BigtableClient<AuthSvc> {
        create_client(transport, self.token_provider.clone(), self.is_read_only)
    }
}

/// How many channels a [`ManagedTransport`] holds, and how often they are replaced and pinged.
#[derive(Clone, Copy)]
struct PoolSettings {
    channel_size: usize,
    max_channel_age: Option<Duration>,
    ping_and_warm_interval: Option<Duration>,
}

type LoadTrackedChannel = PendingRequests<Channel, CompleteOnResponse>;
type ChannelChange = Change<usize, LoadTrackedChannel>;
/// Load balances requests onto the channels of the pool. This is what
/// `tonic::transport::Channel::balance_channel` builds internally, except that the pool is made
/// of channels we own, so that they can be primed before being added to it, and sent
/// `PingAndWarm` requests individually.
type BalancedChannels = Buffer<
    Request<Body>,
    <Balance<ChannelStream, Request<Body>> as Service<Request<Body>>>::Future,
>;

fn load_tracked(channel: Channel) -> LoadTrackedChannel {
    PendingRequests::new(channel, CompleteOnResponse::default())
}

/// A pool of channels, kept fresh by background tasks that stop when the last clone is dropped.
#[derive(Clone)]
pub(crate) struct ManagedTransport {
    channels: BalancedChannels,
    /// Aborts the tasks it contains on drop. The background tasks send requests through the
    /// pool too, with a clone that does not hold this, as they would otherwise never stop.
    _background_tasks: Option<Arc<JoinSet<()>>>,
}

impl ManagedTransport {
    async fn connect(
        factory: ChannelFactory,
        ping_and_warm: PingAndWarm,
        pool: PoolSettings,
    ) -> Result<Self> {
        let (changes, change_receiver) = mpsc::channel(pool.channel_size);
        let (channels, worker) = Buffer::pair(
            Balance::new(ChannelStream {
                changes: change_receiver,
            }),
            1024,
        );
        let mut background_tasks = JoinSet::new();
        background_tasks.spawn(worker);

        let mut channel_clients = Vec::with_capacity(pool.channel_size);
        for key in 0..pool.channel_size {
            let (channel, client) = factory.open().await?;
            channel_clients.push(client);
            // Cannot fail: there is capacity for every channel, and the receiver is alive.
            let _ = changes.try_send(Change::Insert(key, load_tracked(channel)));
        }

        let transport = Self {
            channels,
            _background_tasks: None,
        };
        let manager = ChannelManager {
            balanced_client: factory.client(transport.clone()),
            channel_clients: Mutex::new(channel_clients),
            changes,
            factory,
            ping_and_warm,
            pool,
        };
        background_tasks.spawn(async move { manager.run().await });

        Ok(Self {
            _background_tasks: Some(Arc::new(background_tasks)),
            ..transport
        })
    }
}

impl Service<Request<Body>> for ManagedTransport {
    type Response = <BalancedChannels as Service<Request<Body>>>::Response;
    type Error = <BalancedChannels as Service<Request<Body>>>::Error;
    type Future = <BalancedChannels as Service<Request<Body>>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        self.channels.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        self.channels.call(request)
    }
}

/// The background half of a [`ManagedTransport`].
struct ChannelManager {
    factory: ChannelFactory,
    ping_and_warm: PingAndWarm,
    pool: PoolSettings,
    changes: mpsc::Sender<ChannelChange>,
    /// Sends requests through the load balancer, like the requests of the user.
    balanced_client: BigtableClient<AuthSvc>,
    /// One client per channel of the pool, each sending requests through that channel alone.
    channel_clients: Mutex<Vec<BigtableClient<AuthSvc>>>,
}

impl ChannelManager {
    async fn run(&self) {
        tokio::join!(self.refresh_channels(), self.ping_and_warm_channels());
    }

    /// Replaces every channel each `max_channel_age`.
    ///
    /// This is best-effort: if a replacement cannot be created, the existing channel stays in
    /// the pool until the next round. Should the server close it in the meantime, `tonic`
    /// transparently reconnects it on the next request.
    async fn refresh_channels(&self) {
        let Some(max_channel_age) = self.pool.max_channel_age else {
            return;
        };
        let mut balanced_client = self.balanced_client.clone();
        loop {
            // The load balancer only picks up the changes we sent it when it handles a request.
            // Without any traffic, they would still be queued by the next round, leaving no
            // room to send new ones. Sending a request through the load balancer ourselves
            // guarantees that the queue is empty.
            if let Err(error) = self.ping_and_warm.send(&mut balanced_client).await {
                warn!("Failed to send PingAndWarm through the channel pool: {error}");
            }

            tokio::time::sleep(max_channel_age).await;

            debug!("Refreshing {} channels", self.pool.channel_size);
            for key in 0..self.pool.channel_size {
                let (channel, client) = match self.factory.open().await {
                    Ok(opened) => opened,
                    Err(error) => {
                        warn!("Failed to create a replacement for channel {key}: {error}");
                        continue;
                    }
                };
                match self
                    .changes
                    .try_send(Change::Insert(key, load_tracked(channel)))
                {
                    Ok(()) => self.channel_clients.lock().unwrap()[key] = client,
                    Err(error) => warn!("Failed to replace channel {key}: {error}"),
                }
            }
        }
    }

    /// Sends a `PingAndWarm` request through every channel each `ping_and_warm_interval`.
    async fn ping_and_warm_channels(&self) {
        let Some(interval) = self.pool.ping_and_warm_interval else {
            return;
        };
        let mut ticks = tokio::time::interval(interval);
        ticks.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        // The first tick completes immediately, and the channels have just been created.
        ticks.tick().await;
        loop {
            ticks.tick().await;
            let channel_clients = self.channel_clients.lock().unwrap().clone();
            for mut client in channel_clients {
                if let Err(error) = self.ping_and_warm.send(&mut client).await {
                    debug!("Background PingAndWarm failed: {error}");
                }
            }
        }
    }
}

/// Feeds the changes sent by the [`ChannelManager`] to the load balancer.
///
/// Analogous to `tonic`'s `DynamicServiceStream`, which is not exposed.
pub(crate) struct ChannelStream {
    changes: mpsc::Receiver<ChannelChange>,
}

impl Stream for ChannelStream {
    type Item = std::result::Result<ChannelChange, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.changes.poll_recv(cx) {
            Poll::Ready(Some(change)) => Poll::Ready(Some(Ok(change))),
            // The load balancer fails once this stream ends, but the channels in the pool
            // remain usable after the `ChannelManager` is gone, so never end it.
            Poll::Ready(None) | Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::future::Future;
    use std::net::SocketAddr;

    use http::Response;
    use tokio::net::TcpListener;
    use tonic::server::NamedService;
    use tonic::transport::server::TcpIncoming;
    use tonic::transport::Server;

    use super::*;
    use crate::google::bigtable::v2::PingAndWarmResponse;

    type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

    /// A server answering `PingAndWarm` requests, recording the client address of each of
    /// them. As every channel has its own connection, the address tells the
    /// channels apart.
    #[derive(Clone, Default)]
    struct FakeBigtable {
        pings: Arc<Mutex<Vec<SocketAddr>>>,
    }

    impl FakeBigtable {
        /// Starts serving, and returns a builder for a connection with 3 channels to it. Its
        /// background tasks are disabled, so that tests only see the requests they expect.
        async fn serve(&self) -> ManagedConnectionBuilder {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = Server::builder().add_service(self.clone());
            tokio::spawn(server.serve_with_incoming(TcpIncoming::from(listener)));
            BigTableConnection::managed("project-1", "instance-1")
                .emulator_endpoint(address.to_string())
                .channel_size(3)
                .max_channel_age(None)
                .ping_and_warm_interval(None)
        }

        fn pings(&self) -> Vec<SocketAddr> {
            self.pings.lock().unwrap().clone()
        }

        fn channels(&self) -> HashSet<SocketAddr> {
            self.pings().into_iter().collect()
        }

        async fn wait_for(&self, condition: impl Fn(&Self) -> bool) {
            tokio::time::timeout(Duration::from_secs(10), async {
                while !condition(self) {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .expect("condition not met in time");
        }
    }

    impl NamedService for FakeBigtable {
        const NAME: &'static str = "google.bigtable.v2.Bigtable";
    }

    impl Service<Request<Body>> for FakeBigtable {
        type Response = Response<Body>;
        type Error = Infallible;
        type Future = BoxFuture<std::result::Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<std::result::Result<(), Infallible>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, request: Request<Body>) -> Self::Future {
            let this = self.clone();
            Box::pin(async move {
                let mut grpc = tonic::server::Grpc::new(tonic_prost::ProstCodec::default());
                Ok(grpc.unary(this, request).await)
            })
        }
    }

    impl tonic::server::UnaryService<PingAndWarmRequest> for FakeBigtable {
        type Response = PingAndWarmResponse;
        type Future =
            BoxFuture<std::result::Result<tonic::Response<Self::Response>, tonic::Status>>;

        fn call(&mut self, request: tonic::Request<PingAndWarmRequest>) -> Self::Future {
            let address = request.remote_addr().unwrap();
            self.pings.lock().unwrap().push(address);
            Box::pin(async { Ok(tonic::Response::new(PingAndWarmResponse {})) })
        }
    }

    #[tokio::test]
    async fn primes_every_channel_before_use() {
        let server = FakeBigtable::default();
        let _connection = server.serve().await.build().await.unwrap();

        assert_eq!(server.pings().len(), 3);
        assert_eq!(server.channels().len(), 3);
    }

    #[tokio::test]
    async fn fails_to_connect_when_priming_fails() {
        // Nothing is listening on the address of a listener that has been dropped.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);

        let result = BigTableConnection::managed("project-1", "instance-1")
            .emulator_endpoint(address.to_string())
            .build()
            .await;

        assert!(matches!(result, Err(Error::TransportError(_))));
    }

    #[tokio::test]
    async fn pings_every_channel_on_each_interval() {
        let server = FakeBigtable::default();
        let builder = server.serve().await;
        let _connection = builder
            .prime_channels(false)
            .ping_and_warm_interval(Some(Duration::from_millis(50)))
            .build()
            .await
            .unwrap();

        server.wait_for(|server| server.pings().len() >= 6).await;

        // No channel is pinged a second time before every channel has been pinged once.
        let pings = server.pings();
        assert_eq!(HashSet::<_>::from_iter(&pings[..3]).len(), 3);
        assert_eq!(pings[..3], pings[3..6]);
    }

    #[tokio::test]
    async fn replaces_channels_older_than_max_age() {
        let server = FakeBigtable::default();
        let builder = server.serve().await;
        let connection = builder
            .max_channel_age(Some(Duration::from_millis(200)))
            .build()
            .await
            .unwrap();
        let initial_channels = server.channels();
        assert_eq!(initial_channels.len(), 3);

        // Replacements are primed too, which is how we know that they exist.
        server.wait_for(|server| server.channels().len() >= 6).await;

        // Requests now go through the replacements, and no longer through the initial channels.
        // The pool is given a moment to pick up the last replacement first.
        tokio::time::sleep(Duration::from_millis(50)).await;
        let pings_before = server.pings().len();
        let mut bigtable = connection.client();
        for _ in 0..10 {
            let request = PingAndWarmRequest::default();
            bigtable.get_client().ping_and_warm(request).await.unwrap();
        }
        let pings = server.pings();
        let ours = &pings[pings_before..];
        assert!(ours.len() >= 10);
        assert!(ours
            .iter()
            .all(|address| !initial_channels.contains(address)));
    }
}
