# AI Patent Assist - 专利说明书自动生成系统

智能专利说明书和软件著作权说明书自动生成系统。

## 功能特性

- **专利说明书生成**: 支持发明专利、实用新型、外观设计三种类型
- **软著说明书生成**: 根据软件代码和使用说明自动生成软著文档
- **AI 智能生成**: 集成 Claude 等 AI 模型，智能生成专业文档
- **文档版本管理**: 完整的版本控制和历史记录
- **审核流程**: 支持文档审核和审批流程
- **模板管理**: 可自定义文档模板
- **JWT 认证**: 完整的用户认证和授权系统
- **Docker 一键部署**: 支持 Docker Compose 和 Kubernetes 部署
- **响应式设计**: 支持桌面、平板、移动端多种设备
- **通用组件库**: DataTable、FormDialog 等可复用组件

## 测试账号

| 用户名 | 密码 | 说明 |
|--------|------|------|
| admin | admin123 | 测试管理员账号 |

## 技术栈

### 前端
- Vue 3 + TypeScript
- Element Plus UI 组件库
- Pinia 状态管理
- Vue Router 路由
- Axios HTTP 客户端
- Vite 构建工具

### 后端
- Rust 1.75+
- Axum 0.8 Web 框架
- PostgreSQL 数据库
- SQLx 异步数据库访问
- JWT 认证
- Argon2 密码哈希

### 部署
- Docker + Docker Compose
- GitHub Actions CI/CD
- Nginx

## 快速开始

### 使用 Docker Compose（开发环境）

```bash
# 克隆项目
git clone https://github.com/your-org/ai-patent-assist.git
cd ai-patent-assist

# 配置环境变量
cp .env.example .env
# 编辑 .env 文件，填入实际配置

# 启动所有服务
docker-compose up -d

# 访问服务
# 前端：http://localhost:8080
# 后端 API：http://localhost:3000
# PostgreSQL: localhost:5432
# Redis: localhost:6379

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

### 使用 Docker Compose（生产环境）

```bash
# 配置生产环境变量
export POSTGRES_USER=your-user
export POSTGRES_PASSWORD=your-secure-password
export JWT_SECRET=your-jwt-secret
export ANTHROPIC_API_KEY=your-api-key

# 使用生产配置启动
docker-compose -f docker-compose.prod.yml up -d
```

### 使用 Kubernetes 部署

```bash
# 应用配置
kubectl apply -f deploy/kubernetes/namespace.yaml
kubectl apply -f deploy/kubernetes/configmap.yaml
kubectl apply -f deploy/kubernetes/secrets.yaml

# 部署数据库和缓存
kubectl apply -f deploy/kubernetes/postgres-statefulset.yaml
kubectl apply -f deploy/kubernetes/postgres-service.yaml
kubectl apply -f deploy/kubernetes/redis-deployment.yaml
kubectl apply -f deploy/kubernetes/redis-service.yaml

# 部署应用
kubectl apply -f deploy/kubernetes/backend-deployment.yaml
kubectl apply -f deploy/kubernetes/backend-service.yaml
kubectl apply -f deploy/kubernetes/frontend-deployment.yaml
kubectl apply -f deploy/kubernetes/frontend-service.yaml

# 查看状态
kubectl get pods -n patent-assist
kubectl get services -n patent-assist
```

## 项目结构

```
ai-patent-assist/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── api/            # API 路由和处理器
│   │   ├── auth/           # 认证模块
│   │   ├── config/         # 配置管理
│   │   ├── db/             # 数据库访问
│   │   ├── middleware/     # JWT 认证中间件
│   │   ├── models/         # 数据模型
│   │   ├── services/       # 业务服务
│   │   └── lib.rs          # 库入口文件
│   ├── tests/              # 集成测试
│   ├── migrations/         # 数据库迁移
│   ├── Cargo.toml
│   └── Dockerfile
│
├── frontend/               # Vue 前端
│   ├── src/
│   │   ├── api/           # API 客户端
│   │   ├── components/    # 通用组件 (DataTable, FormDialog)
│   │   ├── layouts/       # 布局组件 (响应式 MainLayout)
│   │   ├── pages/         # 页面组件
│   │   ├── router/        # 路由配置
│   │   ├── stores/        # Pinia 状态 (user, app)
│   │   └── main.ts        # 入口文件
│   ├── package.json
│   ├── Dockerfile
│   └── nginx.conf
│
├── .github/workflows/      # GitHub Actions CI/CD
├── deploy/                 # 部署配置
│   ├── docker-compose.yml
│   ├── docker-compose.prod.yml
│   └── kubernetes/
│
├── docs/                   # 文档
├── .env.example           # 环境变量示例
└── README.md
```

## API 接口

### 认证
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出
- `POST /api/auth/refresh` - 刷新 Token
- `POST /api/auth/register` - 用户注册
- `GET /api/auth/me` - 获取当前用户

### 项目管理
- `GET /api/projects` - 获取项目列表
- `POST /api/projects` - 创建项目
- `GET /api/projects/:id` - 获取项目详情
- `PUT /api/projects/:id` - 更新项目
- `DELETE /api/projects/:id` - 删除项目

### 专利文档
- `POST /api/patents/generate` - 生成专利文档
- `GET /api/patents/:id` - 获取专利详情
- `PUT /api/patents/:id` - 更新专利
- `POST /api/patents/:id/review` - 提交审核
- `POST /api/patents/:id/export` - 导出文档

### 软著文档
- `POST /api/copyrights/generate` - 生成软著文档
- `GET /api/copyrights/:id` - 获取软著详情
- `PUT /api/copyrights/:id` - 更新软著
- `POST /api/copyrights/:id/review` - 提交审核
- `POST /api/copyrights/:id/export` - 导出文档

## 开发指南

### 后端开发

```bash
cd backend

