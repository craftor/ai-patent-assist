<template>
  <div class="copyright-edit-page">
    <el-page-header @back="handleBack" title="返回软著详情">
      <template #content>
        <span class="page-title">编辑软著：{{ copyright.software_name }}</span>
      </template>
    </el-page-header>

    <div class="content-container" v-loading="loading">
      <el-card v-if="copyright.id">
        <el-form
          ref="formRef"
          :model="copyright"
          :rules="rules"
          label-width="140px"
          label-position="top"
        >
          <!-- 基本信息 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Monitor /></el-icon>
              软件基本信息
            </h3>
            <el-row :gutter="20">
              <el-col :span="24">
                <el-form-item label="软件名称" prop="software_name">
                  <el-input v-model="copyright.software_name" placeholder="请输入软件名称" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="版本号" prop="software_version">
                  <el-input v-model="copyright.software_version" placeholder="请输入版本号，如 V1.0" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="开发者" prop="developer">
                  <el-input v-model="copyright.developer" placeholder="请输入开发者姓名或公司名称" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="完成日期" prop="completion_date">
                  <el-date-picker
                    v-model="copyright.completion_date"
                    type="date"
                    placeholder="请选择完成日期"
                    style="width: 100%"
                    value-format="YYYY-MM-DD"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="软件分类" prop="software_category">
                  <el-input v-model="copyright.software_category" placeholder="请输入软件分类" />
                </el-form-item>
              </el-col>
            </el-row>
          </div>

          <!-- 运行环境 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Setting /></el-icon>
              运行环境
            </h3>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="操作系统" prop="operating_system">
                  <el-input v-model="copyright.operating_system" placeholder="如：Windows/Linux/macOS" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="编程语言" prop="programming_language">
                  <el-input v-model="copyright.programming_language" placeholder="如：Java/Python/JavaScript" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="代码行数" prop="line_count">
                  <el-input-number
                    v-model="copyright.line_count"
                    :min="0"
                    :step="100"
                    placeholder="请输入代码行数"
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
            </el-row>
          </div>

          <!-- 软件功能说明 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><List /></el-icon>
              软件功能说明
            </h3>
            <el-form-item label="功能描述" prop="description">
              <el-input
                v-model="copyright.description"
                type="textarea"
                :rows="6"
                placeholder="请详细描述软件的主要功能和应用场景"
              />
            </el-form-item>
          </div>

          <!-- 功能特点 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Lightning /></el-icon>
              功能特点
            </h3>
            <el-form-item label="功能特点" prop="function_features">
              <el-input
                v-model="copyright.function_features"
                type="textarea"
                :rows="6"
                placeholder="请描述软件的功能特点和技术亮点"
              />
            </el-form-item>
          </div>

          <!-- 技术特点 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Lightning /></el-icon>
              技术特点
            </h3>
            <el-form-item label="技术特点" prop="technical_features">
              <el-input
                v-model="copyright.technical_features"
                type="textarea"
                :rows="6"
                placeholder="请描述软件的技术特点和创新点"
              />
            </el-form-item>
          </div>

          <!-- 附加文件路径 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Folder /></el-icon>
              附加文件
            </h3>
            <el-row :gutter="20">
              <el-col :span="24">
                <el-form-item label="源代码文件路径" prop="source_code_path">
                  <el-input v-model="copyright.source_code_path" placeholder="请输入源代码文件的存储路径" />
                </el-form-item>
              </el-col>
              <el-col :span="24">
                <el-form-item label="用户手册路径" prop="user_manual_path">
                  <el-input v-model="copyright.user_manual_path" placeholder="请输入用户手册的存储路径" />
                </el-form-item>
              </el-col>
            </el-row>
          </div>

          <!-- 修改说明 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Edit /></el-icon>
              修改说明
            </h3>
            <el-form-item label="本次修改说明">
              <el-input
                v-model="changeSummary"
                type="textarea"
                :rows="3"
                placeholder="请简要说明本次修改的内容（可选）"
              />
            </el-form-item>
          </div>

          <!-- 操作按钮 -->
          <div class="action-buttons">
            <el-button @click="handleBack">取消</el-button>
            <el-button type="primary" :loading="saving" @click="handleSave">保存修改</el-button>
          </div>
        </el-form>
      </el-card>

      <el-empty v-else description="未找到软著文档" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import {
  Monitor,
  Setting,
  List,
  Lightning,
  Folder,
  Edit,
} from '@element-plus/icons-vue'
import { copyrightApi, type CopyrightDocument } from '@/api/copyright'

const router = useRouter()
const route = useRoute()

const loading = ref(false)
const saving = ref(false)
const changeSummary = ref('')
const formRef = ref<FormInstance>()

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

// 表单验证规则
const rules: FormRules = {
  software_name: [{ required: true, message: '请输入软件名称', trigger: 'blur' }],
}

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

// 保存修改
const handleSave = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid: boolean) => {
    if (!valid) return

    saving.value = true
    try {
      const updateData = {
        software_name: copyright.value.software_name,
        software_version: copyright.value.software_version,
        developer: copyright.value.developer,
        completion_date: copyright.value.completion_date,
        software_category: copyright.value.software_category,
        operating_system: copyright.value.operating_system,
        programming_language: copyright.value.programming_language,
        line_count: copyright.value.line_count,
        source_code_path: copyright.value.source_code_path,
        user_manual_path: copyright.value.user_manual_path,
        description: copyright.value.description,
        function_features: copyright.value.function_features,
        technical_features: copyright.value.technical_features,
      }

      await copyrightApi.update(route.params.id as string, updateData)
      ElMessage.success('保存成功')
      handleBack()
    } catch (error) {
      console.error('Failed to update copyright:', error)
      ElMessage.error('保存失败')
    } finally {
      saving.value = false
    }
  })
}

const handleBack = () => {
  router.back()
}

onMounted(() => {
  fetchCopyrightDetail()
})
</script>

<style scoped>
.copyright-edit-page {
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

.action-buttons {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-top: 30px;
  padding-top: 20px;
  border-top: 1px solid var(--el-border-color-lighter);
}
</style>
