use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { port: None }
    }
}
