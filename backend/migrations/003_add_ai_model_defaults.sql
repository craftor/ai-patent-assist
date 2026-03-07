-- 添加 is_default 字段到 ai_model_configs 表
ALTER TABLE ai_model_configs ADD COLUMN IF NOT EXISTS is_default BOOLEAN DEFAULT false;

-- 确保只有一个默认模型
CREATE UNIQUE INDEX IF NOT EXISTS idx_ai_model_configs_is_default
ON ai_model_configs (is_default) WHERE is_default = true;
