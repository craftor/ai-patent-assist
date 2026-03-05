<template>
  <div class="main-layout">
    <el-container>
      <!-- 侧边栏 -->
      <el-aside width="220px" class="sidebar">
        <div class="logo">
          <h2>专利助手</h2>
        </div>

        <el-menu
          :default-active="activeMenu"
          router
          background-color="#1a1a2e"
          text-color="#a0a0a0"
          active-text-color="#409EFF"
        >
          <el-menu-item index="/dashboard">
            <el-icon><DataAnalysis /></el-icon>
            <span>仪表盘</span>
          </el-menu-item>

          <el-menu-item index="/projects">
            <el-icon><Folder /></el-icon>
            <span>项目管理</span>
          </el-menu-item>

          <el-sub-menu index="patents">
            <template #title>
              <el-icon><Document /></el-icon>
              <span>专利文档</span>
            </template>
            <el-menu-item index="/patents">文档列表</el-menu-item>
            <el-menu-item index="/patents/create">新建专利</el-menu-item>
          </el-sub-menu>

          <el-sub-menu index="copyrights">
            <template #title>
              <el-icon><Files /></el-icon>
              <span>软著文档</span>
            </template>
            <el-menu-item index="/copyrights">文档列表</el-menu-item>
            <el-menu-item index="/copyrights/create">新建软著</el-menu-item>
          </el-sub-menu>

          <el-menu-item index="/templates">
            <el-icon><Setting /></el-icon>
            <span>模板管理</span>
          </el-menu-item>

          <el-menu-item index="/settings">
            <el-icon><Tools /></el-icon>
            <span>系统设置</span>
          </el-menu-item>
        </el-menu>
      </el-aside>

      <!-- 主内容区 -->
      <el-container>
        <!-- 顶部导航 -->
        <el-header class="header">
          <div class="header-left">
            <el-breadcrumb separator="/">
              <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
              <el-breadcrumb-item v-if="breadcrumb">{{ breadcrumb }}</el-breadcrumb-item>
            </el-breadcrumb>
          </div>

          <div class="header-right">
            <el-dropdown @command="handleCommand">
              <span class="user-info">
                <el-avatar :size="32" :icon="UserFilled" />
                <span class="username">{{ username }}</span>
              </span>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="profile">个人中心</el-dropdown-item>
                  <el-dropdown-item command="logout" divided>退出登录</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </el-header>

        <!-- 内容区 -->
        <el-main class="content">
          <router-view />
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  DataAnalysis,
  Folder,
  Document,
  Files,
  Setting,
  Tools,
  UserFilled,
} from '@element-plus/icons-vue'
import { useUserStore } from '@/stores/user'
import { ElMessageBox } from 'element-plus'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const activeMenu = computed(() => route.path)
const breadcrumb = computed(() => route.meta.title as string || '')
const username = computed(() => userStore.username || userStore.user?.username || '用户')

const handleCommand = async (command: string) => {
  if (command === 'logout') {
    await ElMessageBox.confirm('确认退出登录吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    })
    await userStore.logout()
    router.push('/login')
  } else if (command === 'profile') {
    router.push('/settings')
  }
}
</script>

<style scoped>
.main-layout {
  height: 100vh;
}

.el-container {
  height: 100%;
}

.sidebar {
  background-color: #1a1a2e;
  color: #fff;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid #2d2d44;
}

.logo h2 {
  color: #fff;
  font-size: 18px;
  font-weight: 600;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
  padding: 0 20px;
}

.header-right {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.username {
  font-size: 14px;
  color: #333;
}

.content {
  background-color: #f5f7fa;
  padding: 20px;
}
</style>
