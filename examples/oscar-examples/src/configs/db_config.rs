use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DBConfig {
    user: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    db_name: Option<String>,
    schema: Option<String>,
}

impl DBConfig {
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }

    pub fn db_name(&self) -> &str {
        self.db_name.as_deref().unwrap_or("postgres")
    }

    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("postgres")
    }
}
