use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let host = self.host.clone();
        let port = self.port;
        let result = format!("{host}:{port}");
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::Server;

    #[test]
    fn can_convert_to_string_with_port() {
        let server = Server {
            host: "http://localhost".to_string(),
            port: 3000,
        };
        let string: String = server.to_string();
        assert_eq!("http://localhost:3000", string);
    }
}
