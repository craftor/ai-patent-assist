import { http } from './http'

export interface CopyrightDocument {
  id: string
  project_id: string
  software_name: string
  software_version?: string
  developer?: string
  completion_date?: string
  publication_date?: string
  software_category?: string
  operating_system?: string
  programming_language?: string
  line_count?: number
  source_code_path?: string
  user_manual_path?: string
  description?: string
  function_features?: string
  technical_features?: string
  ai_prompt?: string
  ai_model?: string
  version: number
  status: string
  review_comments?: string
  reviewed_by?: string
  reviewed_at?: string
  created_at: string
  updated_at: string
}

export interface GenerateCopyrightParams {
  project_id: string
  software_name: string
  software_version?: string
  developer: string
  description: string
  function_features: string
  technical_features: string
  source_code_summary?: string
}

export interface UpdateCopyrightParams {
  software_name?: string
  description?: string
  function_features?: string
  technical_features?: string
}

export const copyrightApi = {
  generate: (data: GenerateCopyrightParams) =>
    http.post<CopyrightDocument>('/copyrights/generate', data),

  get: (id: string) => http.get<CopyrightDocument>(`/copyrights/${id}`),

  update: (id: string, data: UpdateCopyrightParams) =>
    http.put<CopyrightDocument>(`/copyrights/${id}`, data),

  listVersions: (id: string) =>
    http.get<Array<any>>(`/copyrights/${id}/versions`),

  submitReview: (id: string, approved: boolean, comments: string) =>
    http.post<CopyrightDocument>(`/copyrights/${id}/review`, { approved, comments }),

  export: (id: string, format: string) =>
    http.post<{ download_url: string; file_name: string }>(`/copyrights/${id}/export`, { format }),
}
