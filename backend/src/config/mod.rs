use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u32,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let mut settings = config::Config::builder()
            .set_default("database_url", "postgres://patent_user:patent_password@localhost:5432/patent_db")?
            .set_default("jwt_secret", "dev-secret-change-in-production")?
            .set_default("jwt_expiry_hours", 24)?
            .set_default("server_port", 3000)?
            .add_source(config::Environment::default()
                .separator("__")
                .prefix("APP")
            );

        settings.build()?.try_deserialize()
    }
}
