<template>
  <div class="patent-edit-page">
    <el-page-header @back="handleBack" title="返回专利详情">
      <template #content>
        <span class="page-title">编辑专利：{{ patent.title }}</span>
      </template>
    </el-page-header>

    <div class="content-container" v-loading="loading">
      <el-card v-if="patent.id">
        <el-form
          ref="formRef"
          :model="patent"
          :rules="rules"
          label-width="140px"
          label-position="top"
        >
          <!-- 基本信息 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Document /></el-icon>
              基本信息
            </h3>
            <el-row :gutter="20">
              <el-col :span="24">
                <el-form-item label="专利名称" prop="title">
                  <el-input v-model="patent.title" placeholder="请输入专利名称" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="专利类型" prop="patent_type">
                  <el-select v-model="patent.patent_type" style="width: 100%">
                    <el-option label="发明专利" value="invention" />
                    <el-option label="实用新型" value="utility" />
                    <el-option label="外观设计" value="design" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="技术领域" prop="technical_field">
                  <el-input v-model="patent.technical_field" placeholder="请输入技术领域" />
                </el-form-item>
              </el-col>
            </el-row>
          </div>

          <!-- 背景技术 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Clock /></el-icon>
              背景技术
            </h3>
            <el-form-item label="背景技术描述" prop="background_art">
              <el-input
                v-model="patent.background_art"
                type="textarea"
                :rows="6"
                placeholder="描述现有技术的不足和待解决的技术问题"
              />
            </el-form-item>
          </div>

          <!-- 发明内容 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Aim /></el-icon>
              发明内容
            </h3>
            <el-form-item label="发明内容描述" prop="invention_content">
              <el-input
                v-model="patent.invention_content"
                type="textarea"
                :rows="6"
                placeholder="描述本发明的技术方案和有益效果"
              />
            </el-form-item>
          </div>

          <!-- 权利要求书 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><List /></el-icon>
              权利要求书
            </h3>
            <el-form-item label="权利要求">
              <div class="claims-container">
                <div v-for="(claim, index) in claimsList" :key="index" class="claim-item">
                  <el-input
                    v-model="claimsList[index]"
                    type="textarea"
                    :rows="2"
                    placeholder="请输入权利要求"
                  >
                    <template #prepend>权{{ index + 1 }}.</template>
                  </el-input>
                  <el-button
                    type="danger"
                    :icon="Delete"
                    @click="removeClaim(index)"
                    :disabled="claimsList.length <= 1"
                  />
                </div>
                <el-button type="primary" :icon="Plus" @click="addClaim" style="margin-top: 10px">
                  添加权利要求
                </el-button>
              </div>
            </el-form-item>
          </div>

          <!-- 摘要 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Document /></el-icon>
              摘要
            </h3>
            <el-form-item label="摘要内容" prop="abstract_text">
              <el-input
                v-model="patent.abstract_text"
                type="textarea"
                :rows="4"
                placeholder="简要概括本发明的技术方案"
              />
            </el-form-item>
          </div>

          <!-- 附图说明 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Picture /></el-icon>
              附图说明
            </h3>
            <el-form-item label="附图说明" prop="drawings_description">
              <el-input
                v-model="patent.drawings_description"
                type="textarea"
                :rows="4"
                placeholder="描述附图内容和各图说明"
              />
            </el-form-item>
          </div>

          <!-- 具体实施方式 -->
          <div class="section">
            <h3 class="section-title">
              <el-icon><Setting /></el-icon>
              具体实施方式
            </h3>
            <el-form-item label="实施方式描述" prop="embodiment">
              <el-input
                v-model="patent.embodiment"
                type="textarea"
                :rows="6"
                placeholder="详细描述本发明的具体实施方式和实施例"
              />
            </el-form-item>
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

      <el-empty v-else description="未找到专利文档" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import {
  Document,
  Clock,
  Aim,
  List,
  Picture,
  Setting,
  Edit,
  Plus,
  Delete,
} from '@element-plus/icons-vue'
import { patentApi, type PatentDocument } from '@/api/patent'

const router = useRouter()
const route = useRoute()

const loading = ref(false)
const saving = ref(false)
const changeSummary = ref('')

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

// 表单验证规则
const rules: FormRules = {
  title: [{ required: true, message: '请输入专利名称', trigger: 'blur' }],
  patent_type: [{ required: true, message: '请选择专利类型', trigger: 'change' }],
  technical_field: [{ required: true, message: '请输入技术领域', trigger: 'blur' }],
}

// 权利要求列表
const claimsList = ref<string[]>([])

// 从对象转换为数组
const loadClaims = () => {
  if (!patent.value.claims || Object.keys(patent.value.claims).length === 0) {
    claimsList.value = ['']
  } else {
    claimsList.value = Object.values(patent.value.claims)
  }
}

// 添加权利要求
const addClaim = () => {
  claimsList.value.push('')
}

// 删除权利要求
const removeClaim = (index: number) => {
  if (claimsList.value.length > 1) {
    claimsList.value.splice(index, 1)
  }
}

// 获取专利详情
const fetchPatentDetail = async () => {
  loading.value = true
  try {
    const response = await patentApi.get(route.params.id as string)
    if (response.data) {
      patent.value = response.data
      loadClaims()
    }
  } catch (error) {
    console.error('Failed to fetch patent detail:', error)
    ElMessage.error('加载失败')
  } finally {
    loading.value = false
  }
}

// 保存修改
const handleSave = async () => {
  const formRef = formRef as any
  if (!formRef) return

  await formRef.validate(async (valid: boolean) => {
    if (!valid) return

    saving.value = true
    try {
      // 将数组转换为对象
      const claimsObject: Record<string, any> = {}
      claimsList.value.forEach((claim, index) => {
        if (claim && claim.trim()) {
          claimsObject[`权${index + 1}`] = claim
        }
      })

      const updateData = {
        title: patent.value.title,
        technical_field: patent.value.technical_field,
        background_art: patent.value.background_art,
        invention_content: patent.value.invention_content,
        claims: claimsObject,
        abstract_text: patent.value.abstract_text,
        drawings_description: patent.value.drawings_description,
        embodiment: patent.value.embodiment,
        change_summary: changeSummary.value || undefined,
      }

      await patentApi.update(route.params.id as string, updateData)
      ElMessage.success('保存成功')
      handleBack()
    } catch (error) {
      console.error('Failed to update patent:', error)
      ElMessage.error('保存失败')
    } finally {
      saving.value = false
    }
  })
}

const handleBack = () => {
  router.back()
}

const formRef = ref<FormInstance>()

onMounted(() => {
  fetchPatentDetail()
})
</script>

<style scoped>
.patent-edit-page {
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

.claims-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.claim-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  width: 100%;
}

.claim-item :deep(.el-input) {
  flex: 1;
  width: auto;
}

.claim-item :deep(.el-input__wrapper) {
  flex: 1;
}

.claim-item :deep(.el-input__inner) {
  resize: vertical;
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
