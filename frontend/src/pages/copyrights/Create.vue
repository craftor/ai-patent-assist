<template>
  <div class="create-copyright-page">
    <el-page-header @back="handleBack" title="返回软著列表">
      <template #content>
        <span class="page-title">新建软件著作权文档</span>
      </template>
    </el-page-header>

    <div class="form-container">
      <el-card>
        <el-form
          ref="formRef"
          :model="form"
          :rules="rules"
          label-width="140px"
          size="large"
        >
          <!-- 第一步：选择模板 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Document /></el-icon>
              选择模板
            </h3>

            <el-form-item label="文档模板">
              <el-select
                v-model="selectedTemplateId"
                placeholder="选择模板（可选），可快速填充表单"
                style="width: 100%"
                @change="handleTemplateChange"
              >
                <el-option
                  v-for="template in templates"
                  :key="template.id"
                  :label="template.name"
                  :value="template.id"
                >
                  <div class="template-option">
                    <span>{{ template.name }}</span>
                    <el-tag v-if="template.is_system" type="warning" size="small">系统模板</el-tag>
                  </div>
                </el-option>
              </el-select>
            </el-form-item>

            <el-alert
              v-if="selectedTemplate"
              title="已选择模板"
              type="info"
              show-icon
              :closable="false"
              style="margin-top: 10px"
            >
              <div class="template-info">
                <p><strong>模板内容预览：</strong></p>
                <div class="template-preview">{{ selectedTemplate.content_template }}</div>
              </div>
            </el-alert>
          </div>

          <!-- 软件基本信息 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Monitor /></el-icon>
              软件基本信息
            </h3>

            <el-form-item label="所属项目" prop="project_id">
              <el-select v-model="form.project_id" placeholder="请选择所属项目" style="width: 100%">
                <el-option
                  v-for="project in projects"
                  :key="project.id"
                  :label="project.name"
                  :value="project.id"
                />
              </el-select>
            </el-form-item>

            <el-form-item label="软件名称" prop="software_name">
              <el-input
                v-model="form.software_name"
                placeholder="请输入软件名称，如：智能图像识别系统 V1.0"
                maxlength="200"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="软件版本号" prop="software_version">
              <el-input
                v-model="form.software_version"
                placeholder="请输入版本号，如：V1.0.0"
                maxlength="50"
              />
            </el-form-item>

            <el-form-item label="开发者" prop="developer">
              <el-input
                v-model="form.developer"
                placeholder="请输入开发者姓名或公司名称"
                maxlength="200"
              />
            </el-form-item>

            <el-form-item label="完成日期" prop="completion_date">
              <el-date-picker
                v-model="form.completion_date"
                type="date"
                placeholder="选择完成日期"
                style="width: 100%"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
              />
            </el-form-item>

            <el-form-item label="编程语言" prop="programming_language">
              <el-select
                v-model="form.programming_language"
                placeholder="请选择编程语言"
                allow-create
                filterable
                style="width: 100%"
              >
                <el-option label="Java" value="Java" />
                <el-option label="Python" value="Python" />
                <el-option label="JavaScript" value="JavaScript" />
                <el-option label="TypeScript" value="TypeScript" />
                <el-option label="C++" value="C++" />
                <el-option label="C#" value="C#" />
                <el-option label="Go" value="Go" />
                <el-option label="Rust" value="Rust" />
                <el-option label="PHP" value="PHP" />
                <el-option label="Swift" value="Swift" />
                <el-option label="Kotlin" value="Kotlin" />
                <el-option label="其他" value="其他" />
              </el-select>
            </el-form-item>
          </div>

          <!-- 运行环境 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Setting /></el-icon>
              运行环境
            </h3>

            <el-form-item label="操作系统" prop="operating_system">
              <el-select
                v-model="form.operating_system"
                placeholder="请选择操作系统"
                allow-create
                filterable
                style="width: 100%"
              >
                <el-option label="Windows 10/11" value="Windows 10/11" />
                <el-option label="Linux" value="Linux" />
                <el-option label="macOS" value="macOS" />
                <el-option label="Android" value="Android" />
                <el-option label="iOS" value="iOS" />
                <el-option label="跨平台" value="跨平台" />
                <el-option label="Web 浏览器" value="Web 浏览器" />
                <el-option label="其他" value="其他" />
              </el-select>
            </el-form-item>

            <el-form-item label="硬件要求" prop="hardware_requirements">
              <el-input
                v-model="form.hardware_requirements"
                type="textarea"
                :rows="2"
                placeholder="请描述最低硬件配置要求，如：CPU 2.0GHz 以上，内存 4GB 以上，硬盘空间 500MB"
                maxlength="1000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="依赖环境" prop="dependencies">
              <el-input
                v-model="form.dependencies"
                type="textarea"
                :rows="2"
                placeholder="请描述运行所需的依赖环境，如：JDK 1.8+、Node.js 16+、.NET 6.0 等"
                maxlength="1000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 功能说明 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><List /></el-icon>
              功能说明
            </h3>

            <el-form-item label="软件分类" prop="software_category">
              <el-select
                v-model="form.software_category"
                placeholder="请选择软件分类"
                allow-create
                filterable
                style="width: 100%"
              >
                <el-option label="系统软件" value="系统软件" />
                <el-option label="应用软件" value="应用软件" />
                <el-option label="中间件" value="中间件" />
                <el-option label="数据库" value="数据库" />
                <el-option label="网络软件" value="网络软件" />
                <el-option label="多媒体软件" value="多媒体软件" />
                <el-option label="游戏软件" value="游戏软件" />
                <el-option label="工业控制软件" value="工业控制软件" />
                <el-option label="人工智能软件" value="人工智能软件" />
                <el-option label="其他" value="其他" />
              </el-select>
            </el-form-item>

            <el-form-item label="功能特性描述" prop="function_features">
              <el-input
                v-model="form.function_features"
                type="textarea"
                :rows="5"
                placeholder="请详细描述软件的各项功能，例如：
