<template>
  <div class="copyright-detail-page">
    <el-page-header @back="handleBack" title="返回软著列表">
      <template #content>
        <span class="page-title">{{ copyright.software_name || '软著详情' }}</span>
      </template>
      <template #extra>
        <el-tag :type="getStatusType(copyright.status)" size="large">
          {{ getStatusLabel(copyright.status) }}
        </el-tag>
      </template>
    </el-page-header>

    <div class="content-container" v-loading="loading">
      <el-card v-if="copyright.id">
        <!-- 软件基本信息 -->
        <div class="section">
          <h3 class="section-title">
            <el-icon><Monitor /></el-icon>
            软件基本信息
          </h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="软件名称">{{ copyright.software_name }}</el-descriptions-item>
            <el-descriptions-item label="版本号">{{ copyright.software_version || '-' }}</el-descriptions-item>
            <el-descriptions-item label="开发者">{{ copyright.developer || '-' }}</el-descriptions-item>
            <el-descriptions-item label="软件分类">{{ copyright.software_category || '-' }}</el-descriptions-item>
            <el-descriptions-item label="完成日期">{{ copyright.completion_date || '-' }}</el-descriptions-item>
            <el-descriptions-item label="文档版本">V{{ copyright.version }}</el-descriptions-item>
            <el-descriptions-item label="创建时间">{{ formatDate(copyright.created_at) }}</el-descriptions-item>
            <el-descriptions-item label="更新时间">{{ formatDate(copyright.updated_at) }}</el-descriptions-item>
          </el-descriptions>
        </div>

        <!-- 运行环境 -->
        <div class="section" v-if="copyright.operating_system || copyright.programming_language">
          <h3 class="section-title">
            <el-icon><Setting /></el-icon>
            运行环境
          </h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="操作系统">{{ copyright.operating_system || '-' }}</el-descriptions-item>
            <el-descriptions-item label="编程语言">{{ copyright.programming_language || '-' }}</el-descriptions-item>
            <el-descriptions-item label="代码行数">{{ copyright.line_count ? `${copyright.line_count} 行` : '-' }}</el-descriptions-item>
          </el-descriptions>
        </div>

        <!-- 软件功能说明 -->
        <div class="section" v-if="copyright.function_features">
          <h3 class="section-title">
            <el-icon><List /></el-icon>
            软件功能说明
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ copyright.function_features }}</p>
          </el-card>
        </div>

        <!-- 技术特点 -->
        <div class="section" v-if="copyright.technical_features">
          <h3 class="section-title">
            <el-icon><Lightning /></el-icon>
            技术特点
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ copyright.technical_features }}</p>
          </el-card>
        </div>

        <!-- 软件描述 -->
        <div class="section" v-if="copyright.description">
          <h3 class="section-title">
            <el-icon><Document /></el-icon>
            软件描述
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ copyright.description }}</p>
          </el-card>
        </div>

        <!-- 源代码说明 -->
        <div class="section" v-if="copyright.source_code_path">
          <h3 class="section-title">
            <el-icon><Folder /></el-icon>
            源代码说明
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ copyright.source_code_path }}</p>
          </el-card>
        </div>

        <!-- 用户手册 -->
        <div class="section" v-if="copyright.user_manual_path">
          <h3 class="section-title">
            <el-icon><User /></el-icon>
            用户手册
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ copyright.user_manual_path }}</p>
          </el-card>
        </div>

        <!-- 审核信息 -->
        <div class="section" v-if="copyright.status !== 'draft'">
          <h3 class="section-title">
            <el-icon><User /></el-icon>
            审核信息
          </h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="审核状态">
              <el-tag :type="getStatusType(copyright.status)">
                {{ getStatusLabel(copyright.status) }}
              </el-tag>
            </el-descriptions-item>
            <el-descriptions-item label="审核意见">{{ copyright.review_comments || '-' }}</el-descriptions-item>
            <el-descriptions-item label="审核人">{{ copyright.reviewed_by || '-' }}</el-descriptions-item>
            <el-descriptions-item label="审核时间">{{ copyright.reviewed_at ? formatDate(copyright.reviewed_at) : '-' }}</el-descriptions-item>
          </el-descriptions>
        </div>

        <!-- 操作按钮 -->
        <div class="action-buttons">
          <el-button @click="handleBack">返回</el-button>
          <el-button type="primary" @click="handleEdit">编辑</el-button>
          <el-button type="success" @click="handleGenerate">AI 生成</el-button>
        </div>
      </el-card>

      <el-empty v-else description="未找到软著文档" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import {
  Monitor,
  Setting,
  List,
  Lightning,
  Document,
  Folder,
  User,
} from '@element-plus/icons-vue'
import { copyrightApi, type CopyrightDocument } from '@/api/copyright'

const router = useRouter()
const route = useRoute()

const loading = ref(false)
const copyright = ref<CopyrightDocument>({
  id: '',
  project_id: '',
  software_name: '',
  software_version: '',
  developer: '',
  completion_date: '',
  publication_date: '',
  software_category: '',
  operating_system: '',
  programming_language: '',
  line_count: 0,
  source_code_path: '',
  user_manual_path: '',
  description: '',
  function_features: '',
  technical_features: '',
  version: 1,
  status: 'draft',
  created_at: '',
  updated_at: '',
})

// 获取软著详情
const fetchCopyrightDetail = async () => {
  loading.value = true
  try {
    const response = await copyrightApi.get(route.params.id as string)
    if (response.data) {
      copyright.value = response.data
    }
  } catch (error) {
    console.error('Failed to fetch copyright detail:', error)
    ElMessage.error('加载失败')
  } finally {
    loading.value = false
  }
}

const handleBack = () => {
  router.back()
}

const handleEdit = () => {
  router.push(`/copyrights/${copyright.value.id}/edit`)
}

const handleGenerate = () => {
  router.push(`/copyrights/${copyright.value.id}/generate`)
}

const getStatusType = (status: string) => {
  const types: Record<string, any> = {
    draft: 'info',
    reviewing: 'warning',
    approved: 'success',
    rejected: 'danger',
  }
  return types[status] || 'info'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    draft: '草稿',
    reviewing: '审核中',
    approved: '已通过',
    rejected: '已拒绝',
  }
  return labels[status] || status
}

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
  fetchCopyrightDetail()
})
</script>

<style scoped>
.copyright-detail-page {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-title {
  font-size: 18px;
  font-weight: 500;
  margin-left: 10px;
}

.content-container {
  margin-top: 20px;
}

.section {
  margin-bottom: 30px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.section:last-of-type {
  border-bottom: none;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  color: var(--el-text-color-primary);
  margin-bottom: 16px;
}

.content-card {
  background: var(--el-fill-color-light);
}

.content-text {
  line-height: 1.8;
  white-space: pre-wrap;
  color: var(--el-text-color-regular);
}

.action-buttons {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid var(--el-border-color-lighter);
}
</style>
