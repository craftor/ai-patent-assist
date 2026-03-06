<template>
  <div class="copyrights-page">
    <div class="page-header">
      <h2>软件著作权文档</h2>
      <el-button type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>
        新建软著
      </el-button>
    </div>

    <!-- 搜索和筛选 -->
    <el-card class="filter-card">
      <el-form :inline="true">
        <el-form-item label="关键词">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索软件名称"
            clearable
            style="width: 200px"
          />
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

    <!-- 软著列表 -->
    <el-card class="table-card">
      <el-table :data="filteredCopyrights" style="width: 100%" v-loading="loading">
        <el-table-column prop="software_name" label="软件名称" min-width="200" />
        <el-table-column prop="software_version" label="版本" width="100" />
        <el-table-column prop="developer" label="开发者" width="150" show-overflow-tooltip />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="software_category" label="软件分类" width="150" show-overflow-tooltip />
        <el-table-column prop="version" label="文档版本" width="80" />
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
      <el-empty v-if="!loading && copyrights.length === 0" description="暂无软著文档，点击右上角创建" />
    </el-card>

    <!-- 创建软著对话框 -->
    <el-dialog
      v-model="showCreateDialog"
      title="新建软件著作权文档"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="createFormRef"
        :model="createForm"
        :rules="createRules"
        label-width="120px"
      >
        <el-form-item label="软件名称" prop="software_name">
          <el-input
            v-model="createForm.software_name"
            placeholder="请输入软件名称，如：智能图像识别系统 V1.0"
          />
        </el-form-item>
        <el-form-item label="软件版本" prop="software_version">
          <el-input
            v-model="createForm.software_version"
            placeholder="请输入版本号，如：V1.0.0"
          />
        </el-form-item>
        <el-form-item label="开发者" prop="developer">
          <el-input
            v-model="createForm.developer"
            placeholder="请输入开发者姓名或公司名称"
          />
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
        <el-form-item label="软件功能简述" prop="description">
          <el-input
            v-model="createForm.description"
            type="textarea"
            :rows="3"
            placeholder="请简要描述软件的主要功能"
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
import { copyrightApi, type CopyrightDocument } from '@/api/copyright'

type DocumentStatus = 'draft' | 'reviewing' | 'approved' | 'rejected'

interface CreateCopyrightParams {
  project_id: string
  software_name: string
  software_version?: string
  developer: string
  description: string
}

const router = useRouter()

const loading = ref(false)
const creating = ref(false)
const copyrights = ref<CopyrightDocument[]>([])
const projects = ref<Project[]>([])
const showCreateDialog = ref(false)

// 搜索和筛选
const searchKeyword = ref('')
const filterStatus = ref<string>('')

// 创建表单
const createFormRef = ref<FormInstance>()
const createForm = ref<CreateCopyrightParams>({
  project_id: '',
  software_name: '',
  software_version: '',
  developer: '',
  description: '',
})

const createRules: FormRules = {
  software_name: [{ required: true, message: '请输入软件名称', trigger: 'blur' }],
  developer: [{ required: true, message: '请输入开发者', trigger: 'blur' }],
  project_id: [{ required: true, message: '请选择所属项目', trigger: 'change' }],
  description: [{ required: true, message: '请输入软件功能简述', trigger: 'blur' }],
}

// 筛选后的软著列表
const filteredCopyrights = computed(() => {
  return copyrights.value.filter((copyright) => {
    const matchKeyword = !searchKeyword.value ||
      copyright.software_name.toLowerCase().includes(searchKeyword.value.toLowerCase()) ||
      (copyright.developer && copyright.developer.toLowerCase().includes(searchKeyword.value.toLowerCase()))

    const matchStatus = !filterStatus.value || copyright.status === filterStatus.value

    return matchKeyword && matchStatus
  })
})

// 获取软著列表
const fetchCopyrights = async () => {
  loading.value = true
  try {
    // TODO: 调用后端 API 获取软著列表
    // const response = await copyrightApi.list()
    // if (response.data) {
    //   copyrights.value = response.data
    // }
    // 临时：从项目列表获取软著类型的项
    const response = await projectApi.list()
    if (response.data) {
      copyrights.value = response.data
        .filter(p => p.type === 'copyright')
        .map(p => ({
          id: p.id,
          project_id: p.id,
          software_name: p.name,
          software_version: 'V1.0',
          developer: '',
          description: p.description || '',
          status: p.status as DocumentStatus,
          version: 1,
          created_at: p.created_at,
          updated_at: p.updated_at,
        } as CopyrightDocument))
    }
  } catch (error) {
    console.error('Failed to fetch copyrights:', error)
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
  filterStatus.value = ''
  ElMessage.success('已重置筛选条件')
}

// 查看详情
const handleView = (copyright: CopyrightDocument) => {
  router.push(`/copyrights/${copyright.id}`)
}

// AI 生成
const handleGenerate = (copyright: CopyrightDocument) => {
  router.push(`/copyrights/${copyright.id}/generate`)
}

// 编辑
const handleEdit = (copyright: CopyrightDocument) => {
  router.push(`/copyrights/${copyright.id}/edit`)
}

// 删除
const handleDelete = (copyright: CopyrightDocument) => {
  ElMessageBox.confirm(
    `确定要删除软著"${copyright.software_name}"吗？`,
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(async () => {
    try {
      // TODO: 调用删除 API
      // await copyrightApi.delete(copyright.id)
      ElMessage.success('删除成功')
      fetchCopyrights()
    } catch (error) {
      console.error('Failed to delete copyright:', error)
    }
  }).catch(() => {})
}

// 创建软著
const handleCreate = async () => {
  if (!createFormRef.value) return

  await createFormRef.value.validate(async (valid) => {
    if (!valid) return

    creating.value = true
    try {
      // TODO: 调用创建软著 API
      // await copyrightApi.create(createForm.value)
      ElMessage.success('创建成功')
      showCreateDialog.value = false
      fetchCopyrights()
      // 重置表单
      createForm.value = {
        project_id: '',
        software_name: '',
        software_version: '',
        developer: '',
        description: '',
      }
    } catch (error) {
      console.error('Failed to create copyright:', error)
    } finally {
      creating.value = false
    }
  })
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
  fetchCopyrights()
  fetchProjects()
})
</script>

<style scoped>
.copyrights-page {
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
