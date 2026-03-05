use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let mut settings = config::Config::builder()
            .set_default("database_url", "postgres://localhost:5432/patent_db")?
            .set_default("jwt_secret", "dev-secret")?
            .set_default("server_port", 3000)?
            .add_source(config::Environment::default().separator("_"));

        settings.build()?.try_deserialize()
    }
}
