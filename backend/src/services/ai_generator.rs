use crate::models::{AiModelConfig, AiUsageLog};
use crate::config::Config;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct AiGenerator {
    client: Client,
    config: Config,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: i32,
    messages: Vec<Message>,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    input_tokens: i32,
    output_tokens: i32,
}

impl AiGenerator {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// 生成专利说明书
    pub async fn generate_patent(
        &self,
        model_config: &AiModelConfig,
        title: &str,
        technical_field: &str,
        background_art: &str,
        invention_description: &str,
        embodiments: &[String],
        claims_input: Option<&str>,
    ) -> Result<PatentGenerationResult, Box<dyn std::error::Error>> {
        let start = Instant::now();

        let system_prompt = r#"你是一位专业的专利代理人，具有多年的专利撰写经验。
你的任务是根据用户提供的技术信息，编写符合中国国家知识产权局标准的专利说明书。

专利说明书应包含以下部分：
1. 技术领域
2. 背景技术
3. 发明内容
4. 附图说明（如有）
5. 具体实施方式
6. 权利要求书
7. 摘要

请使用专业、准确的语言，确保描述清晰、完整、无歧义。"#;

        let mut user_prompt = format!(
            r#"请根据以下信息编写专利说明书：

【发明名称】
{}

【技术领域】
{}

【背景技术】
{}

【发明内容】
{}

【具体实施方式】
{}"#,
            title,
            technical_field,
            background_art,
            invention_description,
            embodiments.join("\n\n")
        );

        if let Some(claims) = claims_input {
            user_prompt.push_str(&format!(
                "\n\n【权利要求书初步构思】\n{}",
                claims
            ));
        }

        let response = self.call_anthropic_api(
            &model_config.model_name,
            system_prompt,
            &user_prompt,
            model_config.max_tokens,
        ).await?;

        let duration = start.elapsed().as_millis() as i32;

        Ok(PatentGenerationResult {
            content: response.content.first().map(|c| c.text.clone()).unwrap_or_default(),
            input_tokens: response.usage.input_tokens,
            output_tokens: response.usage.output_tokens,
            duration_ms: duration,
        })
    }

    /// 生成软著说明书
    pub async fn generate_copyright(
        &self,
        model_config: &AiModelConfig,
        software_name: &str,
        software_version: Option<&str>,
        developer: &str,
        description: &str,
        function_features: &str,
        technical_features: &str,
        source_code_summary: Option<&str>,
    ) -> Result<CopyrightGenerationResult, Box<dyn std::error::Error>> {
        let start = Instant::now();

        let system_prompt = r#"你是一位专业的软件著作权文档撰写专家。
你的任务是根据用户提供的软件信息，编写符合中国版权保护中心标准的软件著作权说明书。

说明书应包含以下部分：
1. 软件基本信息
2. 软件功能说明
3. 软件技术特点
4. 软件运行环境
5. 源代码说明
6. 使用手册概要

请使用规范、专业的语言，确保描述准确、完整。"#;

        let user_prompt = format!(
            r#"请根据以下信息编写软件著作权说明书：

【软件名称】
{}

【软件版本】
{}

【开发者】
{}

【软件描述】
{}

【功能特点】
{}

【技术特点】
{}

【源代码说明】
{}"#,
            software_name,
            software_version.unwrap_or("1.0"),
            developer,
            description,
            function_features,
            technical_features,
            source_code_summary.unwrap_or("暂无")
        );

        let response = self.call_anthropic_api(
            &model_config.model_name,
            system_prompt,
            &user_prompt,
            model_config.max_tokens,
        ).await?;

        let duration = start.elapsed().as_millis() as i32;

        Ok(CopyrightGenerationResult {
            content: response.content.first().map(|c| c.text.clone()).unwrap_or_default(),
            input_tokens: response.usage.input_tokens,
            output_tokens: response.usage.output_tokens,
            duration_ms: duration,
        })
    }

    /// 调用 Anthropic API
    async fn call_anthropic_api(
        &self,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        max_tokens: i32,
    ) -> Result<AnthropicResponse, Box<dyn std::error::Error>> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .unwrap_or_else(|_| "test-key".to_string());

        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            messages: vec![Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            }],
            system: Some(system_prompt.to_string()),
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(format!("Anthropic API error: {}", error).into());
        }

        let result: AnthropicResponse = response.json().await?;
        Ok(result)
    }

    /// 记录 AI 使用日志
    pub async fn log_usage(
        &self,
        pool: &sqlx::PgPool,
        user_id: Option<Uuid>,
        model_config_id: Option<Uuid>,
        input_tokens: i32,
        output_tokens: i32,
        duration_ms: i32,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let total_tokens = input_tokens + output_tokens;

        sqlx::query(
            r#"
            INSERT INTO ai_usage_logs (
                user_id, model_config_id, prompt_tokens, completion_tokens,
                total_tokens, duration_ms, status, error_message
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(user_id)
        .bind(model_config_id)
        .bind(input_tokens)
        .bind(output_tokens)
        .bind(total_tokens)
        .bind(duration_ms)
        .bind(status)
        .bind(error_message)
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PatentGenerationResult {
    pub content: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i32,
}

#[derive(Debug, Clone)]
pub struct CopyrightGenerationResult {
    pub content: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i32,
}
