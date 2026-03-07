import { http } from './http'
import type { User } from './auth'

export interface UpdateUserParams {
  email?: string
  full_name?: string
  avatar_url?: string
}

export interface ChangePasswordParams {
  current_password: string
  new_password: string
}

export const userApi = {
  // 获取用户列表
  list: () => http.get<User[]>('/users'),

  // 获取用户详情
  get: (id: string) => http.get<User>(`/users/${id}`),

  // 更新用户信息
  update: (id: string, data: UpdateUserParams) =>
    http.put<User>(`/users/${id}`, data),

  // 删除用户
  delete: (id: string) =>
    http.delete(`/users/${id}`),

  // 修改密码
  changePassword: (id: string, data: ChangePasswordParams) =>
    http.put(`/users/${id}/password`, data),
}
