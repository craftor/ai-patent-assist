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
            <el-tag :type="row.is_system ? 'warning' : 'info'" size="small">
              {{ row.is_system ? '系统' : '自定义' }}
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
              :disabled="row.is_system"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty v-if="templates.length === 0" description="暂无模板" />
    </div>

    <!-- 新建/编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑模板' : '新建模板'"
      width="800px"
      @close="handleDialogClose"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="rules"
        label-width="120px"
      >
        <el-form-item label="模板名称" prop="name">
          <el-input v-model="formData.name" placeholder="请输入模板名称" />
        </el-form-item>
        <el-form-item label="模板类型" prop="template_type">
          <el-select v-model="formData.template_type" style="width: 100%">
            <el-option label="发明专利" value="patent_invention" />
            <el-option label="实用新型" value="patent_utility" />
            <el-option label="外观设计" value="patent_design" />
            <el-option label="软件著作权" value="copyright" />
          </el-select>
        </el-form-item>
        <el-form-item label="模板内容" prop="content_template">
          <el-input
            v-model="formData.content_template"
            type="textarea"
            :rows="15"
            placeholder="请输入模板内容，使用 {变量名} 表示可变内容"
          />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input
            v-model="formData.description"
            type="textarea"
            :rows="3"
            placeholder="请输入模板描述"
          />
        </el-form-item>
        <el-form-item label="启用状态">
          <el-switch v-model="formData.is_active" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="saving" @click="handleSubmit">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
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

const rules: FormRules = {
  name: [{ required: true, message: '请输入模板名称', trigger: 'blur' }],
  template_type: [{ required: true, message: '请选择模板类型', trigger: 'change' }],
  content_template: [{ required: true, message: '请输入模板内容', trigger: 'blur' }],
}

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

const fetchTemplates = async () => {
  loading.value = true
  try {
    const response = await templateApi.list()
    templates.value = response.data || []
  } catch (error) {
    console.error('Failed to fetch templates:', error)
    ElMessage.error('加载模板列表失败')
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  isEdit.value = false
  editingId.value = null
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
  dialogVisible.value = true
}

const handleEdit = (row: Template) => {
  isEdit.value = true
  editingId.value = row.id
  formData.value = { ...row }
  dialogVisible.value = true
}

const handleDelete = async (row: Template) => {
  if (row.is_system) {
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

const handleSubmit = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid: boolean) => {
    if (!valid) return

    saving.value = true
    try {
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
  })
}

const handleDialogClose = () => {
  formRef.value?.resetFields()
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
</style>
