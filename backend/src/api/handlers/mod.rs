mod auth;
mod projects;
mod patents;
mod copyrights;
mod templates;
mod api_keys;
mod ai;
mod logs;
mod users;

pub use auth::{login, register, get_test_account, logout, get_current_user};
pub use projects::{
    list_projects,
    get_project,
    create_project,
    update_project,
    delete_project,
    upload_attachment,
    delete_attachment,
};
pub use patents::{list_patents, get_patent, generate_patent, update_patent};
pub use copyrights::{list_copyrights, get_copyright, generate_copyright, update_copyright};
pub use templates::{list_templates, create_template, update_template, delete_template};
pub use api_keys::{list_keys, create_key, revoke_key};
pub use ai::{list_models, add_model, update_model, delete_model, get_default_model, set_default_model};
pub use logs::{list_audit_logs, list_ai_usage};
pub use users::{list_users, get_user, update_user, delete_user};

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

/// 通用 API 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
        }
    }
}

/// 健康检查
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

