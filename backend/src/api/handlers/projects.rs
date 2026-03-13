use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use super::ApiResponse;
use crate::middleware::Claims;
use crate::AppState;
use axum::extract::Extension;

/// 项目数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
    pub status: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 创建项目请求
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
}

/// 更新项目请求
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// 附件数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: uuid::Uuid,
    pub project_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: Option<String>,
    pub file_size: Option<i64>,
    pub uploaded_by: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 获取项目列表
pub async fn list_projects(State(state): State<AppState>) -> Json<ApiResponse<Vec<Project>>> {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY updated_at DESC")
        .fetch_all(&*state.pool)
        .await;

    match projects {
        Ok(projects) => Json(ApiResponse::success(projects)),
        Err(e) => {
            tracing::error!("Failed to list projects: {}", e);
            Json(ApiResponse::error("Failed to list projects"))
        }
    }
}

/// 获取单个项目详情
pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Project>>, StatusCode> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(&id)
        .fetch_optional(&*state.pool)
        .await;

    match project {
        Ok(Some(project)) => Ok(Json(ApiResponse::success(project))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 创建新项目
pub async fn create_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateProjectRequest>,
) -> Json<ApiResponse<Project>> {
    // 验证项目类型
    if !["patent", "copyright"].contains(&req.r#type.as_str()) {
        return Json(ApiResponse::error("Invalid project type"));
    }

    // 从 JWT Claims 获取当前用户 ID
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse user ID from JWT: {}", e);
            return Json(ApiResponse::error("Invalid user ID"));
        }
    };

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (user_id, name, description, type, status, metadata)
        VALUES ($1, $2, $3, $4, 'draft', '{}')
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.r#type)
    .fetch_one(&*state.pool)
    .await;

    match project {
        Ok(project) => Json(ApiResponse::success(project)),
        Err(e) => {
            tracing::error!("Failed to create project: {}", e);
            Json(ApiResponse::error("Failed to create project"))
        }
    }
}

/// 更新项目 - 简化版本，分别处理不同字段
pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<ApiResponse<Project>>, StatusCode> {
    // 验证状态值
    if let Some(ref status) = req.status {
        if !["draft", "in_progress", "review", "completed", "archived"].contains(&status.as_str()) {
            return Ok(Json(ApiResponse::error("Invalid status")));
        }
    }

    // 首先检查项目是否存在
    let existing = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(&id)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let existing = existing.unwrap();

    // 使用新值或保留原值
    let name = req.name.unwrap_or(existing.name);
    let description = req.description.or(existing.description);
    let status = req.status.unwrap_or(existing.status);

    // 处理 completed_at - 使用参数化查询而非字符串拼接
    let completed_at_value = if status == "completed" {
        Some(chrono::Utc::now())
    } else {
        None
    };

    let project = sqlx::query_as::<_, Project>(
        r#"
        UPDATE projects
        SET name = $1, description = $2, status = $3, completed_at = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(&name)
    .bind(&description)
    .bind(&status)
    .bind(&completed_at_value)
    .bind(&id)
    .fetch_one(&*state.pool)
    .await;

    match project {
        Ok(project) => Ok(Json(ApiResponse::success(project))),
        Err(e) => {
            tracing::error!("Failed to update project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 删除项目
pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(&id)
        .execute(&*state.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete project: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 上传附件
pub async fn upload_attachment(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Extension(claims): Extension<Claims>,
    mut multipart: axum::extract::Multipart,
) -> Json<ApiResponse<Attachment>> {
    // 获取第一个文件字段
    let Some(field) = multipart.next_field().await.ok().flatten() else {
        return Json(ApiResponse::error("No file provided"));
    };

    let file_name = field.file_name().unwrap_or("unknown").to_string();
    let content_type = field.content_type().map(|ct| ct.to_string());
    let data = field.bytes().await;

    let Ok(data) = data else {
        return Json(ApiResponse::error("Failed to read file data"));
    };

    let file_size = data.len() as i64;

    // 生成唯一的文件 ID 和路径
    let file_id = uuid::Uuid::new_v4().to_string();
    let file_extension = file_name.split('.').last().unwrap_or("bin");
    let file_path = format!("uploads/{}/{}.{}", project_id, file_id, file_extension);

    // 创建上传目录（如果不存在）
    let _ = tokio::fs::create_dir_all("uploads").await;

    // 保存文件
    match tokio::fs::write(&file_path, &data).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to save file: {}", e);
            return Json(ApiResponse::error(format!("Failed to save file: {}", e)));
        }
    }

    // 从 JWT Claims 获取当前用户 ID
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse user ID from JWT: {}", e);
            return Json(ApiResponse::error("Invalid user ID"));
        }
    };

    // 保存到数据库
    let attachment = sqlx::query_as::<_, Attachment>(
        r#"
        INSERT INTO project_attachments (id, project_id, file_name, file_path, file_type, file_size, uploaded_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(&file_id)
    .bind(&project_id)
    .bind(&file_name)
    .bind(&file_path)
    .bind(content_type)
    .bind(file_size)
    .bind(user_id)
    .fetch_one(&*state.pool)
    .await;

    match attachment {
        Ok(attachment) => Json(ApiResponse::success(attachment)),
        Err(e) => {
            tracing::error!("Failed to save attachment metadata: {}", e);
            // 清理已保存的文件
            let _ = tokio::fs::remove_file(&file_path).await;
            Json(ApiResponse::error(format!(
                "Failed to save attachment: {}",
                e
            )))
        }
    }
}

/// 删除附件
pub async fn delete_attachment(
    State(state): State<AppState>,
    Path((project_id, file_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    // 首先获取附件信息以便删除文件
    let attachment = sqlx::query_as::<_, Attachment>(
        "SELECT * FROM project_attachments WHERE project_id = $1 AND id = $2",
    )
    .bind(&project_id)
    .bind(&file_id)
    .fetch_optional(&*state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(attachment) = attachment {
        // 删除文件
        let _ = tokio::fs::remove_file(&attachment.file_path).await;
    }

    let result = sqlx::query("DELETE FROM project_attachments WHERE project_id = $1 AND id = $2")
        .bind(&project_id)
        .bind(&file_id)
        .execute(&*state.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete attachment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
