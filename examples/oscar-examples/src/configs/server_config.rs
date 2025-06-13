use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    ip: String,
    port: Option<u16>,
}

impl ServerConfig {
    pub fn get_server_ip(&self) -> &str {
        &self.ip
    }
    pub fn get_server_port(&self) -> u16 {
        self.port.unwrap_or(3003)
    }
}
