use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;

pub type HttpClient = ClientWithMiddleware;

pub fn default() -> HttpClient {
    ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .build()
}
