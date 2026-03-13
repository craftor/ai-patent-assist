import { http } from './http'

export interface LoginParams {
  username: string
  password: string
}

export interface User {
  id: string
  username: string
  email: string
  full_name?: string
  avatar_url?: string
  role_id?: string
}

export interface LoginResponse {
  access_token: string
  refresh_token?: string
  user: User
}

export const authApi = {
  login: (data: LoginParams) => http.post<LoginResponse>('/auth/login', data),

  logout: () => http.post('/auth/logout'),

  // TODO: 实现刷新 token 端点
  // refreshToken: (refreshToken: string) =>
  //   http.post<LoginResponse>('/auth/refresh', { refresh_token: refreshToken }),

  getCurrentUser: () => http.get<User>('/auth/me'),

  register: (data: { username: string; email: string; password: string; full_name?: string }) =>
    http.post<User>('/auth/register', data),
}
