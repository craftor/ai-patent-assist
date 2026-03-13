import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage, ElNotification } from 'element-plus'

const baseURL = import.meta.env.VITE_API_BASE_URL || '/api'

export interface ApiResponse<T> {
  success: boolean
  message?: string
  data?: T
  error?: {
    code: string
    message: string
    details?: any
  }
}

// 请求缓存
const pendingRequests = new Map<string, any>()

// 生成请求 key
const generateRequestKey = (config: AxiosRequestConfig): string => {
  const { method, url, params, data } = config
  return `${method}_${url}_${JSON.stringify(params)}_${JSON.stringify(data)}`
}

// 是否跳过重复请求的 URL 列表
const skipDuplicateUrls = new Set<string>([
  // 这些 URL 允许并发请求
])

class HttpClient {
  private instance: AxiosInstance

  constructor() {
    this.instance = axios.create({
      baseURL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    // 请求拦截器
    this.instance.interceptors.request.use(
      (config) => {
        const token = localStorage.getItem('access_token')
        if (token) {
          config.headers.Authorization = `Bearer ${token}`
        }

        // 处理重复请求
        const requestKey = generateRequestKey(config)
        if (!skipDuplicateUrls.has(config.url || '')) {
          if (pendingRequests.has(requestKey)) {
            // 取消重复的请求
            const cancelSource = pendingRequests.get(requestKey)
            cancelSource.cancel('重复请求已取消')
          }
          // 创建新的取消令牌
          const CancelToken = axios.CancelToken
          const cancelSource = CancelToken.source()
          config.cancelToken = cancelSource.token
          pendingRequests.set(requestKey, cancelSource)
        }

        return config
      },
      (error) => Promise.reject(error)
    )

    // 响应拦截器
    this.instance.interceptors.response.use(
      (response: AxiosResponse<ApiResponse<any>>) => {
        // 从 pending 中移除
        const requestKey = generateRequestKey(response.config)
        pendingRequests.delete(requestKey)

        const { data } = response

        if (!data.success) {
          // 优先使用 error.message，如果没有则使用 message 字段
          const errorMsg = data.error?.message || data.message || '请求失败'
          ElMessage.error(errorMsg)
          return Promise.reject(new Error(errorMsg))
        }

        return response
      },
      (error) => {
        // 从 pending 中移除
        if (error.config) {
          const requestKey = generateRequestKey(error.config)
          pendingRequests.delete(requestKey)
        }

        // 取消请求不显示错误
        if (axios.isCancel(error)) {
          return Promise.reject(error)
        }

        if (error.response?.status === 401) {
          // 未授权，跳转到登录页
          localStorage.removeItem('access_token')
          // 避免循环跳转
          if (!window.location.pathname.startsWith('/login')) {
            window.location.href = '/login'
          }
        } else if (error.response?.status === 403) {
          ElMessage.error('没有权限访问该资源')
        } else if (error.response?.status === 404) {
          ElMessage.error('请求的资源不存在')
        } else if (error.response?.status === 500) {
          ElMessage.error('服务器错误')
        } else if (error.response?.status === 502) {
          ElMessage.error('网关错误，请稍后重试')
        } else if (error.response?.status === 503) {
          ElMessage.error('服务暂时不可用')
        } else if (error.message.includes('timeout')) {
          ElMessage.warning('请求超时，请检查网络连接')
        } else if (error.message.includes('Network Error')) {
          ElMessage.error('网络错误，请检查网络连接')
        } else if (!error.message.includes('已取消')) {
          ElMessage.error(error.message || '未知错误')
        }

        return Promise.reject(error)
      }
    )
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.instance.get(url, config)
    return response.data
  }

  async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.instance.post(url, data, config)
    return response.data
  }

  async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.instance.put(url, data, config)
    return response.data
  }

  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.instance.delete(url, config)
    return response.data
  }

  async upload<T>(url: string, formData: FormData): Promise<ApiResponse<T>> {
    const response = await this.instance.post(url, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
    return response.data
  }
}

export const http = new HttpClient()
