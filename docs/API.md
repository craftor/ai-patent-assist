# API 文档

AI Patent Assist 后端 API 接口文档。

**Base URL**: `/api`

## 目录

1. [认证接口](#认证接口)
2. [用户接口](#用户接口)
3. [项目接口](#项目接口)
4. [专利接口](#专利接口)
5. [软著接口](#软著接口)
6. [模板接口](#模板接口)
7. [AI 模型接口](#AI 模型接口)

---

## 认证接口

### 用户登录

**POST** `/auth/login`

用户登录获取访问令牌。

**请求体**
```json
{
  "username": "string",
  "password": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "access_token": "string",
    "user": {
      "id": "uuid",
      "username": "string",
      "email": "string",
      "role": "string",
      "created_at": "datetime"
    }
  }
}
```

**测试账号**
- 用户名：`admin`
- 密码：`admin123`

---

### 用户注册

**POST** `/auth/register`

注册新用户。

**请求体**
```json
{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "string",
      "email": "string"
    }
  }
}
```

---

### 获取当前用户

**GET** `/auth/me`

获取当前登录用户信息。

**请求头**
```
Authorization: Bearer <access_token>
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string",
    "role": "string",
    "created_at": "datetime"
  }
}
```

---

### 用户登出

**POST** `/auth/logout`

用户登出，使当前令牌失效。

**请求头**
```
Authorization: Bearer <access_token>
```

**响应** `200 OK`
```json
{
  "success": true,
  "message": "登出成功"
}
```

---

### 获取测试账号

**GET** `/auth/test-account`

获取系统测试账号信息。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "username": "admin",
    "password": "admin123",
    "description": "测试管理员账号"
  }
}
```

---

## 用户接口

### 获取用户列表

**GET** `/users`

获取所有用户列表（分页）。

**查询参数**
- `page` (integer): 页码，默认 1
- `page_size` (integer): 每页数量，默认 10

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "username": "string",
      "email": "string",
      "role": "string",
      "created_at": "datetime"
    }
  ]
}
```

---

### 获取用户详情

**GET** `/users/{id}`

获取指定用户详情。

**路径参数**
- `id` (uuid): 用户 ID

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string",
    "role": "string",
    "created_at": "datetime"
  }
}
```

---

### 更新用户

**PUT** `/users/{id}`

更新用户信息。

**请求体**
```json
{
  "email": "string",
  "role": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string"
  }
}
```

---

### 删除用户

**DELETE** `/users/{id}`

删除指定用户。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "用户已删除"
}
```

---

### 修改密码

**PUT** `/users/{id}/password`

修改用户密码。

**请求体**
```json
{
  "old_password": "string",
  "new_password": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "message": "密码已修改"
}
```

---

## 项目接口

### 获取项目列表

**GET** `/projects`

获取当前用户的项目列表。

**查询参数**
- `page` (integer): 页码
- `page_size` (integer): 每页数量
- `type` (string): 项目类型过滤 (patent/copyright)
- `status` (string): 状态过滤

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "type": "patent",
      "status": "draft",
      "description": "string",
      "created_at": "datetime",
      "updated_at": "datetime"
    }
  ]
}
```

---

### 创建项目

**POST** `/projects`

创建新项目。

**请求体**
```json
{
  "name": "string",
  "type": "patent",
  "description": "string"
}
```

**响应** `201 Created`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "type": "patent",
    "status": "draft"
  }
}
```

---

### 获取项目详情

**GET** `/projects/{id}`

获取指定项目详情。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "type": "patent",
    "status": "draft",
    "description": "string",
    "patents": [...],
    "copyrights": [...]
  }
}
```

---

### 更新项目

**PUT** `/projects/{id}`

更新项目信息。

**请求体**
```json
{
  "name": "string",
  "description": "string",
  "status": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "updated_at": "datetime"
  }
}
```

---

### 删除项目

**DELETE** `/projects/{id}`

删除项目及其关联文档。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "项目已删除"
}
```

---

### 上传附件

**POST** `/projects/{id}/attachments`

上传项目附件。

**Content-Type**: `multipart/form-data`

**请求体**
- `file`: 文件

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "file_id": "uuid",
    "file_name": "string",
    "file_size": 12345,
    "file_type": "application/pdf",
    "upload_url": "string"
  }
}
```

---

### 删除附件

**DELETE** `/projects/{id}/attachments/{file_id}`

删除项目附件。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "附件已删除"
}
```

---

## 专利接口

### 获取专利列表

**GET** `/patents`

获取专利文档列表。

**查询参数**
- `page` (integer): 页码
- `page_size` (integer): 每页数量
- `project_id` (uuid): 项目 ID 过滤
- `status` (string): 状态过滤

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "title": "string",
      "type": "invention",
      "status": "draft",
      "project_id": "uuid",
      "created_at": "datetime",
      "updated_at": "datetime"
    }
  ]
}
```

---

### 获取专利详情

**GET** `/patents/{id}`

获取专利文档详情。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "title": "string",
    "type": "invention",
    "status": "draft",
    "content": "string",
    "claims": [...],
    "abstract": "string",
    "attachments": [...]
  }
}
```

