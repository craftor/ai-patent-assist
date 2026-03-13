<template>
  <div class="settings-page">
    <el-page-header @back="handleBack" title="返回主页">
      <template #content>
        <span class="page-title">系统设置</span>
      </template>
    </el-page-header>

    <div class="settings-container">
      <el-tabs v-model="activeTab" type="border-card" class="settings-tabs">
        <!-- 个人中心 -->
        <el-tab-pane label="个人中心" name="profile">
          <div class="tab-content">
            <el-alert
              title="个人信息"
              description="管理您的个人资料和账户信息"
              type="info"
              show-icon
              style="margin-bottom: 20px"
            />

            <el-form :model="profileForm" label-width="120px" size="large">
              <el-form-item label="用户 ID">
                <el-input v-model="profileForm.id" disabled style="width: 300px" />
              </el-form-item>

              <el-form-item label="用户名">
                <el-input v-model="profileForm.username" disabled style="width: 300px" />
              </el-form-item>

              <el-form-item label="邮箱">
                <el-input
                  v-model="profileForm.email"
                  type="email"
                  placeholder="请输入邮箱"
                  style="width: 300px"
                />
              </el-form-item>

              <el-form-item label="全名">
                <el-input
                  v-model="profileForm.full_name"
                  placeholder="请输入您的全名"
                  style="width: 300px"
                />
              </el-form-item>

              <el-form-item>
                <el-button type="primary" @click="handleUpdateProfile" :loading="updatingProfile">
                  保存修改
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- 账户安全 -->
        <el-tab-pane label="账户安全" name="security">
          <div class="tab-content">
            <el-alert
              title="修改密码"
              description="定期修改密码可以提高账户安全性"
              type="warning"
              show-icon
              style="margin-bottom: 20px"
            />

            <el-form
              ref="passwordFormRef"
              :model="passwordForm"
              :rules="passwordRules"
              label-width="120px"
              size="large"
            >
              <el-form-item label="当前密码" prop="current_password">
                <el-input
                  v-model="passwordForm.current_password"
                  type="password"
                  placeholder="请输入当前密码"
                  show-password
                  style="width: 300px"
                />
              </el-form-item>

              <el-form-item label="新密码" prop="new_password">
                <el-input
                  v-model="passwordForm.new_password"
                  type="password"
                  placeholder="请输入新密码"
                  show-password
                  style="width: 300px"
                />
              </el-form-item>

              <el-form-item label="确认新密码" prop="confirm_password">
                <el-input
                  v-model="passwordForm.confirm_password"
                  type="password"
                  placeholder="请再次输入新密码"
                  show-password
                  style="width: 300px"
                />
              </el-form-item>

              <el-form-item>
                <el-button type="danger" @click="handleChangePassword" :loading="changingPassword">
                  修改密码
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- AI 模型配置 -->
        <el-tab-pane label="AI 模型" name="ai-model">
          <div class="tab-content">
            <el-alert
              title="AI 模型配置"
              description="配置用于专利文档生成的 AI 模型"
              type="success"
              show-icon
              style="margin-bottom: 20px"
            />

            <div class="model-config-section">
              <div class="section-header">
                <h3>当前模型配置</h3>
                <el-button type="primary" @click="showAddModelDialog = true">
                  <el-icon><Plus /></el-icon>
                  添加模型
                </el-button>
              </div>

              <el-table :data="modelList" style="width: 100%" stripe>
                <el-table-column prop="provider" label="提供商" width="150" />
                <el-table-column prop="model_name" label="模型名称" min-width="200" />
                <el-table-column prop="is_default" label="默认模型" width="100">
                  <template #default="{ row }">
                    <el-tag v-if="row.is_default" type="success" size="small">默认</el-tag>
                    <el-button
                      v-else
                      type="primary"
                      size="small"
                      link
                      @click="handleSetDefaultModel(row)"
                    >
                      设为默认
                    </el-button>
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="150" fixed="right">
                  <template #default="{ row }">
                    <el-button
                      type="danger"
                      size="small"
                      link
                      @click="handleDeleteModel(row)"
                      :disabled="row.is_default"
                    >
                      删除
                    </el-button>
                  </template>
                </el-table-column>
              </el-table>

              <el-empty v-if="modelList.length === 0" description="暂无 AI 模型配置" />
            </div>

            <!-- 添加模型对话框 -->
            <el-dialog
              v-model="showAddModelDialog"
              title="添加 AI 模型"
              width="500px"
              @close="handleAddModelDialogClose"
            >
              <el-form ref="addModelFormRef" :model="addModelForm" label-width="100px">
                <el-form-item label="提供商" required>
                  <el-select v-model="addModelForm.provider" placeholder="选择模型提供商" style="width: 100%">
                    <el-option label="Anthropic" value="anthropic" />
                    <el-option label="OpenAI" value="openai" />
                    <el-option label="DeepSeek" value="deepseek" />
                    <el-option label="智谱 AI" value="zhipu" />
                    <el-option label="Moonshot" value="moonshot" />
                    <el-option label="其他" value="other" />
                  </el-select>
                </el-form-item>

                <el-form-item label="模型名称" required>
                  <el-input
                    v-model="addModelForm.model_name"
                    placeholder="例如：claude-sonnet-4-5-20251001"
                  />
                </el-form-item>

                <el-form-item label="API Key" required>
                  <el-input
                    v-model="addModelForm.api_key"
                    type="password"
                    placeholder="请输入 API Key"
                    show-password
                  />
                </el-form-item>
              </el-form>

              <template #footer>
                <el-button @click="showAddModelDialog = false">取消</el-button>
                <el-button type="primary" @click="handleAddModel" :loading="addingModel">添加</el-button>
              </template>
            </el-dialog>
          </div>
        </el-tab-pane>

        <!-- 系统信息 -->
        <el-tab-pane label="系统信息" name="system">
          <div class="tab-content">
            <el-alert
              title="系统信息"
              description="查看系统版本和运行状态"
              type="info"
              show-icon
              style="margin-bottom: 20px"
            />

            <el-descriptions title="系统状态" :column="1" border>
              <el-descriptions-item label="系统名称">AI 专利助手</el-descriptions-item>
              <el-descriptions-item label="版本号">v1.0.0</el-descriptions-item>
              <el-descriptions-item label="后端状态">
                <el-tag v-if="backendHealth" type="success" size="small">运行中</el-tag>
                <el-tag v-else type="danger" size="small">未知</el-tag>
              </el-descriptions-item>
              <el-descriptions-item label="数据库">
                <el-tag type="success" size="small">PostgreSQL</el-tag>
              </el-descriptions-item>
              <el-descriptions-item label="前端框架">
                <el-tag type="info" size="small">Vue 3 + Element Plus</el-tag>
              </el-descriptions-item>
            </el-descriptions>

            <el-divider />

            <el-descriptions title="开发信息" :column="1" border>
              <el-descriptions-item label="技术栈">
                <el-tag size="small">Rust</el-tag>
                <el-tag size="small">Axum</el-tag>
                <el-tag size="small">SQLx</el-tag>
                <el-tag size="small">Vue 3</el-tag>
                <el-tag size="small">TypeScript</el-tag>
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { authApi } from '@/api/auth'
import { userApi } from '@/api/user'
import { http } from '@/api/http'

