// 认证模块 - 使用 argon2 进行密码哈希

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// 使用 argon2 哈希密码
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
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
