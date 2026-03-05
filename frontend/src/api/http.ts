import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'

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
        return config
      },
      (error) => Promise.reject(error)
    )

    // 响应拦截器
    this.instance.interceptors.response.use(
      (response: AxiosResponse<ApiResponse<any>>) => {
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
        if (error.response?.status === 401) {
          localStorage.removeItem('access_token')
          window.location.href = '/login'
        } else if (error.response?.status === 500) {
          ElMessage.error('服务器错误')
        } else {
          ElMessage.error(error.message || '网络错误')
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