const router = useRouter()

const activeTab = ref('profile')
const updatingProfile = ref(false)
const changingPassword = ref(false)
const addingModel = ref(false)
const backendHealth = ref(false)
const showAddModelDialog = ref(false)

const profileForm = ref({
  id: '',
  username: '',
  email: '',
  full_name: '',
})

const passwordForm = reactive({
  current_password: '',
  new_password: '',
  confirm_password: '',
})

const passwordRules: FormRules = {
  current_password: [{ required: true, message: '请输入当前密码', trigger: 'blur' }],
  new_password: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度至少 6 位', trigger: 'blur' },
  ],
  confirm_password: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== passwordForm.new_password) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur',
    },
  ],
}

const addModelFormRef = ref<FormInstance>()
const addModelForm = reactive({
  provider: '',
  model_name: '',
  api_key: '',
})

interface AIModel {
  id: string
  provider: string
  model_name: string
  is_default: boolean
}

const modelList = ref<AIModel[]>([])

// 加载用户信息
const loadUserProfile = async () => {
  try {
    const response = await authApi.getCurrentUser()
    if (response.data) {
      const user = response.data
      profileForm.value = {
        id: user.id,
        username: user.username,
        email: user.email || '',
        full_name: user.full_name || '',
      }
    }
  } catch (error) {
    console.error('Failed to load user profile:', error)
  }
}

// 检查后端健康状态
const checkBackendHealth = async () => {
  try {
    const response = await http.getSilent('/health')
    backendHealth.value = true
  } catch (error) {
    backendHealth.value = false
  }
}

