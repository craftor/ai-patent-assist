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

/// AI 使用日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiUsageLog {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub model_config_id: Option<uuid::Uuid>,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub cost: Option<f64>,
    pub status: String,
    pub error_message: Option<String>,
    pub duration_ms: i32,
}
