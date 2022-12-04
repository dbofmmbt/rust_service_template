use self::http_client::HttpClient;

mod http_client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: HttpClient,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            http_client: http_client::default(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
