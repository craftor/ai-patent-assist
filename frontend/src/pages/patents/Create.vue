<template>
  <div class="create-patent-page">
    <el-page-header @back="handleBack" title="返回专利列表">
      <template #content>
        <span class="page-title">新建专利文档</span>
      </template>
    </el-page-header>

    <div class="form-container">
      <el-card>
        <el-form
          ref="formRef"
          :model="form"
          :rules="rules"
          label-width="120px"
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

          <!-- 基本信息 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Document /></el-icon>
              基本信息
            </h3>

            <el-form-item label="所属项目" prop="project_id">
              <el-select v-model="form.project_id" placeholder="请选择所属项目" style="width: 100%">
                <el-option
                  v-for="project in projects"
                  :key="project.id"
                  :label="project.name"
                  :value="project.id"
                  :disabled="project.type !== 'patent'"
                />
              </el-select>
            </el-form-item>

            <el-form-item label="专利类型" prop="patent_type">
              <el-radio-group v-model="form.patent_type" size="large">
                <el-radio-button label="invention">
                  <el-icon><Rank /></el-icon>
                  发明专利
                </el-radio-button>
                <el-radio-button label="utility">
                  <el-icon><Setting /></el-icon>
                  实用新型
                </el-radio-button>
                <el-radio-button label="design">
                  <el-icon><Picture /></el-icon>
                  外观设计
                </el-radio-button>
              </el-radio-group>
            </el-form-item>

            <el-form-item label="专利名称" prop="title">
              <el-input
                v-model="form.title"
                placeholder="请输入专利名称，如：一种基于人工智能的图像识别方法"
                maxlength="200"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="技术领域" prop="technical_field">
              <el-input
                v-model="form.technical_field"
                type="textarea"
                :rows="3"
                placeholder="请简要描述本发明涉及的技术领域，例如：本发明涉及人工智能技术领域，具体涉及一种图像识别方法..."
                maxlength="1000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 背景技术 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Clock /></el-icon>
              背景技术
            </h3>

            <el-form-item label="背景技术描述" prop="background_art">
              <el-input
                v-model="form.background_art"
                type="textarea"
                :rows="5"
                placeholder="请描述现有技术的情况和存在的不足，例如：
1. 现有技术的基本原理
2. 现有技术存在的缺陷或不足
3. 需要解决的技术问题"
                maxlength="3000"
                show-word-limit
              />
            </el-form-item>
          </div>

          <!-- 发明内容 -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><Aim /></el-icon>
              发明内容
            </h3>

            <el-form-item label="技术问题" prop="invention_description">
              <el-input
                v-model="form.invention_description"
                type="textarea"
                :rows="4"
                placeholder="本发明要解决什么技术问题？"
                maxlength="5000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="技术方案">
              <el-input
                v-model="technical_solution"
                type="textarea"
                :rows="4"
                placeholder="为解决上述技术问题，本发明采用了什么技术方案？"
                maxlength="5000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="有益效果">
              <el-input
                v-model="beneficial_effects"
                type="textarea"
                :rows="4"
                placeholder="本发明与现有技术相比有哪些有益效果？"
                maxlength="5000"
                show-word-limit
              />
            </el-form-item>

            <el-form-item label="具体实施方式">
              <div class="embodiment-section">
                <div v-for="(embodiment, index) in form.embodiments" :key="index" class="embodiment-item">
                  <div class="embodiment-header">
                    <span class="embodiment-label">实施方式 {{ index + 1 }}</span>
                    <el-button
                      type="danger"
                      size="small"
                      @click="removeEmbodiment(index)"
                      :disabled="form.embodiments.length <= 1"
                    >
                      删除
                    </el-button>
                  </div>
                  <el-input
                    v-model="form.embodiments[index]"
                    type="textarea"
                    :rows="4"
                    placeholder="请描述该实施方式的具体内容..."
                    maxlength="2000"
                  />
                </div>
                <el-button type="primary" @click="addEmbodiment" style="margin-top: 10px">
                  <el-icon><Plus /></el-icon>
                  添加实施方式
                </el-button>
              </div>
            </el-form-item>
          </div>

          <!-- 权利要求（可选） -->
          <div class="form-section">
            <h3 class="section-title">
              <el-icon><List /></el-icon>
              权利要求（可选）
            </h3>

            <el-form-item label="权利要求构思" prop="claims_input">
              <el-input
                v-model="form.claims_input"
                type="textarea"
                :rows="4"
                placeholder="请输入您初步构思的权利要求..."
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
              {{ generating ? 'AI 生成中...' : 'AI 生成专利文档' }}
            </el-button>
          </div>
        </el-form>
      </el-card>
    </div>

    <!-- 生成结果对话框 -->
    <el-dialog
      v-model="showResultDialog"
      title="专利文档已生成"
      width="800px"
      :close-on-click-modal="false"
    >
      <el-alert
        title="生成成功"
        description="AI 已根据您提供的信息生成专利文档，请查看下方预览。建议您仔细检查并编辑完善后再提交审核。"
        type="success"
        show-icon
        style="margin-bottom: 20px"
      />

      <el-tabs>
        <el-tab-pane label="专利说明书">
          <div class="preview-content">
            <h4>{{ generatedData.title }}</h4>
            <p><strong>技术领域：</strong></p>
            <p>{{ generatedData.technical_field }}</p>
            <p><strong>背景技术：</strong></p>
            <p>{{ generatedData.background_art }}</p>
            <p><strong>发明内容：</strong></p>
            <p>{{ generatedData.invention_content }}</p>
          </div>
        </el-tab-pane>
        <el-tab-pane label="权利要求书">
          <div class="preview-content">
            <p>{{ generatedData.claims_text }}</p>
          </div>
        </el-tab-pane>
        <el-tab-pane label="摘要">
          <div class="preview-content">
            <p>{{ generatedData.abstract_text }}</p>
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
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import {
  Document,
  Rank,
  Setting,
  Picture,
  Clock,
  Aim,
  List,
  Plus,
  MagicStick,
} from '@element-plus/icons-vue'
import { projectApi, type Project } from '@/api/project'
import { templateApi, type Template } from '@/api/template'

