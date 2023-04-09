use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Deserialize)]
pub struct Server {
    protocol: String,
    host: String,
    port: Option<u16>,
}

impl Server {
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let protocol = self.protocol();
        let host = self.host.clone();
        let result = if let Some(port) = &self.port {
            format!("{protocol}://{host}:{port}")
        } else {
            format!("{protocol}://{host}")
        };
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::Server;

    #[test]
    fn can_convert_to_string_with_port() {
        let server = Server {
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(3000),
        };
        let string: String = server.to_string();
        assert_eq!("http://localhost:3000", string);
    }

    #[test]
    fn can_convert_to_string_without_port() {
        let server = Server {
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: None,
        };
        let string: String = server.to_string();
        assert_eq!("http://localhost", string);
    }
}
