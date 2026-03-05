import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User } from '@/api/auth'
import { authApi } from '@/api/auth'

export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('access_token'))

  const isLoggedIn = computed(() => !!token.value)
  const username = computed(() => user.value?.username || '')
  const email = computed(() => user.value?.email || '')

  async function login(username: string, password: string) {
    const response = await authApi.login({ username, password })
    console.log('Login response:', response)
    if (response.success && response.data) {
      token.value = response.data.access_token
      user.value = response.data.user
      localStorage.setItem('access_token', response.data.access_token)
      console.log('Token saved:', response.data.access_token)
      return true
    }
    throw new Error(response.message || '登录失败')
  }

  async function logout() {
    try {
      await authApi.logout()
    } finally {
      token.value = null
      user.value = null
      localStorage.removeItem('access_token')
    }
  }

  async function fetchCurrentUser() {
    if (!token.value) return

    try {
      const response = await authApi.getCurrentUser()
      if (response.data) {
        user.value = response.data
      }
    } catch {
      logout()
    }
  }

  return {
    user,
    token,
    isLoggedIn,
    username,
    email,
    login,
    logout,
    fetchCurrentUser,
  }
})
