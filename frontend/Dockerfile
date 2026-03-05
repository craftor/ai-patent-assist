# 多阶段构建 - 前端 Vue 应用
FROM node:20-slim AS frontend-builder

WORKDIR /app

# 复制 package 文件
COPY frontend/package.json ./package.json
COPY frontend/package-lock.json* ./package-lock.json

# 安装依赖
RUN npm install

# 复制源代码
COPY frontend/. ./

# 构建（跳过类型检查）
RUN npm run build-only

# 运行阶段 - 使用 Nginx
FROM nginx:alpine AS frontend

# 复制构建产物
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

# 复制 nginx 配置
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
