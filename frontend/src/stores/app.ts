import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ElMessageOptions } from 'element-plus'

export interface Notification {
  id: string
  type: 'success' | 'warning' | 'info' | 'error'
  message: string
  title?: string
  duration?: number
}

export interface LoadingConfig {
  text?: string
  fullscreen?: boolean
  background?: string
  lock?: boolean
  timeout?: number
}

export const useAppStore = defineStore('app', () => {
  // 全局 loading 状态
  const loading = ref(false)
  const loadingText = ref('')
  const loadingCount = ref(0)

  // 通知队列
  const notifications = ref<Notification[]>([])

  // 侧边栏折叠状态（桌面端）
  const sidebarCollapsed = ref(false)

  // 全局错误状态
  const globalError = ref<string | null>(null)

  /**
   * 显示全局 loading
   */
  const showLoading = (config?: LoadingConfig) => {
    loadingText.value = config?.text || '加载中...'
    loadingCount.value++
    loading.value = true
  }

  /**
   * 隐藏全局 loading
   */
  const hideLoading = () => {
    loadingCount.value = Math.max(0, loadingCount.value - 1)
    if (loadingCount.value === 0) {
      loading.value = false
      loadingText.value = ''
    }
  }

  /**
   * 强制隐藏 loading（不计 count）
   */
  const forceHideLoading = () => {
    loading.value = false
    loadingText.value = ''
    loadingCount.value = 0
  }

  /**
   * 显示通知
   */
  const showNotification = (notification: Omit<Notification, 'id'>) => {
    const id = `notif_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
    const newNotification: Notification = {
      id,
      duration: 3000,
      ...notification,
    }
    notifications.value.push(newNotification)

    // 自动移除
    if (newNotification.duration !== 0) {
      setTimeout(() => {
        removeNotification(id)
      }, newNotification.duration)
    }

    return id
  }

  /**
   * 移除通知
   */
  const removeNotification = (id: string) => {
    const index = notifications.value.findIndex((n) => n.id === id)
    if (index > -1) {
      notifications.value.splice(index, 1)
    }
  }

  /**
   * 清除所有通知
   */
  const clearNotifications = () => {
    notifications.value = []
  }

  /**
   * 快捷通知方法
   */
  const success = (message: string, options?: Omit<Notification, 'id' | 'type' | 'message'>) => {
    return showNotification({ type: 'success', message, ...options })
  }

  const warning = (message: string, options?: Omit<Notification, 'id' | 'type' | 'message'>) => {
    return showNotification({ type: 'warning', message, ...options })
  }

  const info = (message: string, options?: Omit<Notification, 'id' | 'type' | 'message'>) => {
    return showNotification({ type: 'info', message, ...options })
  }

  const error = (message: string, options?: Omit<Notification, 'id' | 'type' | 'message'>) => {
    return showNotification({ type: 'error', message, ...options })
  }

  /**
   * 设置全局错误
   */
  const setGlobalError = (error: string | null) => {
    globalError.value = error
  }

  /**
   * 切换侧边栏状态
   */
  const toggleSidebar = () => {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  /**
   * 设置侧边栏状态
   */
  const setSidebarCollapsed = (collapsed: boolean) => {
    sidebarCollapsed.value = collapsed
  }

  return {
    // 状态
    loading,
    loadingText,
    loadingCount,
    notifications,
    sidebarCollapsed,
    globalError,
    // 方法
    showLoading,
    hideLoading,
    forceHideLoading,
    showNotification,
    removeNotification,
    clearNotifications,
    success,
    warning,
    info,
    error,
    setGlobalError,
    toggleSidebar,
    setSidebarCollapsed,
  }
})
