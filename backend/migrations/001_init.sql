-- 创建扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================
-- 用户与权限系统
-- ============================================

-- 角色表
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    permissions JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role_id UUID REFERENCES roles(id),
    full_name VARCHAR(100),
    avatar_url TEXT,
    is_active BOOLEAN DEFAULT true,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- API Keys 表
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    key_hash VARCHAR(255) NOT NULL,
    permissions JSONB DEFAULT '[]'::jsonb,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 项目管理
-- ============================================

-- 项目表
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    type VARCHAR(50) NOT NULL CHECK (type IN ('patent', 'copyright')),
    status VARCHAR(50) DEFAULT 'draft' CHECK (status IN ('draft', 'in_progress', 'review', 'completed', 'archived')),
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- 项目附件表
CREATE TABLE IF NOT EXISTS project_attachments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_type VARCHAR(50),
    file_size BIGINT,
    uploaded_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 专利说明书相关
-- ============================================

-- 专利类型枚举
DO $$ BEGIN
    CREATE TYPE patent_type AS ENUM ('invention', 'utility', 'design');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 文档状态枚举
DO $$ BEGIN
    CREATE TYPE document_status AS ENUM ('draft', 'reviewing', 'approved', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 专利说明书表
CREATE TABLE IF NOT EXISTS patent_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    patent_type patent_type NOT NULL,
    title VARCHAR(500) NOT NULL,
    technical_field TEXT,
    background_art TEXT,
    invention_content TEXT,
    claims JSONB,
    abstract_text TEXT,
    drawings_description TEXT,
    embodiment TEXT,
    ai_prompt TEXT,
    ai_model VARCHAR(100),
    version INTEGER DEFAULT 1,
    status document_status DEFAULT 'draft',
    review_comments TEXT,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 专利版本历史表
CREATE TABLE IF NOT EXISTS patent_document_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    patent_document_id UUID REFERENCES patent_documents(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    content_snapshot JSONB NOT NULL,
    change_summary TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 软件著作权说明书相关
-- ============================================

-- 软著说明书表
CREATE TABLE IF NOT EXISTS copyright_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    software_name VARCHAR(255) NOT NULL,
    software_version VARCHAR(50),
    developer VARCHAR(255),
    completion_date DATE,
    publication_date DATE,
    software_category VARCHAR(100),
    operating_system VARCHAR(255),
    programming_language VARCHAR(100),
    line_count INTEGER,
    source_code_path VARCHAR(500),
    user_manual_path VARCHAR(500),
    description TEXT,
    function_features TEXT,
    technical_features TEXT,
    ai_prompt TEXT,
    ai_model VARCHAR(100),
    version INTEGER DEFAULT 1,
    status document_status DEFAULT 'draft',
    review_comments TEXT,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 软著版本历史表
CREATE TABLE IF NOT EXISTS copyright_document_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    copyright_document_id UUID REFERENCES copyright_documents(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    content_snapshot JSONB NOT NULL,
    change_summary TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- AI 配置与模板
-- ============================================

-- AI 模型配置表
CREATE TABLE IF NOT EXISTS ai_model_configs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    provider VARCHAR(50) NOT NULL CHECK (provider IN ('anthropic', 'openai', 'azure', 'local')),
    model_name VARCHAR(100) NOT NULL,
    api_key_encrypted TEXT,
    api_endpoint VARCHAR(500),
    max_tokens INTEGER DEFAULT 4096,
    temperature DECIMAL(3,2) DEFAULT 0.7,
    is_active BOOLEAN DEFAULT true,
    priority INTEGER DEFAULT 0,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 文档模板表
CREATE TABLE IF NOT EXISTS document_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL CHECK (type IN ('patent_invention', 'patent_utility', 'patent_design', 'copyright')),
    content_template TEXT NOT NULL,
    variables JSONB DEFAULT '[]'::jsonb,
    description TEXT,
    is_system BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 审计与日志
-- ============================================

-- 审计日志表
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    resource_id UUID,
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- AI 使用日志表
CREATE TABLE IF NOT EXISTS ai_usage_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    model_config_id UUID REFERENCES ai_model_configs(id),
    prompt_tokens INTEGER,
    completion_tokens INTEGER,
    total_tokens INTEGER,
    cost DECIMAL(10,6),
    request_payload JSONB,
    response_summary TEXT,
    status VARCHAR(50) DEFAULT 'success',
    error_message TEXT,
    duration_ms INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 索引
-- ============================================

CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_projects_user_id ON projects(user_id);
CREATE INDEX IF NOT EXISTS idx_projects_type ON projects(type);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_patent_docs_project_id ON patent_documents(project_id);
CREATE INDEX IF NOT EXISTS idx_patent_docs_status ON patent_documents(status);
CREATE INDEX IF NOT EXISTS idx_copyright_docs_project_id ON copyright_documents(project_id);
CREATE INDEX IF NOT EXISTS idx_copyright_docs_status ON copyright_documents(status);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_ai_usage_user_id ON ai_usage_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_usage_created_at ON ai_usage_logs(created_at);

-- ============================================
-- 触发器：自动更新 updated_at
-- ============================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;
CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_patent_docs_updated_at ON patent_documents;
CREATE TRIGGER update_patent_docs_updated_at BEFORE UPDATE ON patent_documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_copyright_docs_updated_at ON copyright_documents;
CREATE TRIGGER update_copyright_docs_updated_at BEFORE UPDATE ON copyright_documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_ai_model_configs_updated_at ON ai_model_configs;
CREATE TRIGGER update_ai_model_configs_updated_at BEFORE UPDATE ON ai_model_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_document_templates_updated_at ON document_templates;
CREATE TRIGGER update_document_templates_updated_at BEFORE UPDATE ON document_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 初始化数据
-- ============================================

-- 插入默认角色
INSERT INTO roles (id, name, description, permissions) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', '系统管理员', '["all"]'::jsonb),
    ('00000000-0000-0000-0000-000000000002', 'user', '普通用户', '["read", "write"]'::jsonb),
    ('00000000-0000-0000-0000-000000000003', 'reviewer', '审核员', '["read", "review"]'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- 插入默认管理员账户 (密码：admin123)
-- 使用 argon2 哈希的密码
INSERT INTO users (id, username, email, password_hash, role_id, full_name) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', 'admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZXhhbXBsZXNhbHQ$K3u3W8T6zZqQHmCqV8K9J5vLqxBJxQKzJ5vLqxBJxQK',
     '00000000-0000-0000-0000-000000000001', '系统管理员')
ON CONFLICT (id) DO NOTHING;

-- 插入默认 AI 模型配置
INSERT INTO ai_model_configs (provider, model_name, max_tokens, is_active, priority) VALUES
    ('anthropic', 'claude-3-5-sonnet-20241022', 4096, true, 1),
    ('anthropic', 'claude-3-opus-20240229', 4096, false, 2)
ON CONFLICT DO NOTHING;

-- 插入默认模板
INSERT INTO document_templates (name, type, content_template, is_system, is_active) VALUES
    ('发明专利标准模板', 'patent_invention', '# 发明专利说明书\n\n## 技术领域\n{technical_field}\n\n## 背景技术\n{background_art}\n\n## 发明内容\n{invention_content}\n\n## 具体实施方式\n{embodiment}\n\n## 权利要求书\n{claims}\n\n## 摘要\n{abstract}', true, true),
    ('实用新型标准模板', 'patent_utility', '# 实用新型专利说明书\n\n## 技术领域\n{technical_field}\n\n## 背景技术\n{background_art}\n\n## 实用新型内容\n{invention_content}\n\n## 附图说明\n{drawings}\n\n## 具体实施方式\n{embodiment}\n\n## 权利要求书\n{claims}', true, true),
    ('软著说明书模板', 'copyright', '# 软件著作权说明书\n\n## 软件基本信息\n- 软件名称：{software_name}\n- 版本号：{software_version}\n- 开发者：{developer}\n\n## 软件功能说明\n{function_description}\n\n## 软件技术特点\n{technical_features}\n\n## 运行环境\n{operating_system}\n\n## 源代码说明\n{source_code_description}', true, true)
ON CONFLICT DO NOTHING;
