use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    timeout: Option<u64>,
}

impl DatabaseConfig {
    pub fn host(&self) -> String {
        self.host.clone().unwrap_or("localhost".to_string())
    }
    pub fn port(&self) -> u16 {
        self.port.clone().unwrap_or(5432)
    }
    pub fn username(&self) -> String {
        self.username.clone().unwrap_or("postgres".to_string())
    }
    pub fn password(&self) -> String {
        self.password.clone().unwrap_or("".to_string())
    }
    pub fn database(&self) -> String {
        self.database.clone().unwrap_or("postgres".to_string())
    }
    pub fn schema(&self) -> String {
        self.schema.clone().unwrap_or("public".to_string())
    }
    pub fn timeout(&self) -> u64 {
        self.timeout.clone().unwrap_or(5)
    }
}
