// API 路由模块
// 重新导出所有 handlers 供 main.rs 使用

pub use crate::api::handlers::health_check;

// Auth handlers
pub use crate::api::handlers::auth::{
    login,
    logout,
    register,
    refresh_token,
    get_current_user,
};

// User handlers
pub use crate::api::handlers::users::{
    list_users,
    get_user,
    update_user,
    delete_user,
};

// Project handlers
pub use crate::api::handlers::projects::{
    list_projects,
    create_project,
    get_project,
    update_project,
    delete_project,
    upload_attachment,
    delete_attachment,
};

// Patent handlers
pub use crate::api::handlers::patents::{
    generate as generate_patent,
    get_patent,
    update_patent,
    list_versions as list_patent_versions,
    submit_review as submit_patent_review,
    export_document as export_patent,
};

// Copyright handlers
pub use crate::api::handlers::copyrights::{
    generate as generate_copyright,
    get_copyright,
    update_copyright,
    list_versions as list_copyright_versions,
    submit_review as submit_copyright_review,
    export_document as export_copyright,
};

// Template handlers
pub use crate::api::handlers::templates::{
    list_templates,
    create_template,
    update_template,
    delete_template,
};

// API Key handlers
pub use crate::api::handlers::api_keys::{
    list_keys,
    create_key,
    revoke_key,
};

// AI handlers
pub use crate::api::handlers::ai::{
    list_models,
    add_model,
    update_model,
    delete_model,
    get_default_model,
    set_default_model,
};

// Log handlers
pub use crate::api::handlers::logs::{
    list_audit_logs,
    list_ai_usage,
};
