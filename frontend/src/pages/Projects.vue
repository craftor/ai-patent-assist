<template>
  <div class="projects-page">
    <div class="page-header">
      <h2>项目管理</h2>
      <el-button type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>
        新建项目
      </el-button>
    </div>

    <!-- 搜索和筛选 -->
    <el-card class="filter-card">
      <el-form :inline="true">
        <el-form-item label="关键词">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索项目名称"
            clearable
            style="width: 200px"
          />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="filterType" placeholder="全部" clearable style="width: 120px">
            <el-option label="专利" value="patent" />
            <el-option label="软著" value="copyright" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="filterStatus" placeholder="全部" clearable style="width: 120px">
            <el-option label="草稿" value="draft" />
            <el-option label="进行中" value="in_progress" />
            <el-option label="审核中" value="review" />
            <el-option label="已完成" value="completed" />
            <el-option label="已归档" value="archived" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 项目列表 -->
    <el-card class="table-card">
      <el-table :data="filteredProjects" style="width: 100%" v-loading="loading">
        <el-table-column prop="name" label="项目名称" min-width="200" />
        <el-table-column prop="type" label="类型" width="100">
          <template #default="{ row }">
            <el-tag :type="row.type === 'patent' ? 'primary' : 'success'">
              {{ row.type === 'patent' ? '专利' : '软著' }}
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
        <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
        <el-table-column prop="updated_at" label="更新时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.updated_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="220" fixed="right">
          <template #default="{ row }">
            <el-button
              type="primary"
              size="small"
              @click="handleView(row)"
            >
              详情
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
      <el-empty v-if="!loading && projects.length === 0" description="暂无项目，点击右上角创建" />
    </el-card>

    <!-- 创建项目对话框 -->
    <el-dialog
      v-model="showCreateDialog"
      title="新建项目"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="createFormRef"
        :model="createForm"
        :rules="createRules"
        label-width="80px"
      >
        <el-form-item label="项目名称" prop="name">
          <el-input
            v-model="createForm.name"
            placeholder="请输入项目名称"
          />
        </el-form-item>
        <el-form-item label="项目类型" prop="project_type">
          <el-radio-group v-model="createForm.project_type">
            <el-radio label="patent">专利</el-radio>
            <el-radio label="copyright">软著</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="项目描述" prop="description">
          <el-input
            v-model="createForm.description"
            type="textarea"
            :rows="3"
            placeholder="请输入项目描述（可选）"
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
import { Plus } from '@element-plus/icons-vue'
import { projectApi, type Project, type CreateProjectParams } from '@/api/project'

const router = useRouter()

const loading = ref(false)
const creating = ref(false)
const projects = ref<Project[]>([])
const showCreateDialog = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const filterType = ref<string>('')
const filterStatus = ref<string>('')

// 创建表单
const createFormRef = ref<FormInstance>()
const createForm = ref<CreateProjectParams>({
  name: '',
  description: '',
  project_type: 'patent',
})

const createRules: FormRules = {
  name: [{ required: true, message: '请输入项目名称', trigger: 'blur' }],
  project_type: [{ required: true, message: '请选择项目类型', trigger: 'change' }],
}

// 筛选后的项目列表
const filteredProjects = computed(() => {
  return projects.value.filter((project) => {
    const matchKeyword = !searchKeyword.value ||
      project.name.toLowerCase().includes(searchKeyword.value.toLowerCase()) ||
      (project.description && project.description.toLowerCase().includes(searchKeyword.value.toLowerCase()))

    const matchType = !filterType.value || project.type === filterType.value
    const matchStatus = !filterStatus.value || project.status === filterStatus.value

    return matchKeyword && matchType && matchStatus
  })
})

// 获取项目列表
const fetchProjects = async () => {
  loading.value = true
  try {
    const response = await projectApi.list()
    if (response.data) {
      projects.value = response.data
    }
  } catch (error) {
    console.error('Failed to fetch projects:', error)
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  // 筛选已经在 computed 中处理
  ElMessage.success('搜索完成')
}

// 重置
const handleReset = () => {
  searchKeyword.value = ''
  filterType.value = ''
  filterStatus.value = ''
  ElMessage.success('已重置筛选条件')
}

// 查看详情
const handleView = (project: Project) => {
  router.push(`/projects/${project.id}`)
}

// 编辑
const handleEdit = (project: Project) => {
  router.push(`/projects/${project.id}?action=edit`)
}

// 删除
const handleDelete = (project: Project) => {
  ElMessageBox.confirm(
    `确定要删除项目"${project.name}"吗？`,
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(async () => {
    try {
      await projectApi.delete(project.id)
      ElMessage.success('删除成功')
      fetchProjects()
    } catch (error) {
      console.error('Failed to delete project:', error)
    }
  }).catch(() => {})
}

// 创建项目
const handleCreate = async () => {
  if (!createFormRef.value) return

  await createFormRef.value.validate(async (valid) => {
    if (!valid) return

    creating.value = true
    try {
      await projectApi.create(createForm.value)
      ElMessage.success('创建成功')
      showCreateDialog.value = false
      fetchProjects()
      // 重置表单
      createForm.value = {
        name: '',
        description: '',
        project_type: 'patent',
      }
    } catch (error) {
      console.error('Failed to create project:', error)
    } finally {
      creating.value = false
    }
  })
}

// 状态样式映射
const getStatusType = (status: string) => {
  const types: Record<string, any> = {
    draft: 'info',
    in_progress: 'primary',
    review: 'warning',
    completed: 'success',
    archived: '',
  }
  return types[status] || 'info'
}

// 状态文本映射
const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    draft: '草稿',
    in_progress: '进行中',
    review: '审核中',
    completed: '已完成',
    archived: '已归档',
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
  fetchProjects()
})
</script>

<style scoped>
.projects-page {
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
