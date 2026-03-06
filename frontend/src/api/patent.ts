import { http } from './http'

export type PatentType = 'invention' | 'utility' | 'design'
export type DocumentStatus = 'draft' | 'reviewing' | 'approved' | 'rejected'

export interface PatentDocument {
  id: string
  project_id: string
  patent_type: PatentType
  title: string
  technical_field?: string
  background_art?: string
  invention_content?: string
  claims: Record<string, any>
  abstract_text?: string
  drawings_description?: string
  embodiment?: string
  ai_prompt?: string
  ai_model?: string
  version: number
  status: DocumentStatus
  review_comments?: string
  reviewed_by?: string
  reviewed_at?: string
  created_at: string
  updated_at: string
}

export interface GeneratePatentParams {
  project_id: string
  patent_type: PatentType
  title: string
  technical_field: string
  background_art: string
  invention_description: string
  embodiments: string[]
  claims_input?: string
}

export interface UpdatePatentParams {
  title?: string
  technical_field?: string
  background_art?: string
  invention_content?: string
  claims?: Record<string, any>
  abstract_text?: string
  embodiment?: string
  change_summary?: string
}

export const patentApi = {
  list: () => http.get<PatentDocument[]>('/patents'),

  generate: (data: GeneratePatentParams) =>
    http.post<PatentDocument>('/patents/generate', data),

  get: (id: string) => http.get<PatentDocument>(`/patents/${id}`),

  update: (id: string, data: UpdatePatentParams) =>
    http.put<PatentDocument>(`/patents/${id}`, data),

  listVersions: (id: string) =>
    http.get<Array<any>>(`/patents/${id}/versions`),

  submitReview: (id: string, approved: boolean, comments: string) =>
    http.post<PatentDocument>(`/patents/${id}/review`, { approved, comments }),

  export: (id: string, format: string) =>
    http.post<{ download_url: string; file_name: string }>(`/patents/${id}/export`, { format }),
}