# 复制环境变量配置
cp .env.example .env
# 编辑 .env 文件，填入实际配置

# 安装依赖
cargo install sqlx-cli --no-default-features --features postgres
cargo install

# 运行数据库迁移
cargo sqlx migrate run

# 启动开发服务器
cargo run

# 运行测试
cargo test --test integration

# 代码格式化
cargo fmt

# 代码检查
cargo clippy --all-targets --all-features -- -D warnings
```

### 前端开发

```bash
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build

# 运行代码检查
npm run lint
```

### 环境变量配置

1. 复制根目录的 `.env.example` 文件为 `.env`
2. 修改实际配置值：
   - `DATABASE_URL`: PostgreSQL 连接字符串
   - `JWT_SECRET`: JWT 密钥（生产环境请使用随机生成的安全密钥）
   - `ANTHROPIC_API_KEY`: Anthropic API 密钥

```bash
# 快速开始配置
cp .env.example .env
# 编辑 .env 文件
```

## 安全说明

- 生产环境务必修改 `JWT_SECRET` 为随机生成的安全密钥
- 数据库密码请使用强密码
- 建议启用 HTTPS
- 定期更新 API 密钥

## 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| DATABASE_URL | PostgreSQL 连接 URL | - |
| REDIS_URL | Redis 连接 URL | redis://localhost:6379 |
| JWT_SECRET | JWT 密钥 | - |
| JWT_EXPIRY_HOURS | JWT 过期时间 (小时) | 24 |
| ANTHROPIC_API_KEY | Anthropic API 密钥 | - |
| SERVER_PORT | 服务器端口 | 3000 |
| FILE_STORAGE_PATH | 文件存储路径 | ./uploads |
| MAX_UPLOAD_SIZE_MB | 最大上传文件大小 | 50 |

## 许可证

MIT License

## 联系方式

- Email: your-email@example.com
- Issues: https://github.com/your-org/ai-patent-assist/issues

## 附录：通用组件使用指南

### DataTable 组件

通用数据表格组件，支持分页、排序、筛选等功能。

```vue
<template>
  <DataTable
    :data="tableData"
    :columns="columns"
    :loading="loading"
    :pagination="pagination"
    searchable
    refreshable
    @search="handleSearch"
    @refresh="handleRefresh"
    @edit="handleEdit"
    @delete="handleDelete"
    @page-change="handlePageChange"
  >
    <template #toolbar-left>
      <el-button type="primary" @click="handleAdd">新增</el-button>
    </template>

    <template #status="{ row }">
      <el-tag :type="getStatusType(row.status)">
        {{ getStatusLabel(row.status) }}
      </el-tag>
    </template>
  </DataTable>
</template>

<script setup lang="ts">
import { DataTable } from '@/components'
import type { TableColumn } from '@/components'

const columns: TableColumn[] = [
  { prop: 'name', label: '名称', minWidth: 150 },
  { prop: 'type', label: '类型', width: 100 },
  { prop: 'status', label: '状态', width: 100, slotName: 'status' },
  { prop: 'createdAt', label: '创建时间', width: 180, sortable: true },
]
</script>
```

### FormDialog 组件

通用表单对话框组件，支持动态表单字段配置。

```vue
<template>
  <FormDialog
    v-model="dialogVisible"
    title="编辑项目"
    :form-data="formData"
    :fields="formFields"
    :form-rules="formRules"
    :loading="submitting"
    @submit="handleSubmit"
    @cancel="handleCancel"
  />
</template>

<script setup lang="ts">
import { FormDialog, type FormField } from '@/components'
import type { FormRules } from 'element-plus'

const formData = ref({
  name: '',
  type: 'patent',
  description: '',
})

const formFields: FormField[] = [
  {
    prop: 'name',
    label: '项目名称',
    type: 'input',
    required: true,
    placeholder: '请输入项目名称',
  },
  {
    prop: 'type',
    label: '项目类型',
    type: 'select',
    options: [
      { label: '专利', value: 'patent' },
      { label: '软著', value: 'copyright' },
    ],
  },
  {
    prop: 'description',
    label: '项目描述',
    type: 'textarea',
    rows: 4,
  },
]

const formRules: FormRules = {
  name: [{ required: true, message: '请输入项目名称', trigger: 'blur' }],
}
</script>
```
