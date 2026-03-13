mod ai;
mod auth;
mod copyrights;
mod patents;
mod projects;
mod templates;
mod users;

pub use ai::{
    add_model, delete_model, get_default_model, list_models, set_default_model, update_model,
};
pub use auth::{get_current_user, get_test_account, login, logout, register};
pub use copyrights::{generate_copyright, get_copyright, list_copyrights, update_copyright};
pub use patents::{generate_patent, get_patent, list_patents, update_patent};
pub use projects::{
    create_project, delete_attachment, delete_project, get_project, list_projects, update_project,
    upload_attachment,
};
pub use templates::{
    create_template, delete_template, get_template, list_templates, update_template,
};
pub use users::{change_password, delete_user, get_user, list_users, update_user};

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
