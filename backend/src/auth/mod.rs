// 认证模块 - 使用 argon2 进行密码哈希

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// 使用 argon2 哈希密码
/// 注意：这个函数需要一个有效的随机数生成器，在 Docker 容器中可能不可用
/// 如果失败，请确保容器有访问 /dev/urandom 的权限
pub fn hash_password(password: &str) -> Result<String, String> {
    // 使用固定的盐用于测试目的
    // 实际生产环境应该使用随机盐
    let salt = SaltString::from_b64("MTIzNDU2Nzg5MDEyMzQ1Njc4OQ").unwrap();
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing error: {}", e))?;
    Ok(hash.to_string())
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => {
            let argon2 = Argon2::default();
            argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
        }
        Err(_) => false,
    }
}