// 加载 AI 模型列表
const loadModelList = async () => {
  try {
    const response = await http.getSilent<AIModel[]>('/ai/models')
    if (response.data) {
      modelList.value = response.data
    }
  } catch (error) {
    console.error('Failed to load models:', error)
    // 使用模拟数据用于展示
    modelList.value = [
      { id: '1', provider: 'anthropic', model_name: 'claude-sonnet-4-5-20251001', is_default: true },
    ]
  }
}

// 更新个人信息
const handleUpdateProfile = async () => {
  updatingProfile.value = true
  try {
    await userApi.update(profileForm.value.id, {
      email: profileForm.value.email,
      full_name: profileForm.value.full_name,
    })
    ElMessage.success('个人信息更新成功')
  } catch (error) {
    console.error('Failed to update profile:', error)
    ElMessage.error('更新失败')
  } finally {
    updatingProfile.value = false
  }
}

// 修改密码
const handleChangePassword = async () => {
  if (!passwordForm.current_password || !passwordForm.new_password) {
    ElMessage.warning('请填写完整的密码信息')
    return
  }

  if (passwordForm.new_password !== passwordForm.confirm_password) {
    ElMessage.warning('两次输入的新密码不一致')
    return
  }

  try {
    await ElMessageBox.confirm('确定要修改密码吗？', '提示', {
      type: 'warning',
    })
  } catch {
    return
  }

  changingPassword.value = true
  try {
    await userApi.changePassword(profileForm.value.id, {
      current_password: passwordForm.current_password,
      new_password: passwordForm.new_password,
    })
    ElMessage.success('密码修改成功')
    passwordForm.current_password = ''
    passwordForm.new_password = ''
    passwordForm.confirm_password = ''
  } catch (error) {
    console.error('Failed to change password:', error)
    ElMessage.error('密码修改失败')
  } finally {
    changingPassword.value = false
  }
}

// 添加 AI 模型
const handleAddModel = async () => {
  if (!addModelFormRef.value) return

  await addModelFormRef.value.validate(async (valid) => {
    if (!valid) return

    if (!addModelForm.provider || !addModelForm.model_name || !addModelForm.api_key) {
      ElMessage.warning('请填写完整的模型信息')
      return
    }

    addingModel.value = true
    try {
      await http.post('/ai/models', addModelForm)
      ElMessage.success('模型添加成功')
      showAddModelDialog.value = false
      loadModelList()
    } catch (error) {
      console.error('Failed to add model:', error)
      ElMessage.error('添加失败')
    } finally {
      addingModel.value = false
    }
  })
}

// 设为默认模型
const handleSetDefaultModel = async (model: AIModel) => {
  try {
    await http.post(`/ai/models/${model.id}/set-default`)
    ElMessage.success('已设为默认模型')
    loadModelList()
  } catch (error) {
    console.error('Failed to set default model:', error)
    ElMessage.error('设置失败')
  }
}

// 删除模型
const handleDeleteModel = async (model: AIModel) => {
  if (model.is_default) {
    ElMessage.warning('不能删除默认模型')
    return
  }

  try {
    await ElMessageBox.confirm('确定要删除此模型吗？', '提示', {
      type: 'warning',
    })
  } catch {
    return
  }

  try {
    await http.delete(`/ai/models/${model.id}`)
    ElMessage.success('删除成功')
    loadModelList()
  } catch (error) {
    console.error('Failed to delete model:', error)
    ElMessage.error('删除失败')
  }
}

// 添加模型对话框关闭
const handleAddModelDialogClose = () => {
  addModelFormRef.value?.resetFields()
  addModelForm.provider = ''
  addModelForm.model_name = ''
  addModelForm.api_key = ''
}

// 返回
const handleBack = () => {
  router.back()
}

onMounted(() => {
  loadUserProfile()
  checkBackendHealth()
  loadModelList()
})
</script>

<style scoped>
.settings-page {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-title {
  font-size: 18px;
  font-weight: 500;
  margin-left: 10px;
}

.settings-container {
  margin-top: 20px;
}

.settings-tabs {
  min-height: 600px;
}

.tab-content {
  padding: 20px 10px;
}

.model-config-section {
  margin-top: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--el-text-color-primary);
}

:deep(.el-tabs__header) {
  margin-bottom: 0;
}

:deep(.el-tabs__content) {
  padding: 0;
}

:deep(.el-tab-pane) {
  padding: 20px;
}
</style>
