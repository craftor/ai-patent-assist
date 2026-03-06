import { http } from './http'

export type TemplateType = 'patent_invention' | 'patent_utility' | 'patent_design' | 'copyright'

export interface Template {
  id: string
  name: string
  template_type: TemplateType
  content_template: string
  variables?: Record<string, any>
  description?: string
  is_system: boolean
  is_active: boolean
  created_by?: string
  created_at: string
  updated_at: string
}

export interface CreateTemplateParams {
  name: string
  template_type: string
  content_template: string
  variables?: Record<string, any>
  description?: string
  is_active?: boolean
}

export interface UpdateTemplateParams {
  name?: string
  template_type?: string
  content_template?: string
  variables?: Record<string, any>
  description?: string
  is_active?: boolean
}

export const templateApi = {
  list: () => http.get<Template[]>('/templates'),

  get: (id: string) => http.get<Template>(`/templates/${id}`),

  create: (data: CreateTemplateParams) =>
    http.post<Template>('/templates', data),

  update: (id: string, data: UpdateTemplateParams) =>
    http.put<Template>(`/templates/${id}`, data),

  delete: (id: string) =>
    http.delete(`/templates/${id}`),
}
