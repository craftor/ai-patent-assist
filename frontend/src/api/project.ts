import { http } from './http'

export interface Project {
  id: string
  user_id: string
  name: string
  description?: string
  type: 'patent' | 'copyright'
  status: 'draft' | 'in_progress' | 'review' | 'completed' | 'archived'
  metadata: Record<string, any>
  created_at: string
  updated_at: string
  completed_at?: string
}

export interface CreateProjectParams {
  name: string
  description?: string
  project_type: 'patent' | 'copyright'
}

export interface UpdateProjectParams {
  name?: string
  description?: string
  status?: string
}

export const projectApi = {
  list: () => http.get<Project[]>('/projects'),

  get: (id: string) => http.get<Project>(`/projects/${id}`),

  create: (data: CreateProjectParams) => http.post<Project>('/projects', data),

  update: (id: string, data: UpdateProjectParams) =>
    http.put<Project>(`/projects/${id}`, data),

  delete: (id: string) => http.delete(`/projects/${id}`),

  uploadAttachment: (projectId: string, file: File) => {
    const formData = new FormData()
    formData.append('file', file)
    return http.upload(`/projects/${projectId}/attachments`, formData)
  },

  deleteAttachment: (projectId: string, fileId: string) =>
    http.delete(`/projects/${projectId}/attachments/${fileId}`),
}
