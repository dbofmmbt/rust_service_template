use std::net::TcpListener;

use reqwest::RequestBuilder;

use {{crate_name}}::server;

pub struct TestServer {
    pub port: u16,
    client: reqwest::Client,
}

impl TestServer {
    pub async fn start() -> eyre::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;
        let port = listener.local_addr()?.port();
        let fut = async { server::start(listener).await };
        tokio::task::spawn(fut);
        Ok(Self {
            port,
            client: reqwest::Client::new(),
        })
    }

    pub fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub fn url(&self, path: &str) -> String {
        self.base_url() + path
    }
}

macro_rules! impl_verbs {
    ($($verb:tt),+) => {$(
        pub fn $verb(&self, path: &str) -> RequestBuilder {
            self.client.$verb(self.url(path))
        }
    )*};
}

#[allow(unused)]
impl TestServer {
    impl_verbs!(get, post, put, patch, delete);
}
