<template>
  <div class="project-detail-page">
    <div v-if="loading" class="loading">
      <el-skeleton :rows="5" animated />
    </div>

    <div v-else-if="project" class="content">
      <!-- 头部操作区 -->
      <div class="page-header">
        <div class="breadcrumb">
          <el-button text @click="$router.push('/projects')">
            <el-icon><ArrowLeft /></el-icon>
            返回列表
          </el-button>
        </div>
        <div class="actions">
          <el-button :type="project.status === 'completed' ? 'success' : 'primary'" @click="handleComplete">
            {{ project.status === 'completed' ? '已完成' : '完成项目' }}
          </el-button>
          <el-button type="danger" @click="handleDelete">删除项目</el-button>
        </div>
      </div>

      <!-- 项目基本信息 -->
      <el-card class="info-card">
        <template #header>
          <div class="card-header">
            <span>项目信息</span>
            <el-button v-if="!isEditing" type="primary" size="small" @click="isEditing = true">编辑</el-button>
          </div>
        </template>

        <el-form v-if="isEditing" :model="editForm" label-width="100px">
          <el-form-item label="项目名称">
            <el-input v-model="editForm.name" />
          </el-form-item>
          <el-form-item label="项目类型">
            <el-tag :type="project.type === 'patent' ? 'primary' : 'success'">
              {{ project.type === 'patent' ? '专利' : '软著' }}
            </el-tag>
          </el-form-item>
          <el-form-item label="项目状态">
            <el-select v-model="editForm.status" style="width: 150px">
              <el-option label="草稿" value="draft" />
              <el-option label="进行中" value="in_progress" />
              <el-option label="审核中" value="review" />
              <el-option label="已完成" value="completed" />
              <el-option label="已归档" value="archived" />
            </el-select>
          </el-form-item>
          <el-form-item label="项目描述">
            <el-input
              v-model="editForm.description"
              type="textarea"
              :rows="3"
              placeholder="请输入项目描述"
            />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="handleSave">保存</el-button>
            <el-button @click="handleCancel">取消</el-button>
          </el-form-item>
        </el-form>

        <el-descriptions v-else :column="2" border>
          <el-descriptions-item label="项目名称">{{ project.name }}</el-descriptions-item>
          <el-descriptions-item label="项目类型">
            <el-tag :type="project.type === 'patent' ? 'primary' : 'success'">
              {{ project.type === 'patent' ? '专利' : '软著' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="项目状态">
            <el-tag :type="getStatusType(project.status)">
              {{ getStatusLabel(project.status) }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="创建时间">{{ formatDate(project.created_at) }}</el-descriptions-item>
          <el-descriptions-item label="更新时间">{{ formatDate(project.updated_at) }}</el-descriptions-item>
          <el-descriptions-item label="完成时间" v-if="project.completed_at">
            {{ formatDate(project.completed_at) }}
          </el-descriptions-item>
          <el-descriptions-item label="项目描述" :span="2">
            {{ project.description || '无' }}
          </el-descriptions-item>
        </el-descriptions>
      </el-card>

      <!-- 关联文档 -->
      <el-card class="documents-card" style="margin-top: 20px">
        <template #header>
          <div class="card-header">
            <span>关联文档</span>
            <el-button
              type="primary"
              size="small"
              @click="handleCreateDocument"
            >
              <el-icon><Plus /></el-icon>
              新建文档
            </el-button>
          </div>
        </template>

        <el-empty v-if="!projectDocuments.length" description="暂无关联文档" />
        <el-table v-else :data="projectDocuments" style="width: 100%">
          <el-table-column prop="name" label="文档名称" min-width="200" />
          <el-table-column prop="type" label="类型" width="100">
            <template #default="{ row }">
              <el-tag size="small" :type="row.type === 'patent' ? 'primary' : 'success'">
                {{ row.type === 'patent' ? '专利' : '软著' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="status" label="状态" width="100">
            <template #default="{ row }">
              <el-tag size="small" :type="getStatusType(row.status)">
                {{ getStatusLabel(row.status) }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="updated_at" label="更新时间" width="180">
            <template #default="{ row }">
              {{ formatDate(row.updated_at) }}
            </template>
          </el-table-column>
          <el-table-column label="操作" width="150" fixed="right">
            <template #default="{ row }">
              <el-button type="primary" size="small" @click="handleViewDocument(row)">
                查看
              </el-button>
              <el-button size="small" @click="handleEditDocument(row)">
                编辑
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-card>

      <!-- 附件管理 -->
      <el-card class="attachments-card" style="margin-top: 20px">
        <template #header>
          <div class="card-header">
            <span>项目附件</span>
            <el-upload
              :auto-upload="false"
              :on-change="handleFileChange"
              :show-file-list="false"
            >
              <el-button type="primary" size="small">
                <el-icon><Upload /></el-icon>
                上传附件
              </el-button>
            </el-upload>
          </div>
        </template>

        <el-empty v-if="!attachments.length" description="暂无附件" />
        <el-table v-else :data="attachments" style="width: 100%">
          <el-table-column prop="file_name" label="文件名" min-width="200" />
          <el-table-column prop="file_type" label="类型" width="100" />
          <el-table-column prop="file_size" label="大小" width="100">
            <template #default="{ row }">
              {{ formatFileSize(row.file_size) }}
            </template>
          </el-table-column>
          <el-table-column prop="created_at" label="上传时间" width="180">
            <template #default="{ row }">
              {{ formatDate(row.created_at) }}
            </template>
          </el-table-column>
          <el-table-column label="操作" width="100" fixed="right">
            <template #default="{ row }">
              <el-button type="danger" size="small" @click="handleDeleteAttachment(row)">
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-card>
    </div>

    <el-empty v-else description="项目不存在" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { ArrowLeft, Plus, Upload } from '@element-plus/icons-vue'
import { projectApi, type Project } from '@/api/project'

const route = useRoute()
const router = useRouter()

const loading = ref(true)
const project = ref<Project | null>(null)
const isEditing = ref(false)
const attachments = ref<any[]>([])
const projectDocuments = ref<any[]>([])

const editForm = ref({
  name: '',
  description: '',
  status: '',
})

const projectId = computed(() => route.params.id as string)

// 加载项目详情
const fetchProjectDetail = async () => {
  loading.value = true
  try {
    const response = await projectApi.get(projectId.value)
    if (response.data) {
      project.value = response.data
      editForm.value = {
        name: response.data.name,
        description: response.data.description || '',
        status: response.data.status,
      }
    }
  } catch (error) {
    console.error('Failed to fetch project detail:', error)
    ElMessage.error('加载项目详情失败')
  } finally {
    loading.value = false
  }
}

// 保存编辑
const handleSave = async () => {
  try {
    const updateData: any = {}
    if (editForm.value.name !== project.value?.name) {
      updateData.name = editForm.value.name
    }
    if (editForm.value.description !== project.value?.description) {
      updateData.description = editForm.value.description
    }
    if (editForm.value.status !== project.value?.status) {
      updateData.status = editForm.value.status
    }

    await projectApi.update(projectId.value, updateData)
    ElMessage.success('保存成功')
    isEditing.value = false
    fetchProjectDetail()
  } catch (error) {
    console.error('Failed to update project:', error)
    ElMessage.error('保存失败')
  }
}

// 取消编辑
const handleCancel = () => {
  isEditing.value = false
  if (project.value) {
    editForm.value = {
      name: project.value.name,
      description: project.value.description || '',
      status: project.value.status,
    }
  }
}

// 完成项目
const handleComplete = async () => {
  if (project.value?.status === 'completed') {
    ElMessage.success('项目已完成')
    return
  }

  try {
    await projectApi.update(projectId.value, { status: 'completed' })
    ElMessage.success('项目已完成')
    fetchProjectDetail()
  } catch (error) {
    console.error('Failed to complete project:', error)
  }
}

// 删除项目
const handleDelete = () => {
  ElMessageBox.confirm(
    `确定要删除项目"${project.value?.name}"吗？此操作不可恢复。`,
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(async () => {
    try {
      await projectApi.delete(projectId.value)
      ElMessage.success('删除成功')
      router.push('/projects')
    } catch (error) {
      console.error('Failed to delete project:', error)
    }
  }).catch(() => {})
}

// 创建文档
const handleCreateDocument = () => {
  const type = project.value?.type === 'patent' ? 'patent' : 'copyright'
  router.push(`/projects/${projectId.value}/documents/create?type=${type}`)
}

// 查看文档
const handleViewDocument = (doc: any) => {
  const type = doc.type === 'patent' ? 'patents' : 'copyrights'
  router.push(`/${type}/${doc.id}`)
}

// 编辑文档
const handleEditDocument = (doc: any) => {
  const type = doc.type === 'patent' ? 'patents' : 'copyrights'
  router.push(`/${type}/${doc.id}/edit`)
}

// 处理文件选择
const handleFileChange = (file: any) => {
  ElMessage.info('文件上传功能待实现')
  console.log('Selected file:', file)
}

// 删除附件
const handleDeleteAttachment = (attachment: any) => {
  ElMessageBox.confirm('确定要删除此附件吗？', '确认删除', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning',
  }).then(async () => {
    try {
      await projectApi.deleteAttachment(projectId.value, attachment.id)
      ElMessage.success('删除成功')
      fetchProjectDetail()
    } catch (error) {
      console.error('Failed to delete attachment:', error)
    }
  }).catch(() => {})
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

// 格式化文件大小
const formatFileSize = (bytes: number | null) => {
  if (!bytes) return '-'
  const size = bytes
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(2) + ' KB'
  return (size / (1024 * 1024)).toFixed(2) + ' MB'
}

onMounted(() => {
  fetchProjectDetail()
})
</script>

<style scoped>
.project-detail-page {
  padding: 20px;
}

.loading {
  padding: 20px;
}

.content {
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 10px;
}

.actions {
  display: flex;
  gap: 10px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-card,
.documents-card,
.attachments-card {
  margin-bottom: 20px;
}
</style>