1. 用户管理功能：支持用户注册、登录、权限管理
2. 数据处理功能：支持数据导入、导出、分析统计
3. 可视化功能：提供图表展示、报表生成
4. ...（请尽可能详细列举）"
                maxlength="5000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="技术特点" prop="technical_features">
              <el-input
                v-model="form.technical_features"
                type="textarea"
                :rows="4"
                placeholder="请描述软件的技术亮点和创新点，例如：
1. 采用的核心技术架构
2. 性能优化措施
3. 安全保护机制
4. 相比同类产品的优势"
                maxlength="3000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 源代码说明 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Document /></el-icon>
              源代码说明
            </h3>

            <el-form-item label="代码总行数" prop="line_count">
              <el-input-number
                v-model="form.line_count"
                :min="0"
                :max="1000000"
                placeholder="请输入代码总行数"
                style="width: 100%"
              />
            </el-form-item>

            <el-form-item label="源代码结构说明" prop="source_code_structure">
              <el-input
                v-model="form.source_code_structure"
                type="textarea"
                :rows="4"
                placeholder="请描述源代码的目录结构和模块划分，例如：
/src
  /controller - 控制器层
  /service - 业务逻辑层
  /dao - 数据访问层
  /model - 数据模型
  /utils - 工具类
  /config - 配置文件"
                maxlength="3000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="核心算法说明" prop="core_algorithms">
              <el-input
                v-model="form.core_algorithms"
                type="textarea"
                :rows="4"
                placeholder="如有核心算法，请详细描述其原理和实现方式；如无特殊算法可填'无'"
                maxlength="3000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 用户手册说明（可选） -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><User /></el-icon>
              用户手册说明（可选）
            </h3>

            <el-form-item label="用户手册概述" prop="manual_overview">
              <el-input
                v-model="form.manual_overview"
                type="textarea"
                :rows="3"
                placeholder="请简要描述用户手册的内容结构，如：本手册包含软件安装指南、功能操作说明、常见问题解答等章节"
                maxlength="2000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="使用说明" prop="usage_instructions">
              <el-input
                v-model="form.usage_instructions"
                type="textarea"
                :rows="4"
                placeholder="请描述软件的基本使用流程，例如：
