//! What connections send their requests through.

use std::task::{Context, Poll};

use futures_util::future::{Either, MapErr};
use futures_util::TryFutureExt;
use http::{Request, Response};
use tonic::body::Body;
use tonic::transport::Channel;
use tower::{BoxError, Service};

use crate::bigtable::managed::ManagedTransport;

/// The transport of a connection.
///
/// This is an enum, so that all connections share the same client type whatever their
/// transport, without boxing the transport and the future of every request.
#[derive(Clone)]
pub(crate) enum Transport {
    /// A `tonic` channel, which load balances requests onto its connections by itself.
    Channel(Channel),
    /// A pool of channels that are managed in the background.
    Managed(ManagedTransport),
}

impl From<Channel> for Transport {
    fn from(channel: Channel) -> Self {
        Self::Channel(channel)
    }
}

impl From<ManagedTransport> for Transport {
    fn from(managed: ManagedTransport) -> Self {
        Self::Managed(managed)
    }
}

impl Service<Request<Body>> for Transport {
    type Response = Response<Body>;
    type Error = BoxError;
    type Future = Either<
        MapErr<
            <Channel as Service<Request<Body>>>::Future,
            fn(tonic::transport::Error) -> BoxError,
        >,
        <ManagedTransport as Service<Request<Body>>>::Future,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self {
            Self::Channel(channel) => channel.poll_ready(cx).map_err(Into::into),
            Self::Managed(managed) => managed.poll_ready(cx),
        }
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        match self {
            Self::Channel(channel) => {
                Either::Left(channel.call(request).map_err(BoxError::from as fn(_) -> _))
            }
            Self::Managed(managed) => Either::Right(managed.call(request)),
        }
    }
}
