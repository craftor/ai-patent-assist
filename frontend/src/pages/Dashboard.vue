<template>
  <div class="dashboard">
    <h2>仪表盘</h2>

    <el-row :gutter="20" style="margin-top: 20px">
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon patent">
              <el-icon :size="32"><Document /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.patentCount }}</div>
              <div class="stat-label">专利文档</div>
            </div>
          </div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon copyright">
              <el-icon :size="32"><Files /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.copyrightCount }}</div>
              <div class="stat-label">软著文档</div>
            </div>
          </div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon project">
              <el-icon :size="32"><Folder /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.projectCount }}</div>
              <div class="stat-label">进行中项目</div>
            </div>
          </div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon ai">
              <el-icon :size="32"><Cpu /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.aiUsageCount }}</div>
              <div class="stat-label">AI 调用次数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px">
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>最近项目</span>
          </template>
          <el-table :data="recentProjects" style="width: 100%">
            <el-table-column prop="name" label="项目名称" />
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
          </el-table>
        </el-card>
      </el-col>

      <el-col :span="12">
        <el-card>
          <template #header>
            <span>快速入口</span>
          </template>
          <div class="quick-actions">
            <el-button type="primary" @click="$router.push('/patents/create')">
              新建专利文档
            </el-button>
            <el-button type="success" @click="$router.push('/copyrights/create')">
              新建软著文档
            </el-button>
            <el-button type="warning" @click="$router.push('/projects')">
              项目管理
            </el-button>
            <el-button type="info" @click="$router.push('/templates')">
              模板管理
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Document, Files, Folder, Cpu } from '@element-plus/icons-vue'

const stats = ref({
  patentCount: 12,
  copyrightCount: 8,
  projectCount: 5,
  aiUsageCount: 156,
})

const recentProjects = ref([
  { name: '发明专利 - 智能控制系统', type: 'patent', status: 'in_progress' },
  { name: '软著 - 数据分析平台 V1.0', type: 'copyright', status: 'review' },
  { name: '实用新型 - 新型传感器结构', type: 'patent', status: 'draft' },
])

const getStatusType = (status: string) => {
  const types: Record<string, any> = {
    draft: 'info',
    in_progress: 'primary',
    review: 'warning',
    completed: 'success',
  }
  return types[status] || 'info'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    draft: '草稿',
    in_progress: '进行中',
    review: '审核中',
    completed: '已完成',
  }
  return labels[status] || status
}
</script>

<style scoped>
.dashboard h2 {
  margin-bottom: 20px;
}

.stat-card {
  height: 100px;
}

.stat-card :deep(.el-card__body) {
  padding: 16px;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 64px;
  height: 64px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.stat-icon.patent {
  background: linear-gradient(135deg, #667eea, #764ba2);
}

.stat-icon.copyright {
  background: linear-gradient(135deg, #f093fb, #f5576c);
}

.stat-icon.project {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
}

.stat-icon.ai {
  background: linear-gradient(135deg, #43e97b, #38f9d7);
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 28px;
  font-weight: bold;
  color: #333;
  line-height: 1;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-top: 8px;
}

.quick-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
</style>