1. 安装部署步骤
2. 启动和登录方式
3. 主要功能操作方法
4. 退出和卸载方式"
                maxlength="3000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 提交按钮 -->
          <div class="form-actions">
            <el-button @click="handleBack" :disabled="generating">取消</el-button>
            <el-button type="primary" @click="handleGenerate" :loading="generating">
              <el-icon><MagicStick /></el-icon>
              {{ generating ? 'AI 生成中...' : 'AI 生成软著文档' }}
            </el-button>
          </div>
        </el-form>
      </el-card>
    </div>

    <!-- 生成结果对话框 -->
    <el-dialog
      v-model="showResultDialog"
      title="软著文档已生成"
      width="800px"
      :close-on-click-modal="false"
    >
      <el-alert
        title="生成成功"
        description="AI 已根据您提供的信息生成软著文档，请查看下方预览。建议您仔细检查并编辑完善后再提交审核。"
        type="success"
        show-icon
        style="margin-bottom: 20px"
      />

      <el-tabs>
        <el-tab-pane label="软件说明">
          <div class="preview-content">
            <h4>{{ generatedData.software_name }}</h4>
            <p><strong>软件版本：</strong>{{ generatedData.software_version }}</p>
            <p><strong>开发者：</strong>{{ generatedData.developer }}</p>
            <p><strong>功能特性：</strong></p>
            <p>{{ generatedData.function_features }}</p>
            <p><strong>技术特点：</strong></p>
            <p>{{ generatedData.technical_features }}</p>
          </div>
        </el-tab-pane>
        <el-tab-pane label="源代码说明">
          <div class="preview-content">
            <p><strong>编程语言：</strong>{{ generatedData.programming_language }}</p>
            <p><strong>代码行数：</strong>{{ generatedData.line_count }} 行</p>
            <p><strong>代码结构：</strong></p>
            <p>{{ generatedData.source_code_structure }}</p>
          </div>
        </el-tab-pane>
        <el-tab-pane label="用户手册">
          <div class="preview-content">
            <p><strong>运行环境：</strong>{{ generatedData.operating_system }}</p>
            <p><strong>使用说明：</strong></p>
            <p>{{ generatedData.usage_instructions }}</p>
          </div>
        </el-tab-pane>
      </el-tabs>

      <template #footer>
        <el-button @click="showResultDialog = false">关闭</el-button>
        <el-button type="primary" @click="handleViewDetail">查看详情</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import {
  Monitor,
  Setting,
  List,
  Document,
  User,
  MagicStick,
} from '@element-plus/icons-vue'
import { projectApi, type Project } from '@/api/project'
import { templateApi, type Template } from '@/api/template'

interface CreateCopyrightForm {
  project_id: string
  software_name: string
  software_version: string
  developer: string
  completion_date: string
  programming_language: string
  operating_system: string
  hardware_requirements: string
  dependencies: string
  software_category: string
  function_features: string
  technical_features: string
  line_count: number
  source_code_structure: string
  core_algorithms: string
  manual_overview: string
  usage_instructions: string
}

const router = useRouter()
const route = useRoute()

const formRef = ref<FormInstance>()
const projects = ref<Project[]>([])
const templates = ref<Template[]>([])
const selectedTemplateId = ref<string>('')
const generating = ref(false)
const showResultDialog = ref(false)

const form = reactive<CreateCopyrightForm>({
  project_id: '',
  software_name: '',
  software_version: '',
  developer: '',
  completion_date: '',
  programming_language: '',
  operating_system: '',
  hardware_requirements: '',
  dependencies: '',
  software_category: '',
  function_features: '',
  technical_features: '',
  line_count: 0,
  source_code_structure: '',
  core_algorithms: '',
  manual_overview: '',
  usage_instructions: '',
})

const rules: FormRules = {
  project_id: [{ required: true, message: '请选择所属项目', trigger: 'change' }],
  software_name: [
    { required: true, message: '请输入软件名称', trigger: 'blur' },
    { min: 3, message: '软件名称至少 3 个字', trigger: 'blur' },
  ],
  developer: [{ required: true, message: '请输入开发者', trigger: 'blur' }],
  function_features: [
    { required: true, message: '请输入功能特性描述', trigger: 'blur' },
    { min: 20, message: '功能描述至少 20 个字', trigger: 'blur' },
  ],
  technical_features: [
    { required: true, message: '请输入技术特点', trigger: 'blur' },
    { min: 10, message: '技术特点描述至少 10 个字', trigger: 'blur' },
  ],
}

const generatedData = ref({
  software_name: '',
  software_version: '',
  developer: '',
  function_features: '',
  technical_features: '',
  programming_language: '',
  line_count: 0,
  source_code_structure: '',
  operating_system: '',
  usage_instructions: '',
})

// 选中的模板
const selectedTemplate = computed(() => {
  return templates.value.find(t => t.id === selectedTemplateId.value)
})

// 获取项目列表
const fetchProjects = async () => {
  try {
    const response = await projectApi.list()
    if (response.data) {
      projects.value = response.data.filter(p => p.type === 'copyright')
    }
  } catch (error) {
    console.error('Failed to fetch projects:', error)
  }
}

// 获取模板列表
const fetchTemplates = async () => {
  try {
    const response = await templateApi.list()
    if (response.data) {
      templates.value = response.data.filter(t => t.template_type === 'copyright')
    }
  } catch (error) {
    console.error('Failed to fetch templates:', error)
  }
}

