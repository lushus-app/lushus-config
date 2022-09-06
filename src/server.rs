use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
