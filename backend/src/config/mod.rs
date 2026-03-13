use serde::{Deserialize, Serialize};

/// 应用配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u32,
    pub server_port: u16,
    pub anthropic_api_key: String,
}

/// 配置验证错误
#[derive(Debug, Clone)]
pub struct ConfigError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "配置错误 [{}]: {}", self.field, self.message)
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// 加载配置
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
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

        let config = Config {
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            server_port,
            anthropic_api_key,
        };

        // 验证配置
        config.validate()?;

        Ok(config)
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 验证数据库 URL
        if self.database_url.is_empty() {
            return Err(ConfigError {
                field: "database_url".to_string(),
                message: "数据库 URL 不能为空".to_string(),
            });
        }

        // 验证数据库 URL 格式
        if !self.database_url.starts_with("postgres://")
            && !self.database_url.starts_with("postgresql://")
        {
            return Err(ConfigError {
                field: "database_url".to_string(),
                message: "数据库 URL 必须以 postgres:// 或 postgresql:// 开头".to_string(),
            });
        }

        // 验证 JWT 密钥
        if self.jwt_secret.is_empty() {
            return Err(ConfigError {
                field: "jwt_secret".to_string(),
                message: "JWT 密钥不能为空".to_string(),
            });
        }

        // 在生产环境中，JWT 密钥应该足够长
        if self.jwt_secret.len() < 32 && !is_dev_environment() {
            return Err(ConfigError {
                field: "jwt_secret".to_string(),
                message: "生产环境中 JWT 密钥长度至少为 32 字符".to_string(),
            });
        }

        // 验证 JWT 过期时间
        if self.jwt_expiry_hours == 0 {
            return Err(ConfigError {
                field: "jwt_expiry_hours".to_string(),
                message: "JWT 过期时间必须大于 0".to_string(),
            });
        }

        // 验证 Anthropic API Key
        if self.anthropic_api_key.is_empty() {
            return Err(ConfigError {
                field: "anthropic_api_key".to_string(),
                message: "Anthropic API Key 不能为空".to_string(),
            });
        }

        // 验证 API Key 格式（应该以 sk- 开头）
        if !self.anthropic_api_key.starts_with("sk-") && !is_dev_environment() {
            return Err(ConfigError {
                field: "anthropic_api_key".to_string(),
                message: "生产环境中 Anthropic API Key 应该以 sk- 开头".to_string(),
            });
        }

        Ok(())
    }

    /// 检查是否是开发环境
    pub fn is_dev_environment(&self) -> bool {
        std::env::var("RUST_ENV")
            .or_else(|_| std::env::var("APP_ENV"))
            .unwrap_or_else(|_| "development".to_string())
            == "development"
    }
}

/// 检查是否是开发环境（独立函数）
fn is_dev_environment() -> bool {
    std::env::var("RUST_ENV")
        .or_else(|_| std::env::var("APP_ENV"))
        .unwrap_or_else(|_| "development".to_string())
        == "development"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_empty_database_url() {
        let config = Config {
            database_url: "".to_string(),
            jwt_secret: "test-secret".to_string(),
            jwt_expiry_hours: 24,
            server_port: 3000,
            anthropic_api_key: "sk-test".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_jwt_secret() {
        let config = Config {
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "".to_string(),
            jwt_expiry_hours: 24,
            server_port: 3000,
            anthropic_api_key: "sk-test".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_database_url() {
        let config = Config {
            database_url: "mysql://localhost/test".to_string(),
            jwt_secret: "test-secret".to_string(),
            jwt_expiry_hours: 24,
            server_port: 3000,
            anthropic_api_key: "sk-test".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_zero_expiry_hours() {
        let config = Config {
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "test-secret".to_string(),
            jwt_expiry_hours: 0,
            server_port: 3000,
            anthropic_api_key: "sk-test".to_string(),
        };

        assert!(config.validate().is_err());
    }
}
