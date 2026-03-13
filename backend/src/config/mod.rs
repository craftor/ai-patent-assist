use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u32,
    pub server_port: u16,
    pub anthropic_api_key: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        // 直接使用环境变量，避免 config crate 对 URL 的错误解析
        let database_url = std::env::var("DATABASE_URL")
            .or_else(|_| std::env::var("APP__DATABASE_URL"))
            .unwrap_or_else(|_| {
                "postgres://patent_user:patent_password@localhost:5432/patent_db".to_string()
            });

        let jwt_secret = std::env::var("JWT_SECRET")
            .or_else(|_| std::env::var("APP__JWT_SECRET"))
            .unwrap_or_else(|_| "dev-secret-change-in-production".to_string());

        let jwt_expiry_hours = std::env::var("JWT_EXPIRY_HOURS")
            .or_else(|_| std::env::var("APP__JWT_EXPIRY_HOURS"))
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(24);

        let server_port = std::env::var("SERVER_PORT")
            .or_else(|_| std::env::var("APP__SERVER_PORT"))
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);

        let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| std::env::var("APP__ANTHROPIC_API_KEY"))
            .unwrap_or_else(|_| "test-key".to_string());

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            server_port,
            anthropic_api_key,
        })
    }
}
