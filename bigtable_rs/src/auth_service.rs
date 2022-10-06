use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use gcp_auth::AuthenticationManager;
use http::{HeaderValue, Request, Response};
use log::debug;
use tonic::body::BoxBody;
use tonic::transport::Body;
use tonic::transport::Channel;
use tower::Service;

#[derive(Clone)]
pub struct AuthSvc {
    inner: Channel,
    authentication_manager: Option<Arc<AuthenticationManager>>,
    scopes: String,
}

impl AuthSvc {
    pub fn new(
        inner: Channel,
        authentication_manager: Option<Arc<AuthenticationManager>>,
        scopes: String,
    ) -> Self {
        AuthSvc {
            inner,
            authentication_manager,
            scopes,
        }
    }
}

impl Service<Request<BoxBody>> for AuthSvc {
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut request: Request<BoxBody>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let authentication_manager = self.authentication_manager.clone();
        let scopes = self.scopes.clone();

        Box::pin(async move {
            let scopes = &[scopes.as_ref()];
            let token_f_opt = authentication_manager.as_ref().map(|m| m.get_token(scopes));

            return match token_f_opt {
                None => {
                    debug!("auth intercepting and not attaching token");
                    let response = inner.call(request).await?;
                    Ok(response)
                }
                Some(token_future) => {
                    let token = token_future.await?;
                    let token = token.as_str().parse::<String>()?;
                    let bearer_header =
                        HeaderValue::from_str(format!("Bearer {}", token.as_str()).as_str())
                            .unwrap();
                    debug!(
                        "auth intercepting with scope {:?} and attaching token's head {}",
                        scopes,
                        std::str::from_utf8(&token.as_bytes()[..5]).unwrap_or("")
                    );
                    request.headers_mut().insert("authorization", bearer_header);
                    let response = inner.call(request).await?;
                    Ok(response)
                }
            };
        })
    }
}
