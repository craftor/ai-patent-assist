# 多阶段构建 - 后端 Rust 服务
FROM rust:latest AS backend-builder

WORKDIR /app

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件
COPY backend/Cargo.toml ./Cargo.toml

# 创建占位源文件以便缓存依赖
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# 复制源代码
COPY backend/src ./src

# 重新构建（使用缓存的依赖）
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim AS backend

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/release/ai-patent-assist-backend /usr/local/bin/server

RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3000

CMD ["server"]