// 专利类型
type PatentType = 'invention' | 'utility' | 'design'

interface CreatePatentForm {
  project_id: string
  patent_type: PatentType
  title: string
  technical_field: string
  background_art: string
  invention_description: string
  embodiments: string[]
  embodiments_input: string
  claims_input: string
}

const router = useRouter()
const route = useRoute()

const formRef = ref<FormInstance>()
const projects = ref<Project[]>([])
const templates = ref<Template[]>([])
const selectedTemplateId = ref<string>('')
const generating = ref(false)
const showResultDialog = ref(false)

// 额外字段，用于合并到 invention_description
const technical_solution = ref('')
const beneficial_effects = ref('')

const form = reactive<CreatePatentForm>({
  project_id: '',
  patent_type: 'invention',
  title: '',
  technical_field: '',
  background_art: '',
  invention_description: '',
  embodiments: [''],
  embodiments_input: '',
  claims_input: '',
})

const rules: FormRules = {
  project_id: [{ required: true, message: '请选择所属项目', trigger: 'change' }],
  patent_type: [{ required: true, message: '请选择专利类型', trigger: 'change' }],
  title: [
    { required: true, message: '请输入专利名称', trigger: 'blur' },
    { min: 5, message: '专利名称至少 5 个字', trigger: 'blur' },
  ],
  technical_field: [
    { required: true, message: '请输入技术领域', trigger: 'blur' },
    { min: 10, message: '技术领域描述至少 10 个字', trigger: 'blur' },
  ],
  background_art: [
    { required: true, message: '请输入背景技术', trigger: 'blur' },
    { min: 20, message: '背景技术描述至少 20 个字', trigger: 'blur' },
  ],
  invention_description: [
    { required: true, message: '请输入技术问题', trigger: 'blur' },
    { min: 10, message: '技术问题描述至少 10 个字', trigger: 'blur' },
  ],
}

const generatedData = ref({
  title: '',
  technical_field: '',
  background_art: '',
  invention_content: '',
  claims_text: '',
  abstract_text: '',
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
      projects.value = response.data.filter(p => p.type === 'patent')
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
      templates.value = response.data.filter(t =>
        t.template_type === 'patent_invention' ||
        t.template_type === 'patent_utility' ||
        t.template_type === 'patent_design'
      )
    }
  } catch (error) {
    console.error('Failed to fetch templates:', error)
  }
}

// 处理模板选择 - 填充表单
const handleTemplateChange = () => {
  const template = selectedTemplate.value
  if (!template) return

  ElMessage.success('已选择模板，请继续填写表单内容')
}

// 添加实施方式
const addEmbodiment = () => {
  form.embodiments.push('')
}

// 删除实施方式
const removeEmbodiment = (index: number) => {
  form.embodiments.splice(index, 1)
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

    // 收集实施方式
    const embodiments = form.embodiments.filter(e => e.trim() !== '')
    if (embodiments.length === 0) {
      ElMessage.warning('请至少添加一个实施方式')
      return
    }

    // 合并发明内容字段
    const fullInventionContent = [
      form.invention_description,
      technical_solution.value,
      beneficial_effects.value
    ].filter(Boolean).join('\n\n')

    generating.value = true
    try {
      // TODO: 调用后端 AI 生成 API
      await new Promise(resolve => setTimeout(resolve, 1500))

      generatedData.value = {
        title: form.title,
        technical_field: form.technical_field,
        background_art: form.background_art,
        invention_content: fullInventionContent,
        claims_text: form.claims_input || '（待 AI 生成权利要求书）',
        abstract_text: `本发明公开了${form.title}，属于${form.technical_field}领域。${technical_solution.value?.substring(0, 200)}...`,
      }

      ElMessage.success('AI 生成成功')
      showResultDialog.value = true
    } catch (error) {
      console.error('Failed to generate patent:', error)
      ElMessage.error('生成失败，请稍后重试')
    } finally {
      generating.value = false
    }
  })
}

// 查看详情
const handleViewDetail = () => {
  showResultDialog.value = false
  ElMessage.info('详情页功能待实现')
}

onMounted(() => {
  fetchProjects()
  fetchTemplates()
})
</script>

<style scoped>
.create-patent-page {
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

.embodiment-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.embodiment-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
}

.embodiment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.embodiment-label {
  font-weight: 500;
  color: var(--el-text-color-regular);
}

.embodiment-item :deep(.el-input),
.embodiment-item :deep(.el-textarea) {
  width: 100%;
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
