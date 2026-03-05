<template>
  <div class="login-page">
    <el-card class="login-card">
      <h2 class="title">专利助手</h2>
      <p class="subtitle">智能专利说明书生成系统</p>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
      >
        <el-form-item label="用户名" prop="username">
          <el-input
            v-model="form.username"
            placeholder="请输入用户名"
            size="large"
          />
        </el-form-item>

        <el-form-item label="密码" prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            size="large"
            show-password
            @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="loading"
            @click="handleLogin"
            style="width: 100%"
          >
            登录
          </el-button>
        </el-form-item>

        <!-- 测试账号提示 -->
        <el-divider />
        <div class="test-account">
          <p class="test-account-title">测试账号</p>
          <el-alert
            title="测试账号信息"
            type="info"
            :closable="false"
            show-icon
          >
            <template #default>
              <div class="test-account-info">
                <p><strong>用户名：</strong>admin</p>
                <p><strong>密码：</strong>admin123</p>
              </div>
              <el-button
                type="info"
                size="small"
                @click="fillTestAccount"
                style="margin-top: 10px"
              >
                一键填充
              </el-button>
            </template>
          </el-alert>
        </div>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  username: '',
  password: '',
})

const rules: FormRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度至少 6 位', trigger: 'blur' },
  ],
}

const handleLogin = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid) => {
    if (!valid) return

    loading.value = true
    try {
      const success = await userStore.login(form.username, form.password)
      if (success) {
        ElMessage.success('登录成功')
        router.push('/dashboard')
      }
    } catch (error) {
      console.error('Login failed:', error)
    } finally {
      loading.value = false
    }
  })
}

// 填充测试账号
const fillTestAccount = () => {
  form.username = 'admin'
  form.password = 'admin123'
  ElMessage.success('已填充测试账号')
}
</script>

<style scoped>
.login-page {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
  padding: 20px;
}

.title {
  text-align: center;
  font-size: 24px;
  color: #333;
  margin-bottom: 8px;
}

.subtitle {
  text-align: center;
  color: #666;
  margin-bottom: 30px;
}

.test-account {
  width: 100%;
}

.test-account-title {
  text-align: center;
  font-size: 14px;
  color: #666;
  margin-bottom: 10px;
}

.test-account-info {
  font-size: 13px;
  color: #333;
}

.test-account-info p {
  margin: 5px 0;
}
</style>
