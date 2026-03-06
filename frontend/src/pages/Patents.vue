<template>
  <div class="patents-page">
    <div class="page-header">
      <h2>专利文档</h2>
      <el-button type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>
        新建专利
      </el-button>
    </div>

    <!-- 搜索和筛选 -->
    <el-card class="filter-card">
      <el-form :inline="true">
        <el-form-item label="关键词">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索专利名称"
            clearable
            style="width: 200px"
          />
        </el-form-item>
        <el-form-item label="专利类型">
          <el-select v-model="filterPatentType" placeholder="全部" clearable style="width: 120px">
            <el-option label="发明专利" value="invention" />
            <el-option label="实用新型" value="utility" />
            <el-option label="外观设计" value="design" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="filterStatus" placeholder="全部" clearable style="width: 120px">
            <el-option label="草稿" value="draft" />
            <el-option label="审核中" value="reviewing" />
            <el-option label="已通过" value="approved" />
            <el-option label="已拒绝" value="rejected" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 专利列表 -->
    <el-card class="table-card">
      <el-table :data="filteredPatents" style="width: 100%" v-loading="loading">
        <el-table-column prop="title" label="专利名称" min-width="200" />
        <el-table-column prop="patent_type" label="类型" width="100">
          <template #default="{ row }">
            <el-tag :type="getPatentTypeColor(row.patent_type)">
              {{ getPatentTypeLabel(row.patent_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="technical_field" label="技术领域" min-width="150" show-overflow-tooltip />
        <el-table-column prop="version" label="版本" width="60" />
        <el-table-column prop="updated_at" label="更新时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.updated_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="280" fixed="right">
          <template #default="{ row }">
            <el-button
              type="primary"
              size="small"
              @click="handleView(row)"
            >
              详情
            </el-button>
            <el-button
              type="success"
              size="small"
              @click="handleGenerate(row)"
            >
              <el-icon><MagicStick /></el-icon>
              AI 生成
            </el-button>
            <el-button
              type="primary"
              size="small"
              @click="handleEdit(row)"
            >
              编辑
            </el-button>
            <el-button
              type="danger"
              size="small"
              @click="handleDelete(row)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 空状态 -->
      <el-empty v-if="!loading && patents.length === 0" description="暂无专利文档，点击右上角创建" />
    </el-card>

    <!-- 创建专利对话框 -->
    <el-dialog
      v-model="showCreateDialog"
      title="新建专利文档"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="createFormRef"
        :model="createForm"
        :rules="createRules"
        label-width="100px"
      >
        <el-form-item label="专利名称" prop="title">
          <el-input
            v-model="createForm.title"
            placeholder="请输入专利名称"
          />
        </el-form-item>
        <el-form-item label="专利类型" prop="patent_type">
          <el-radio-group v-model="createForm.patent_type">
            <el-radio label="invention">发明专利</el-radio>
            <el-radio label="utility">实用新型</el-radio>
            <el-radio label="design">外观设计</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="所属项目" prop="project_id">
          <el-select v-model="createForm.project_id" placeholder="请选择所属项目" style="width: 100%">
            <el-option
              v-for="project in projects"
              :key="project.id"
              :label="project.name"
              :value="project.id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="技术领域" prop="technical_field">
          <el-input
            v-model="createForm.technical_field"
            type="textarea"
            :rows="2"
            placeholder="请简要描述所属技术领域"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">取消</el-button>
        <el-button type="primary" :loading="creating" @click="handleCreate">创建</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { Plus, MagicStick } from '@element-plus/icons-vue'
import { projectApi, type Project } from '@/api/project'

// 专利类型和状态类型
type PatentType = 'invention' | 'utility' | 'design'
type DocumentStatus = 'draft' | 'reviewing' | 'approved' | 'rejected'

interface PatentDocument {
  id: string
  project_id: string
  patent_type: PatentType
  title: string
  technical_field?: string
  background_art?: string
  invention_content?: string
  claims: Record<string, any>
  abstract_text?: string
  drawings_description?: string
  embodiment?: string
  ai_prompt?: string
  ai_model?: string
  version: number
  status: DocumentStatus
  review_comments?: string
  reviewed_by?: string
  reviewed_at?: string
  created_at: string
  updated_at: string
}

interface CreatePatentParams {
  project_id: string
  patent_type: PatentType
  title: string
  technical_field: string
}

const router = useRouter()

const loading = ref(false)
const creating = ref(false)
const patents = ref<PatentDocument[]>([])
const projects = ref<Project[]>([])
const showCreateDialog = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const filterPatentType = ref<string>('')
const filterStatus = ref<string>('')

// 创建表单
const createFormRef = ref<FormInstance>()
const createForm = ref<CreatePatentParams>({
  project_id: '',
  patent_type: 'invention',
  title: '',
  technical_field: '',
})

const createRules: FormRules = {
  title: [{ required: true, message: '请输入专利名称', trigger: 'blur' }],
  patent_type: [{ required: true, message: '请选择专利类型', trigger: 'change' }],
  project_id: [{ required: true, message: '请选择所属项目', trigger: 'change' }],
  technical_field: [{ required: true, message: '请输入技术领域', trigger: 'blur' }],
}

// 筛选后的专利列表
const filteredPatents = computed(() => {
  return patents.value.filter((patent) => {
    const matchKeyword = !searchKeyword.value ||
      patent.title.toLowerCase().includes(searchKeyword.value.toLowerCase()) ||
      (patent.technical_field && patent.technical_field.toLowerCase().includes(searchKeyword.value.toLowerCase()))

    const matchType = !filterPatentType.value || patent.patent_type === filterPatentType.value
    const matchStatus = !filterStatus.value || patent.status === filterStatus.value

    return matchKeyword && matchType && matchStatus
  })
})

// 获取专利列表
const fetchPatents = async () => {
  loading.value = true
  try {
    // TODO: 调用后端 API 获取专利列表
    // const response = await patentApi.list()
    // if (response.data) {
    //   patents.value = response.data
    // }
    // 临时：从项目列表获取专利类型的项
    const response = await projectApi.list()
    if (response.data) {
      patents.value = response.data
        .filter(p => p.type === 'patent')
        .map(p => ({
          id: p.id,
          project_id: p.id,
          patent_type: 'invention' as PatentType,
          title: p.name,
          technical_field: p.description || '',
          status: p.status as DocumentStatus,
          version: 1,
          created_at: p.created_at,
          updated_at: p.updated_at,
          claims: {},
        }))
    }
  } catch (error) {
    console.error('Failed to fetch patents:', error)
  } finally {
    loading.value = false
  }
}

// 获取项目列表（用于下拉框）
const fetchProjects = async () => {
  try {
    const response = await projectApi.list()
    if (response.data) {
      projects.value = response.data
    }
  } catch (error) {
    console.error('Failed to fetch projects:', error)
  }
}

// 搜索
const handleSearch = () => {
  ElMessage.success('搜索完成')
}

// 重置
const handleReset = () => {
  searchKeyword.value = ''
  filterPatentType.value = ''
  filterStatus.value = ''
  ElMessage.success('已重置筛选条件')
}

// 查看详情
const handleView = (patent: PatentDocument) => {
  router.push(`/patents/${patent.id}`)
}

// AI 生成
const handleGenerate = (patent: PatentDocument) => {
  router.push(`/patents/${patent.id}/generate`)
}

// 编辑
const handleEdit = (patent: PatentDocument) => {
  router.push(`/patents/${patent.id}/edit`)
}

// 删除
const handleDelete = (patent: PatentDocument) => {
  ElMessageBox.confirm(
    `确定要删除专利"${patent.title}"吗？`,
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(async () => {
    try {
      // TODO: 调用删除 API
      // await patentApi.delete(patent.id)
      ElMessage.success('删除成功')
      fetchPatents()
    } catch (error) {
      console.error('Failed to delete patent:', error)
    }
  }).catch(() => {})
}

// 创建专利
const handleCreate = async () => {
  if (!createFormRef.value) return

  await createFormRef.value.validate(async (valid) => {
    if (!valid) return

    creating.value = true
    try {
      // TODO: 调用创建专利 API
      // await patentApi.create(createForm.value)
      ElMessage.success('创建成功')
      showCreateDialog.value = false
      fetchPatents()
      // 重置表单
      createForm.value = {
        project_id: '',
        patent_type: 'invention',
        title: '',
        technical_field: '',
      }
    } catch (error) {
      console.error('Failed to create patent:', error)
    } finally {
      creating.value = false
    }
  })
}

// 专利类型颜色映射
const getPatentTypeColor = (type: string) => {
  const colors: Record<string, any> = {
    invention: 'primary',
    utility: 'success',
    design: 'warning',
  }
  return colors[type] || 'info'
}

// 专利类型文本映射
const getPatentTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    invention: '发明专利',
    utility: '实用新型',
    design: '外观设计',
  }
  return labels[type] || type
}

// 状态样式映射
const getStatusType = (status: string) => {
  const types: Record<string, any> = {
    draft: 'info',
    reviewing: 'warning',
    approved: 'success',
    rejected: 'danger',
  }
  return types[status] || 'info'
}

// 状态文本映射
const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    draft: '草稿',
    reviewing: '审核中',
    approved: '已通过',
    rejected: '已拒绝',
  }
  return labels[status] || status
}

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

onMounted(() => {
  fetchPatents()
  fetchProjects()
})
</script>

<style scoped>
.patents-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.page-header h2 {
  margin: 0;
}

.filter-card {
  margin-bottom: 20px;
}

.table-card {
  min-height: 400px;
}
</style>
