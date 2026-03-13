use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// AI 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModelConfig {
    pub id: uuid::Uuid,
    pub provider: String,
    pub model_name: String,
    pub api_key_encrypted: Option<String>,
    pub api_endpoint: Option<String>,
    pub max_tokens: i32,
    pub temperature: f64,
    pub is_active: bool,
    pub priority: i32,
    pub metadata: Option<serde_json::Value>,
}
