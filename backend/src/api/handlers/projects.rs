use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use super::ApiResponse;
use crate::AppState;

/// 项目数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
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
    pub project_type: String,
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
    pub id: String,
    pub project_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: Option<String>,
    pub file_size: Option<i64>,
    pub uploaded_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 获取项目列表
pub async fn list_projects(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<Project>>> {
    let Some(pool) = state.pool.as_ref() else {
        return Json(ApiResponse::error("Database not connected"));
    };

    let projects = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects ORDER BY updated_at DESC"
    )
    .fetch_all(pool.as_ref())
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
    let Some(pool) = state.pool.as_ref() else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };

    let project = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(pool.as_ref())
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
    Json(req): Json<CreateProjectRequest>,
) -> Json<ApiResponse<Project>> {
    let Some(pool) = state.pool.as_ref() else {
        return Json(ApiResponse::error("Database not connected"));
    };

    // 验证项目类型
    if !["patent", "copyright"].contains(&req.project_type.as_str()) {
        return Json(ApiResponse::error("Invalid project type"));
    }

    // 使用默认用户 ID（临时方案，后续应添加认证中间件获取当前用户）
    let default_user_id = "00000000-0000-0000-0000-000000000001";

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (user_id, name, description, type, status, metadata)
        VALUES ($1, $2, $3, $4, 'draft', '{}')
        RETURNING *
        "#
    )
    .bind(default_user_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.project_type)
    .fetch_one(pool.as_ref())
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
    let Some(pool) = state.pool.as_ref() else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };

    // 验证状态值
    if let Some(ref status) = req.status {
        if !["draft", "in_progress", "review", "completed", "archived"].contains(&status.as_str()) {
            return Ok(Json(ApiResponse::error("Invalid status")));
        }
    }

    // 首先检查项目是否存在
    let existing = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(&id)
        .fetch_optional(pool.as_ref())
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
    let completed_at = if status == "completed" {
        "NOW()"
    } else {
        "NULL"
    };

    let project = sqlx::query_as::<_, Project>(
        &format!(
            r#"
            UPDATE projects
            SET name = $1, description = $2, status = $3, completed_at = {}, updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
            completed_at
        )
    )
    .bind(&name)
    .bind(&description)
    .bind(&status)
    .bind(&id)
    .fetch_one(pool.as_ref())
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
    let Some(pool) = state.pool.as_ref() else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };

    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(&id)
        .execute(pool.as_ref())
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
    State(_state): State<AppState>,
    Path(_project_id): Path<String>,
) -> Json<ApiResponse<Attachment>> {
    // TODO: 实现文件上传逻辑
    Json(ApiResponse::error("File upload not implemented yet"))
}

/// 删除附件
pub async fn delete_attachment(
    State(state): State<AppState>,
    Path((project_id, file_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    let Some(pool) = state.pool.as_ref() else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };

    let result = sqlx::query(
        "DELETE FROM project_attachments WHERE project_id = $1 AND id = $2"
    )
    .bind(&project_id)
    .bind(&file_id)
    .execute(pool.as_ref())
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
