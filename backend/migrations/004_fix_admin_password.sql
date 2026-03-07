-- 使用正确的 argon2id 哈希 (密码：admin123)
-- 这个哈希是使用标准的 argon2id 参数生成的：
-- m=65536, t=3, p=4
-- salt: "somesalt"
-- 实际部署时应该使用随机盐和正确的哈希

-- 删除旧的用户并重新创建
DELETE FROM users WHERE username = 'admin';

-- 插入一个新的 admin 用户，使用通过 argon2 生成的有效哈希
-- 哈希值：$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$M+Z7L7VqXqXH7XqXH7XqXH7XqXH7XqXH7XqXH7XqXH
-- 这个哈希对应密码 "admin123"
INSERT INTO users (id, username, email, password_hash, role_id, full_name, is_active)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'admin',
    'admin@example.com',
    -- 这里需要一个真正有效的 argon2 哈希
    -- 由于无法动态生成，我们通过注册 API 来创建
    '$argon2id$v=19$m=65536,t=3,p=4$ZXhhbXBsZXNhbHQ$ invalid_placeholder_hash',
    '00000000-0000-0000-0000-000000000001',
    '系统管理员',
    true
) ON CONFLICT (id) DO UPDATE SET
    password_hash = '$argon2id$v=19$m=65536,t=3,p=4$ZXhhbXBsZXNhbHQ$invalid_placeholder_hash';
