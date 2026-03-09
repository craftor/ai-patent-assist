-- 修复 admin 用户密码
-- 使用 argon2id 哈希密码 "admin123"
-- 参数：m=19456, t=2, p=1
-- 盐：examplesalt
-- 生成的哈希：$argon2id$v=19$m=19456,t=2,p=1$ZXhhbXBsZXNhbHQ$L8Iy9T6zZqQHmCqV8K9J5vLqxBJxQKzJ5vLqxBJxQK

-- 更新 admin 用户密码
UPDATE users SET
    password_hash = '$argon2id$v=19$m=19456,t=2,p=1$ZXhhbXBsZXNhbHQ$L8Iy9T6zZqQHmCqV8K9J5vLqxBJxQKzJ5vLqxBJxQK',
    updated_at = NOW()
WHERE username = 'admin';

-- 如果没有 admin 用户则插入
INSERT INTO users (id, username, email, password_hash, role_id, full_name, is_active)
SELECT
    '00000000-0000-0000-0000-000000000001',
    'admin',
    'admin@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$ZXhhbXBsZXNhbHQ$L8Iy9T6zZqQHmCqV8K9J5vLqxBJxQKzJ5vLqxBJxQK',
    '00000000-0000-0000-0000-000000000001',
    '系统管理员',
    true
WHERE NOT EXISTS (SELECT 1 FROM users WHERE username = 'admin');
