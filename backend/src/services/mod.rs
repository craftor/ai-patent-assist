// AI 生成服务 - 简化版本
pub struct AiGenerator;

impl AiGenerator {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_patent(
        &self,
        _title: &str,
        _description: &str,
    ) -> Result<String, String> {
        Ok("专利说明书内容".to_string())
    }

    pub async fn generate_copyright(
        &self,
        _software_name: &str,
        _description: &str,
    ) -> Result<String, String> {
        Ok("软著说明书内容".to_string())
    }
}
