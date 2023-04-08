use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Server {
    host: String,
    port: Option<u16>,
}

impl Server {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }
}
