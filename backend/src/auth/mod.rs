// 认证模块 - 简化版本
// 完整的 JWT 认证将在后续实现

pub fn hash_password(password: &str) -> Result<String, String> {
    // 简化版本：实际应该使用 argon2
    Ok(format!("hashed_{}", password))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    // 简化版本
    hash == &format!("hashed_{}", password)
}
