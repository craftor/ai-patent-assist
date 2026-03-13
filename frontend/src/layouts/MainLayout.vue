<template>
  <div class="main-layout" :class="{ 'mobile-menu-open': isMobileMenuOpen }">
    <el-container>
      <!-- 侧边栏 -->
      <el-aside :width="isMobile ? '200px' : '220px'" class="sidebar" :class="{ 'sidebar-mobile': isMobile, 'sidebar-collapsed': !isMobileMenuOpen && isMobile }">
        <div class="logo">
          <h2>{{ isMobile ? '专利' : '专利助手' }}</h2>
          <el-icon v-if="isMobile" class="mobile-menu-toggle" @click="toggleMobileMenu">
            <Close v-if="isMobileMenuOpen" />
            <Menu v-else />
          </el-icon>
        </div>

        <el-menu
          :default-active="activeMenu"
          router
          background-color="#1a1a2e"
          text-color="#a0a0a0"
          active-text-color="#409EFF"
          :collapse="isMobile && !isMobileMenuOpen"
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
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  DataAnalysis,
  Folder,
  Document,
  Files,
  Setting,
  Tools,
  UserFilled,
  Close,
  Menu,
} from '@element-plus/icons-vue'
import { useUserStore } from '@/stores/user'
import { ElMessageBox } from 'element-plus'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const isMobile = ref(false)
const isMobileMenuOpen = ref(false)

// 检测移动端的函数
const checkMobile = () => {
  isMobile.value = window.innerWidth <= 768
}

// 窗口大小变化监听
const handleResize = () => {
  const wasMobile = isMobile.value
  checkMobile()
  // 从移动端切换到桌面端时，关闭移动菜单
  if (wasMobile && !isMobile.value) {
    isMobileMenuOpen.value = false
  }
}

const toggleMobileMenu = () => {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

// 生命周期钩子
onMounted(() => {
  checkMobile()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})

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
  transition: transform 0.3s ease;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid #2d2d44;
  position: relative;
}

.logo h2 {
  color: #fff;
  font-size: 18px;
  font-weight: 600;
}

.mobile-menu-toggle {
  position: absolute;
  right: 15px;
  cursor: pointer;
  color: #a0a0a0;
  font-size: 20px;
}

.mobile-menu-toggle:hover {
  color: #fff;
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

/* 响应式布局 - 移动端 */
@media screen and (max-width: 768px) {
  .main-layout {
    position: relative;
  }

  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    z-index: 1000;
    transform: translateX(-100%);
  }

  .sidebar.mobile-menu-open {
    transform: translateX(0);
  }

  .header {
    padding: 0 15px;
  }

  .username {
    display: none;
  }

  .content {
    padding: 10px;
  }
}

/* 平板响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .sidebar {
    width: 180px !important;
  }

  .el-menu-item span:not(.el-icon),
  .el-sub-menu__title span:not(.el-icon) {
    font-size: 13px;
  }
}

/* 小屏幕设备 */
@media screen and (max-width: 480px) {
  .logo h2 {
    font-size: 16px;
  }

  .stat-card {
    margin-bottom: 10px;
  }
}
</style>
