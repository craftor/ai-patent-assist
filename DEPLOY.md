# Docker 部署指南

## 前提条件

- Docker Desktop (Windows/Mac) 或 Docker Engine 20.10+ (Linux)
- Docker Compose v2.0+
- 至少 4GB 可用内存
- 至少 10GB 可用磁盘空间

## 快速开始

### 1. 启动所有服务

```bash
# 启动所有服务（开发环境）
docker compose up -d

# 或者使用测试脚本
./docker-test.sh start
```

### 2. 查看服务状态

```bash
# 查看容器状态
docker compose ps

# 查看服务日志
docker compose logs -f

# 查看特定服务日志
docker compose logs -f backend
docker compose logs -f frontend
```

### 3. 访问应用

- **前端**: http://localhost:8080
- **后端 API**: http://localhost:3000
- **健康检查**: http://localhost:3000/api/health

### 4. 测试账号

```
用户名：admin
密码：admin123
```

## 服务端口

| 服务 | 端口 | 说明 |
|------|------|------|
| frontend | 8080 | Nginx 托管的前端应用 |
| backend | 3000 | Rust/Axum 后端 API |
| postgres | 5432 | PostgreSQL 数据库 |
| redis | 6379 | Redis 缓存 |

## 常用命令

### 构建镜像

```bash
# 重新构建所有镜像
docker compose build

# 仅构建后端
docker compose build backend

# 仅构建前端
docker compose build frontend

# 不使用缓存重新构建
docker compose build --no-cache
```

### 启动/停止服务

```bash
# 启动所有服务
docker compose up -d

# 停止所有服务
docker compose down

# 停止并删除数据卷
docker compose down -v

# 重启服务
docker compose restart
```

### 查看日志

```bash
# 查看所有服务日志
docker compose logs -f

# 查看后端日志
docker compose logs -f backend

# 查看前端日志
docker compose logs -f frontend

# 查看最近 100 行日志
docker compose logs --tail=100 backend
```

### 进入容器

```bash
# 进入后端容器
docker compose exec backend sh

# 进入前端容器
docker compose exec frontend sh

# 进入数据库容器
docker compose exec postgres psql -U patent_user -d patent_db
```

## 运行测试

```bash
# 使用测试脚本进行完整测试
./docker-test.sh test

# 单独执行健康检查
./docker-test.sh health

# 清理旧容器
./docker-test.sh clean
```

### 手动 API 测试

```bash
# 1. 测试登录
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'

# 2. 获取健康状态
curl http://localhost:3000/api/health

# 3. 获取专利列表（需要 Token）
curl -X GET http://localhost:3000/api/patents \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 环境变量配置

复制示例环境变量文件并根据需要修改：

```bash
cp .env.example .env
```

### 主要环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| DATABASE_URL | 数据库连接 URL | postgres://patent_user:patent_password@postgres:5432/patent_db |
| JWT_SECRET | JWT 密钥 | dev-secret-change-in-production |
| JWT_EXPIRY_HOURS | Token 过期时间（小时） | 24 |
| SERVER_PORT | 后端服务端口 | 3000 |
| ANTHROPIC_API_KEY | Anthropic API 密钥 | - |

## 数据持久化

数据存储在以下 Docker 卷中：

- `postgres_data` - PostgreSQL 数据
- `file_storage` - 用户上传的文件

### 备份数据

```bash
# 备份 PostgreSQL 数据
docker compose exec postgres pg_dump -U patent_user patent_db > backup.sql

# 恢复数据
docker compose exec -T postgres psql -U patent_user patent_db < backup.sql
```

## 故障排查

### 后端无法启动

```bash
# 查看后端日志
docker compose logs backend

# 检查数据库是否就绪
docker compose logs postgres
```

### 前端无法访问

```bash
# 查看前端日志
docker compose logs frontend

# 检查 nginx 配置
docker compose exec frontend nginx -t
```

### 数据库连接失败

```bash
# 测试数据库连接
docker compose exec postgres pg_isready -U patent_user

# 查看数据库日志
docker compose logs postgres
```

### 重置所有数据

```bash
# 停止服务并删除数据卷
docker compose down -v

# 重新启动
docker compose up -d
```

## 生产环境部署

使用生产环境配置：

```bash
# 使用生产环境配置启动
docker compose -f docker-compose.prod.yml up -d
```

确保设置以下环境变量：

```bash
export POSTGRES_USER=your_user
export POSTGRES_PASSWORD=your_secure_password
export JWT_SECRET=your_secret_key
export ANTHROPIC_API_KEY=your_api_key
```

## 镜像大小优化

```bash
# 查看镜像大小
docker images | grep patent

# 清理未使用的镜像
docker image prune -a
```

## 网络配置

所有服务都在 `patent-network` 网络中，可以通过服务名称互相访问：

- 后端访问数据库：`postgres:5432`
- 前端访问 API：`backend:3000`（容器内）或 `localhost:3000`（容器外）
