// API 路由模块
// 重新导出所有 handlers 供 main.rs 使用
// 注意：此文件仅供兼容性保留，实际路由定义在 main.rs 的 build_app 函数中

// Auth handlers
pub use crate::api::handlers::auth::{
    login,
    logout,
    register,
    get_test_account,
    get_current_user,
};

// User handlers
pub use crate::api::handlers::users::{
    list_users,
    get_user,
    update_user,
    delete_user,
    change_password,
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
    list_patents,
    get_patent,
    generate_patent,
    update_patent,
};

// Copyright handlers
pub use crate::api::handlers::copyrights::{
    list_copyrights,
    get_copyright,
    generate_copyright,
    update_copyright,
};

// Template handlers
pub use crate::api::handlers::templates::{
    list_templates,
    create_template,
    update_template,
    delete_template,
    get_template,
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

// 以下为未实现的功能（保留供未来扩展）
// API Key handlers (TODO)
// Log handlers (TODO)

