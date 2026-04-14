use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub device_name: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap_or(8080);
        let device_name = env::var("DEVICE_NAME").unwrap_or("CC-G1-Gateway".into());
        Self { port, device_name }
    }
}