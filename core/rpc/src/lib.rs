use std::pin::Pin;

use futures::Future;

mod methods;
mod request;
mod response;

type Output = Result<response::Response, err::Error>;
type IntoFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