---

### 更新专利

**PUT** `/patents/{id}`

更新专利文档。

**请求体**
```json
{
  "title": "string",
  "content": "string",
  "claims": [...],
  "status": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "updated_at": "datetime"
  }
}
```

---

### AI 生成专利

**POST** `/patents/generate`

使用 AI 生成专利文档。

**请求体**
```json
{
  "project_id": "uuid",
  "invention_title": "string",
  "technical_field": "string",
  "background_art": "string",
  "invention_content": "string",
  "key_features": ["string"],
  "embodiments": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "patent_id": "uuid",
    "status": "generating",
    "message": "专利文档生成中"
  }
}
```

---

## 软著接口

### 获取软著列表

**GET** `/copyrights`

获取软著文档列表。

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "version": "string",
      "status": "draft",
      "project_id": "uuid",
      "created_at": "datetime"
    }
  ]
}
```

---

### 获取软著详情

**GET** `/copyrights/{id}`

获取软著文档详情。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "version": "string",
    "content": "string",
    "status": "draft"
  }
}
```

---

### 更新软著

**PUT** `/copyrights/{id}`

更新软著文档。

**请求体**
```json
{
  "name": "string",
  "version": "string",
  "content": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "updated_at": "datetime"
  }
}
```

---

### AI 生成软著

**POST** `/copyrights/generate`

使用 AI 生成软著文档。

**请求体**
```json
{
  "project_id": "uuid",
  "software_name": "string",
  "version": "string",
  "description": "string",
  "features": ["string"],
  "tech_stack": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "message": "软著文档生成中"
}
```

---

## 模板接口

### 获取模板列表

**GET** `/templates`

获取所有文档模板。

**查询参数**
- `type` (string): 模板类型 (patent/copyright)

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "type": "patent",
      "content": "string",
      "is_default": true
    }
  ]
}
```

---

### 创建模板

**POST** `/templates`

创建新模板。

**请求体**
```json
{
  "name": "string",
  "type": "patent",
  "content": "string",
  "description": "string"
}
```

**响应** `201 Created`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string"
  }
}
```

---

### 获取模板详情

**GET** `/templates/{id}`

获取指定模板详情。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "type": "patent",
    "content": "string",
    "description": "string"
  }
}
```

---

### 更新模板

**PUT** `/templates/{id}`

更新模板内容。

**请求体**
```json
{
  "name": "string",
  "content": "string",
  "description": "string"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "updated_at": "datetime"
  }
}
```

---

### 删除模板

**DELETE** `/templates/{id}`

删除模板。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "模板已删除"
}
```

---

## AI 模型接口

### 获取 AI 模型列表

**GET** `/ai/models`

获取所有 AI 模型配置。

**响应** `200 OK`
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "provider": "anthropic",
      "model_name": "claude-3-sonnet",
      "is_default": true,
      "status": "active"
    }
  ]
}
```

---

### 添加 AI 模型

**POST** `/ai/models`

添加新的 AI 模型配置。

**请求体**
```json
{
  "name": "string",
  "provider": "anthropic",
  "model_name": "claude-3-sonnet",
  "api_key": "string",
  "api_base_url": "string"
}
```

**响应** `201 Created`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string"
  }
}
```

---

### 更新 AI 模型

**PUT** `/ai/models/{id}`

更新 AI 模型配置。

**请求体**
```json
{
  "name": "string",
  "api_key": "string",
  "status": "active"
}
```

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "updated_at": "datetime"
  }
}
```

---

### 删除 AI 模型

**DELETE** `/ai/models/{id}`

删除 AI 模型配置。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "模型已删除"
}
```

---

### 设置默认模型

**POST** `/ai/models/{id}/set-default`

设置指定模型为默认模型。

**响应** `200 OK`
```json
{
  "success": true,
  "message": "默认模型已设置"
}
```

---

### 获取默认模型

**GET** `/ai/models/default`

获取当前默认 AI 模型。

**响应** `200 OK`
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "string",
    "provider": "anthropic",
    "model_name": "claude-3-sonnet"
  }
}
```

---

## 错误响应

所有接口错误时返回统一格式：

**4xx 客户端错误**
```json
{
  "success": false,
  "message": "错误描述",
  "error": {
    "code": "ERROR_CODE",
    "message": "详细错误信息",
    "details": {}
  }
}
```

**5xx 服务器错误**
```json
{
  "success": false,
  "message": "服务器错误",
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "内部服务器错误"
  }
}
```

### 常见错误码

| 错误码 | 说明 |
|--------|------|
| `UNAUTHORIZED` | 未授权访问 |
| `FORBIDDEN` | 禁止访问 |
| `NOT_FOUND` | 资源不存在 |
| `VALIDATION_ERROR` | 参数验证失败 |
| `DUPLICATE_ENTRY` | 重复数据 |
| `INTERNAL_ERROR` | 内部错误 |

---

## 认证说明

除登录接口外，所有接口都需要在请求头中携带 JWT Token：

```
Authorization: Bearer <access_token>
```

Token 有效期为 24 小时（可配置），过期后需要重新登录获取新 Token。
