<template>
  <div class="patent-detail-page">
    <el-page-header @back="handleBack" title="返回专利列表">
      <template #content>
        <span class="page-title">{{ patent.title || '专利详情' }}</span>
      </template>
      <template #extra>
        <el-tag :type="getStatusType(patent.status)" size="large">
          {{ getStatusLabel(patent.status) }}
        </el-tag>
      </template>
    </el-page-header>

    <div class="content-container" v-loading="loading">
      <el-card v-if="patent.id">
        <!-- 基本信息 -->
        <div class="section">
          <h3 class="section-title">
            <el-icon><Document /></el-icon>
            基本信息
          </h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="专利名称">{{ patent.title }}</el-descriptions-item>
            <el-descriptions-item label="专利类型">
              <el-tag :type="getPatentTypeColor(patent.patent_type)">
                {{ getPatentTypeLabel(patent.patent_type) }}
              </el-tag>
            </el-descriptions-item>
            <el-descriptions-item label="技术领域">{{ patent.technical_field || '-' }}</el-descriptions-item>
            <el-descriptions-item label="文档版本">V{{ patent.version }}</el-descriptions-item>
            <el-descriptions-item label="创建时间">{{ formatDate(patent.created_at) }}</el-descriptions-item>
            <el-descriptions-item label="更新时间">{{ formatDate(patent.updated_at) }}</el-descriptions-item>
          </el-descriptions>
        </div>

        <!-- 背景技术 -->
        <div class="section" v-if="patent.background_art">
          <h3 class="section-title">
            <el-icon><Clock /></el-icon>
            背景技术
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ patent.background_art }}</p>
          </el-card>
        </div>

        <!-- 发明内容 -->
        <div class="section" v-if="patent.invention_content">
          <h3 class="section-title">
            <el-icon><Aim /></el-icon>
            发明内容
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ patent.invention_content }}</p>
          </el-card>
        </div>

        <!-- 具体实施方式 -->
        <div class="section" v-if="patent.embodiment">
          <h3 class="section-title">
            <el-icon><Setting /></el-icon>
            具体实施方式
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ patent.embodiment }}</p>
          </el-card>
        </div>

        <!-- 权利要求书 -->
        <div class="section" v-if="patent.claims && Object.keys(patent.claims).length > 0">
          <h3 class="section-title">
            <el-icon><List /></el-icon>
            权利要求书
          </h3>
          <el-card shadow="never" class="content-card">
            <div v-for="(claim, key) in patent.claims" :key="key" class="claim-item">
              <strong>{{ key }}：</strong>
              <span>{{ claim }}</span>
            </div>
          </el-card>
        </div>

        <!-- 摘要 -->
        <div class="section" v-if="patent.abstract_text">
          <h3 class="section-title">
            <el-icon><Document /></el-icon>
            摘要
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ patent.abstract_text }}</p>
          </el-card>
        </div>

        <!-- 附图说明 -->
        <div class="section" v-if="patent.drawings_description">
          <h3 class="section-title">
            <el-icon><Picture /></el-icon>
            附图说明
          </h3>
          <el-card shadow="never" class="content-card">
            <p class="content-text">{{ patent.drawings_description }}</p>
          </el-card>
        </div>

        <!-- 审核信息 -->
        <div class="section" v-if="patent.status !== 'draft'">
          <h3 class="section-title">
            <el-icon><User /></el-icon>
            审核信息
          </h3>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="审核状态">
              <el-tag :type="getStatusType(patent.status)">
                {{ getStatusLabel(patent.status) }}
              </el-tag>
            </el-descriptions-item>
            <el-descriptions-item label="审核意见">{{ patent.review_comments || '-' }}</el-descriptions-item>
            <el-descriptions-item label="审核人">{{ patent.reviewed_by || '-' }}</el-descriptions-item>
            <el-descriptions-item label="审核时间">{{ patent.reviewed_at ? formatDate(patent.reviewed_at) : '-' }}</el-descriptions-item>
          </el-descriptions>
        </div>

        <!-- 操作按钮 -->
        <div class="action-buttons">
          <el-button @click="handleBack">返回</el-button>
          <el-button type="primary" @click="handleEdit">编辑</el-button>
          <el-button type="success" @click="handleGenerate">AI 生成</el-button>
        </div>
      </el-card>

      <el-empty v-else description="未找到专利文档" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import {
  Document,
  Clock,
  Aim,
  Setting,
  List,
  Picture,
  User,
} from '@element-plus/icons-vue'
import { patentApi, type PatentDocument } from '@/api/patent'

const router = useRouter()
const route = useRoute()

const loading = ref(false)
const patent = ref<PatentDocument>({
  id: '',
  project_id: '',
  patent_type: 'invention',
  title: '',
  technical_field: '',
  background_art: '',
  invention_content: '',
  claims: {},
  abstract_text: '',
  drawings_description: '',
  embodiment: '',
  version: 1,
  status: 'draft',
  created_at: '',
  updated_at: '',
})

// 获取专利详情
const fetchPatentDetail = async () => {
  loading.value = true
  try {
    const response = await patentApi.get(route.params.id as string)
    if (response.data) {
      patent.value = response.data
    }
  } catch (error) {
    console.error('Failed to fetch patent detail:', error)
    ElMessage.error('加载失败')
  } finally {
    loading.value = false
  }
}

const handleBack = () => {
  router.back()
}

const handleEdit = () => {
  router.push(`/patents/${patent.value.id}/edit`)
}

const handleGenerate = () => {
  router.push(`/patents/${patent.value.id}/generate`)
}

const getPatentTypeColor = (type: string) => {
  const colors: Record<string, any> = {
    invention: 'primary',
    utility: 'success',
    design: 'warning',
  }
  return colors[type] || 'info'
}

const getPatentTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    invention: '发明专利',
    utility: '实用新型',
    design: '外观设计',
  }
  return labels[type] || type
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
  fetchPatentDetail()
})
</script>

<style scoped>
.patent-detail-page {
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

.claim-item {
  margin-bottom: 12px;
  line-height: 1.6;
}

.claim-item:last-child {
  margin-bottom: 0;
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