// 处理模板选择 - 填充表单
const handleTemplateChange = () => {
  const template = selectedTemplate.value
  if (!template) return

  // 解析模板内容，提取占位符对应的字段
  const content = template.content_template

  // 尝试从模板中提取字段值
  const fieldMap: Record<string, string> = {
    software_name: extractTemplateField(content, '软件名称'),
    software_version: extractTemplateField(content, '版本号'),
    developer: extractTemplateField(content, '开发者'),
    function_description: extractTemplateField(content, '软件功能说明'),
    technical_features: extractTemplateField(content, '软件技术特点'),
    operating_system: extractTemplateField(content, '运行环境'),
    source_code_description: extractTemplateField(content, '源代码说明'),
  }

  // 填充表单
  if (fieldMap.software_name) form.software_name = fieldMap.software_name
  if (fieldMap.software_version) form.software_version = fieldMap.software_version
  if (fieldMap.developer) form.developer = fieldMap.developer
  if (fieldMap.function_description) form.function_features = fieldMap.function_description
  if (fieldMap.technical_features) form.technical_features = fieldMap.technical_features
  if (fieldMap.operating_system) form.operating_system = fieldMap.operating_system

  ElMessage.success('已根据模板填充表单，请继续完善内容')
}

// 从模板内容中提取字段值
const extractTemplateField = (content: string, fieldName: string): string => {
  // 支持多种格式匹配
  // 格式 1: - 字段名：{value}
  const regex1 = new RegExp(`-\\s*${fieldName}[:：]\\s*\\{[^}]*\\}`, 'i')
  // 格式 2: ## 字段名\n{value}
  const regex2 = new RegExp(`##\\s*${fieldName}\\s*\\n([\\s\\S]*?)(?=##|$)`, 'i')

  const match1 = content.match(regex1)
  if (match1) {
    // 如果是占位符，返回空字符串
    return ''
  }

  const match2 = content.match(regex2)
  if (match2 && match2[1]) {
    const value = match2[1].trim()
    // 如果内容是占位符格式，返回空
    if (value.match(/^\{.*\}$/)) return ''
    return value
  }

  return ''
}

// 返回
const handleBack = () => {
  router.back()
}

// AI 生成
const handleGenerate = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid) => {
    if (!valid) return

    generating.value = true
    try {
      // TODO: 调用后端 AI 生成 API
      // const response = await copyrightApi.generate({
      //   project_id: form.project_id,
      //   software_name: form.software_name,
      //   software_version: form.software_version,
      //   developer: form.developer,
      //   description: form.function_features,
      //   function_features: form.function_features,
      //   technical_features: form.technical_features,
      //   source_code_summary: form.source_code_structure,
      // })

      // 模拟生成结果（临时）
      await new Promise(resolve => setTimeout(resolve, 2000))

      generatedData.value = {
        software_name: form.software_name,
        software_version: form.software_version || 'V1.0',
        developer: form.developer,
        function_features: form.function_features,
        technical_features: form.technical_features,
        programming_language: form.programming_language || '未指定',
        line_count: form.line_count || 0,
        source_code_structure: form.source_code_structure,
        operating_system: form.operating_system || '未指定',
        usage_instructions: form.usage_instructions || form.manual_overview,
      }

      ElMessage.success('AI 生成成功')
      showResultDialog.value = true
    } catch (error) {
      console.error('Failed to generate copyright:', error)
      ElMessage.error('生成失败，请稍后重试')
    } finally {
      generating.value = false
    }
  })
}

// 查看详情
const handleViewDetail = () => {
  showResultDialog.value = false
  // TODO: 跳转到详情页
  ElMessage.info('详情页功能待实现')
}

onMounted(() => {
  fetchProjects()
  fetchTemplates()
})
</script>

<style scoped>
.create-copyright-page {
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
}

.page-title {
  font-size: 18px;
  font-weight: 500;
  margin-left: 10px;
}

.form-container {
  margin-top: 20px;
}

.form-section {
  margin-bottom: 30px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.form-section:last-of-type {
  border-bottom: none;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  color: var(--el-text-color-primary);
  margin-bottom: 20px;
}

.template-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.template-info {
  margin-top: 10px;
}

.template-preview {
  margin-top: 8px;
  padding: 12px;
  background: var(--el-fill-color);
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
  font-family: monospace;
  font-size: 13px;
  white-space: pre-wrap;
  line-height: 1.6;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.preview-content {
  padding: 20px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
  max-height: 400px;
  overflow-y: auto;
}

.preview-content h4 {
  margin-top: 0;
  color: var(--el-text-color-primary);
}

.preview-content p {
  line-height: 1.8;
  white-space: pre-wrap;
}
</style>
