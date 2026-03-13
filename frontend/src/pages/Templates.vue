<template>
  <div class="templates-page">
    <el-page-header @back="handleBack" title="返回主页">
      <template #content>
        <span class="page-title">模板管理</span>
      </template>
      <template #extra>
        <el-button type="primary" @click="handleCreate">
          <el-icon><Plus /></el-icon>
          新建模板
        </el-button>
      </template>
    </el-page-header>

    <div class="content-container" v-loading="loading">
      <el-table :data="templates" style="width: 100%" stripe>
        <el-table-column prop="name" label="模板名称" min-width="200" />
        <el-table-column prop="template_type" label="类型" width="150">
          <template #default="{ row }">
            <el-tag :type="getTypeTagType(row.template_type)">
              {{ getTypeLabel(row.template_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="is_system" label="系统模板" width="100">
          <template #default="{ row }">
            <el-tag :type="isSystem(row) ? 'warning' : 'info'" size="small">
              {{ isSystem(row) ? '系统' : '自定义' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="handleEdit(row)">编辑</el-button>
            <el-button
              link
              type="danger"
              @click="handleDelete(row)"
              :disabled="isSystem(row)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty v-if="templates.length === 0" description="暂无模板" />
    </div>

    <!-- 新建/编辑对话框 - 使用可视化编辑器 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑模板' : '新建模板'"
      width="1000px"
      @close="handleDialogClose"
    >
      <el-steps :active="currentStep" finish-status="success" align-center style="margin-bottom: 20px">
        <el-step title="选择类型" />
        <el-step title="填写内容" />
        <el-step title="预览确认" />
      </el-steps>

      <!-- 步骤 1：选择模板类型 -->
      <div v-if="currentStep === 0" class="step-content">
        <el-form :model="formData" label-width="100px">
          <el-form-item label="模板类型" required>
            <el-radio-group v-model="formData.template_type" size="large">
              <el-radio-button label="patent_invention">发明专利</el-radio-button>
              <el-radio-button label="patent_utility">实用新型</el-radio-button>
              <el-radio-button label="patent_design">外观设计</el-radio-button>
              <el-radio-button label="copyright">软件著作权</el-radio-button>
            </el-radio-group>
          </el-form-item>

          <el-form-item label="模板名称" required>
            <el-input v-model="formData.name" placeholder="请输入模板名称，如：发明专利标准模板" />
          </el-form-item>

          <el-form-item label="模板描述">
            <el-input
              v-model="formData.description"
              type="textarea"
              :rows="2"
              placeholder="简单描述此模板的用途"
            />
          </el-form-item>
        </el-form>

        <el-alert
          title="如何选择模板类型"
          type="info"
          show-icon
          style="margin-top: 20px"
        >
          <template #default>
            <p>根据您的文档类型选择对应的模板：</p>
            <ul>
              <li><strong>发明专利</strong>：适用于技术创新类的发明专利申请</li>
              <li><strong>实用新型</strong>：适用于产品形状、构造的技术方案</li>
              <li><strong>外观设计</strong>：适用于产品外观设计</li>
              <li><strong>软件著作权</strong>：适用于计算机软件作品著作权登记</li>
            </ul>
          </template>
        </el-alert>
      </div>

      <!-- 步骤 2：填写模板内容 -->
      <div v-if="currentStep === 1" class="step-content">
        <el-form :model="formData" label-width="100px">
          <el-form-item label="模板名称" required>
            <el-input v-model="formData.name" placeholder="请输入模板名称" />
          </el-form-item>

          <el-form-item label="模板描述">
            <el-input
              v-model="formData.description"
              type="textarea"
              :rows="2"
              placeholder="简单描述此模板的用途"
            />
          </el-form-item>

          <el-divider>模板内容结构</el-divider>

          <!-- 根据模板类型显示不同的字段 -->
          <div v-if="isPatentType" class="template-fields">
            <el-alert
              title="填写说明"
              type="info"
              show-icon
              style="margin-bottom: 15px"
            >
              <p>请按照以下结构填写模板内容，系统会自动生成标准格式的专利文档。</p>
              <p>每个字段都是必填项，请尽可能详细描述。</p>
            </el-alert>

            <el-form-item label="技术领域">
              <el-input
                v-model="templateFields.technical_field"
                type="textarea"
                :rows="2"
                placeholder="例如：本发明涉及人工智能技术领域，具体涉及一种图像识别方法"
              />
            </el-form-item>

            <el-form-item label="背景技术">
              <el-input
                v-model="templateFields.background_art"
                type="textarea"
                :rows="3"
                placeholder="描述现有技术的不足和待解决的技术问题"
              />
            </el-form-item>

            <el-form-item label="发明内容">
              <el-input
                v-model="templateFields.invention_content"
                type="textarea"
                :rows="3"
                placeholder="描述本发明的技术方案和有益效果"
              />
            </el-form-item>

            <el-form-item label="具体实施方式">
              <el-input
                v-model="templateFields.embodiment"
                type="textarea"
                :rows="3"
                placeholder="详细描述本发明的具体实施方式和实施例"
              />
            </el-form-item>

            <el-form-item label="权利要求书">
              <el-input
                v-model="templateFields.claims"
                type="textarea"
                :rows="3"
                placeholder="撰写权利要求，如：1. 一种 XXX 方法，其特征在于，包括以下步骤..."
              />
            </el-form-item>

            <el-form-item label="摘要">
              <el-input
                v-model="templateFields.abstract"
                type="textarea"
                :rows="2"
                placeholder="简要概括本发明的技术方案"
              />
            </el-form-item>
          </div>

          <div v-if="isCopyrightType" class="template-fields">
            <el-alert
              title="填写说明"
              type="info"
              show-icon
              style="margin-bottom: 15px"
            >
              <p>请按照以下结构填写模板内容，系统会自动生成标准格式的软著文档。</p>
            </el-alert>

            <el-form-item label="软件基本信息">
              <el-input
                v-model="templateFields.software_info"
                type="textarea"
                :rows="3"
                placeholder="- 软件名称：{software_name}
- 版本号：{software_version}
- 开发者：{developer}"
              />
            </el-form-item>

            <el-form-item label="软件功能说明">
              <el-input
                v-model="templateFields.function_description"
                type="textarea"
                :rows="3"
                placeholder="详细描述软件的主要功能和应用场景"
              />
            </el-form-item>

            <el-form-item label="软件技术特点">
              <el-input
                v-model="templateFields.technical_features"
                type="textarea"
                :rows="3"
                placeholder="描述软件的技术亮点和创新点"
              />
            </el-form-item>

            <el-form-item label="运行环境">
              <el-input
                v-model="templateFields.operating_system"
                type="textarea"
                :rows="2"
                placeholder="描述软件运行的操作系统和硬件环境"
              />
            </el-form-item>

            <el-form-item label="源代码说明">
              <el-input
                v-model="templateFields.source_code_description"
                type="textarea"
                :rows="3"
                placeholder="描述源代码的目录结构和模块划分"
              />
            </el-form-item>
          </div>
        </el-form>
      </div>

      <!-- 步骤 3：预览确认 -->
      <div v-if="currentStep === 2" class="step-content">
        <el-alert
          title="模板预览"
          type="success"
          show-icon
          style="margin-bottom: 15px"
        >
          <p>以下是根据您填写的内容生成的模板预览：</p>
        </el-alert>

        <div class="template-preview-box">
          <h4>{{ formData.name || '未命名模板' }}</h4>
          <el-divider />
          <div class="preview-content" v-html="renderedPreview"></div>
        </div>
      </div>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button v-if="currentStep > 0" @click="prevStep">上一步</el-button>
        <el-button v-if="currentStep < 2" type="primary" @click="nextStep">下一步</el-button>
        <el-button v-else type="primary" :loading="saving" @click="handleSubmit">保存模板</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { templateApi, type Template, type TemplateType } from '@/api/template'

const router = useRouter()

const loading = ref(false)
const saving = ref(false)
const dialogVisible = ref(false)
const isEdit = ref(false)
const currentStep = ref(0)
const formRef = ref<FormInstance>()

const templates = ref<Template[]>([])
const editingId = ref<string | null>(null)

const formData = ref<Template>({
  id: '',
  name: '',
  template_type: 'patent_invention',
  content_template: '',
  variables: {},
  description: '',
  is_system: false,
  is_active: true,
  created_at: '',
  updated_at: '',
})

// 模板字段
const templateFields = ref({
  technical_field: '',
  background_art: '',
  invention_content: '',
  embodiment: '',
  claims: '',
  abstract: '',
  software_info: '',
  function_description: '',
  technical_features: '',
  operating_system: '',
  source_code_description: '',
})

const rules: FormRules = {
  name: [{ required: true, message: '请输入模板名称', trigger: 'blur' }],
  template_type: [{ required: true, message: '请选择模板类型', trigger: 'change' }],
}

// 是否是专利类型
const isPatentType = computed(() => {
  const type = formData.value.template_type
  return type === 'patent_invention' || type === 'patent_utility' || type === 'patent_design'
})

// 是否是软著类型
const isCopyrightType = computed(() => {
  return formData.value.template_type === 'copyright'
})

// 渲染预览
const renderedPreview = computed(() => {
  if (isPatentType.value) {
    return `
<h3>技术领域</h3>
<p>${templateFields.value.technical_field || '（待填写）'}</p>

<h3>背景技术</h3>
<p>${templateFields.value.background_art || '（待填写）'}</p>

<h3>发明内容</h3>
<p>${templateFields.value.invention_content || '（待填写）'}</p>

<h3>具体实施方式</h3>
<p>${templateFields.value.embodiment || '（待填写）'}</p>

<h3>权利要求书</h3>
<p>${templateFields.value.claims || '（待填写）'}</p>

<h3>摘要</h3>
<p>${templateFields.value.abstract || '（待填写）'}</p>
    `.trim()
  } else {
    return `
<h3>软件基本信息</h3>
<div style="white-space: pre-wrap">${templateFields.value.software_info || '（待填写）'}</div>

<h3>软件功能说明</h3>
<p>${templateFields.value.function_description || '（待填写）'}</p>

<h3>软件技术特点</h3>
<p>${templateFields.value.technical_features || '（待填写）'}</p>

<h3>运行环境</h3>
<p>${templateFields.value.operating_system || '（待填写）'}</p>

<h3>源代码说明</h3>
<p>${templateFields.value.source_code_description || '（待填写）'}</p>
    `.trim()
  }
})

const getTypeLabel = (type: TemplateType | string): string => {
  const labels: Record<string, string> = {
    patent_invention: '发明专利',
    patent_utility: '实用新型',
    patent_design: '外观设计',
    copyright: '软件著作权',
  }
  return labels[type] || type
}

const getTypeTagType = (type: TemplateType | string): 'success' | 'warning' | 'info' | 'danger' => {
  const types: Record<string, 'success' | 'warning' | 'info' | 'danger'> = {
    patent_invention: 'success',
    patent_utility: 'warning',
    patent_design: 'info',
    copyright: 'danger',
  }
  return types[type] || 'info'
}

// 判断是否是系统模板（兼容字符串和布尔值）
const isSystem = (row: Template): boolean => {
  return row.is_system === true || row.is_system === 'true'
}

const fetchTemplates = async () => {
  loading.value = true
  try {
    const response = await templateApi.listSilent()
    templates.value = response.data || []
  } catch (error) {
    console.error('Failed to fetch templates:', error)
    // 静默失败，不显示错误消息
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  isEdit.value = false
  editingId.value = null
  currentStep.value = 0
  formData.value = {
    id: '',
    name: '',
    template_type: 'patent_invention',
    content_template: '',
    variables: {},
    description: '',
    is_system: false,
    is_active: true,
    created_at: '',
    updated_at: '',
  }
  templateFields.value = {
    technical_field: '',
    background_art: '',
    invention_content: '',
    embodiment: '',
    claims: '',
    abstract: '',
    software_info: '',
    function_description: '',
    technical_features: '',
    operating_system: '',
    source_code_description: '',
  }
  dialogVisible.value = true
}

const handleEdit = (row: Template) => {
  isEdit.value = true
  editingId.value = row.id
  currentStep.value = 0
  formData.value = { ...row }
  dialogVisible.value = true
}

const handleDelete = async (row: Template) => {
  if (isSystem(row)) {
    ElMessage.warning('系统模板无法删除')
    return
  }

  try {
    await ElMessageBox.confirm('确定要删除此模板吗？', '确认删除', {
      type: 'warning',
    })
  } catch {
    return
  }

  try {
    await templateApi.delete(row.id)
    ElMessage.success('删除成功')
    fetchTemplates()
  } catch (error) {
    console.error('Failed to delete template:', error)
    ElMessage.error('删除失败')
  }
}

const nextStep = () => {
  if (currentStep.value === 0) {
    // 验证第一步
    if (!formData.value.name) {
      ElMessage.warning('请输入模板名称')
      return
    }
  }
  if (currentStep.value === 1) {
    // 验证第二步
    if (isPatentType.value) {
      if (!templateFields.value.technical_field || !templateFields.value.background_art) {
        ElMessage.warning('请填写技术领域和背景技术')
        return
      }
    } else {
      if (!templateFields.value.software_info || !templateFields.value.function_description) {
        ElMessage.warning('请填写软件基本信息和功能说明')
        return
      }
    }
    // 生成模板内容
    generateTemplateContent()
  }
  currentStep.value++
}

const prevStep = () => {
  currentStep.value--
}

// 生成模板内容字符串
const generateTemplateContent = () => {
  if (isPatentType.value) {
    formData.value.content_template = `# 发明专利说明书

## 技术领域
${templateFields.value.technical_field}

## 背景技术
${templateFields.value.background_art}

## 发明内容
${templateFields.value.invention_content}

## 具体实施方式
${templateFields.value.embodiment}

## 权利要求书
${templateFields.value.claims}

## 摘要
${templateFields.value.abstract}`
  } else {
    formData.value.content_template = `# 软件著作权说明书

## 软件基本信息
${templateFields.value.software_info}

## 软件功能说明
${templateFields.value.function_description}

## 软件技术特点
${templateFields.value.technical_features}

## 运行环境
${templateFields.value.operating_system}

## 源代码说明
${templateFields.value.source_code_description}`
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return

  saving.value = true
  try {
    // 确保生成了模板内容
    if (!formData.value.content_template) {
      generateTemplateContent()
    }

    const submitData = {
      name: formData.value.name,
      template_type: formData.value.template_type,
      content_template: formData.value.content_template,
      variables: formData.value.variables,
      description: formData.value.description,
      is_active: formData.value.is_active,
    }

    if (isEdit.value && editingId.value) {
      await templateApi.update(editingId.value, submitData)
      ElMessage.success('更新成功')
    } else {
      await templateApi.create(submitData)
      ElMessage.success('创建成功')
    }

    dialogVisible.value = false
    fetchTemplates()
  } catch (error) {
    console.error('Failed to submit template:', error)
    ElMessage.error(isEdit.value ? '更新失败' : '创建失败')
  } finally {
    saving.value = false
  }
}

const handleDialogClose = () => {
  formRef.value?.resetFields()
  currentStep.value = 0
}

const handleBack = () => {
  router.back()
}

onMounted(() => {
  fetchTemplates()
})
</script>

<style scoped>
.templates-page {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.page-title {
  font-size: 18px;
  font-weight: 500;
  margin-left: 10px;
}

.content-container {
  margin-top: 20px;
  background: #fff;
  border-radius: 8px;
  padding: 20px;
}

.step-content {
  padding: 20px 0;
  max-height: 500px;
  overflow-y: auto;
}

.template-fields {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.template-preview-box {
  padding: 20px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
  max-height: 400px;
  overflow-y: auto;
}

.template-preview-box h4 {
  margin-top: 0;
  color: var(--el-text-color-primary);
}

.preview-content {
  line-height: 1.8;
  white-space: pre-wrap;
}

.preview-content h3 {
  color: var(--el-text-color-primary);
  margin-top: 20px;
  margin-bottom: 10px;
  font-size: 16px;
}

.preview-content p {
  color: var(--el-text-color-regular);
  margin: 0;
}
</style>
