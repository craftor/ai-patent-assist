// JWT 认证中间件

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

use crate::AppState;

/// JWT Claims 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // 用户 ID
    pub username: String,
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
}

/// 提取 JWT 密钥
fn get_jwt_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-jwt-secret-key-for-development-only".to_string())
        .into_bytes()
}

/// JWT 认证中间件
pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从请求头获取 Authorization
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    // 如果没有 Authorization header，检查是否是公开端点
    let auth_header = match auth_header {
        Some(header) => header,
        None => {
            // 允许没有 Token 的请求访问公开端点
            let path = request.uri().path();
            if is_public_endpoint(path) {
                return Ok(next.run(request).await);
            }
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 解析 Bearer Token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 如果是测试 Token，允许通过（开发环境）
    if token == "test-admin-token-12345" {
        request.extensions_mut().insert(Claims {
            sub: "00000000-0000-0000-0000-000000000001".to_string(),
            username: "admin".to_string(),
            exp: 0,
            iat: 0,
        });
        return Ok(next.run(request).await);
    }

    // 验证 JWT Token
    let secret = get_jwt_secret();
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false; // 开发环境不验证过期时间

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret),
        &validation,
    ) {
        Ok(token_data) => {
            // Token 有效，将 Claims 放入 request extensions
            request.extensions_mut().insert(token_data.claims);
            Ok(next.run(request).await)
        }
        Err(_) => {
            // Token 无效
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// 检查是否是公开端点（不需要认证）
fn is_public_endpoint(path: &str) -> bool {
    let public_paths = [
        "/api/auth/login",
        "/api/auth/register",
        "/api/auth/test-account",
        "/health",
        "/api/health",
    ];
    public_paths.iter().any(|&p| path.starts_with(p))
}

/// 生成 JWT Token
pub fn generate_token(user_id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};

    let secret = get_jwt_secret();
    let now = Utc::now();

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: (now + chrono::Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret),
    )
}
