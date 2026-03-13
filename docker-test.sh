#!/bin/bash
# Docker 构建和测试脚本

set -e

echo "========================================"
echo " AI 专利助手 - Docker 构建和测试脚本"
echo "========================================"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查 Docker 是否运行
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo -e "${RED}错误：Docker 未运行${NC}"
        echo "请先启动 Docker Desktop 或运行 'sudo systemctl start docker'"
        exit 1
    fi
    echo -e "${GREEN}✓ Docker 正在运行${NC}"
}

# 停止并清理旧容器
cleanup() {
    echo "清理旧容器和网络..."
    docker compose down --remove-orphans 2>/dev/null || true
}

# 构建镜像
build() {
    echo "构建 Docker 镜像..."

    echo "  - 构建后端..."
    docker compose build backend

    echo "  - 构建前端..."
    docker compose build frontend

    echo -e "${GREEN}✓ 镜像构建完成${NC}"
}

# 启动服务
start() {
    echo "启动服务..."
    docker compose up -d

    echo "等待服务启动..."
    sleep 10

    echo -e "${GREEN}✓ 服务已启动${NC}"
}

# 健康检查
health_check() {
    echo "执行健康检查..."

    # 检查后端健康
    if curl -sf http://localhost:3000/api/health > /dev/null; then
        echo -e "${GREEN}✓ 后端健康检查通过${NC}"
    else
        echo -e "${RED}✗ 后端健康检查失败${NC}"
        return 1
    fi

    # 检查前端健康
    if curl -sf http://localhost:8080 > /dev/null; then
        echo -e "${GREEN}✓ 前端健康检查通过${NC}"
    else
        echo -e "${RED}✗ 前端健康检查失败${NC}"
        return 1
    fi
}

# 运行 API 测试
test_api() {
    echo "运行 API 测试..."

    # 测试登录接口
    echo "  - 测试登录接口..."
    LOGIN_RESPONSE=$(curl -sf -X POST http://localhost:3000/api/auth/login \
        -H "Content-Type: application/json" \
        -d '{"username":"admin","password":"admin123"}')

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ 登录接口测试通过${NC}"
        TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.data.access_token')
        echo "  Token: ${TOKEN:0:20}..."
    else
        echo -e "${RED}✗ 登录接口测试失败${NC}"
        return 1
    fi

    # 测试获取当前用户
    echo "  - 测试获取用户信息..."
    if [ -n "$TOKEN" ] && [ "$TOKEN" != "null" ]; then
        curl -sf -X GET http://localhost:3000/api/auth/me \
            -H "Authorization: Bearer $TOKEN" > /dev/null
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ 用户信息接口测试通过${NC}"
        else
            echo -e "${YELLOW}! 用户信息接口返回异常（可能是测试账号未创建）${NC}"
        fi
    fi

    # 测试专利列表
    echo "  - 测试专利列表接口..."
    if [ -n "$TOKEN" ] && [ "$TOKEN" != "null" ]; then
        curl -sf -X GET http://localhost:3000/api/patents \
            -H "Authorization: Bearer $TOKEN" > /dev/null
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ 专利列表接口测试通过${NC}"
        else
            echo -e "${YELLOW}! 专利列表接口返回异常${NC}"
        fi
    fi
}

# 查看日志
logs() {
    echo "查看服务日志..."
    docker compose logs -f "$@"
}

# 停止服务
stop() {
    echo "停止服务..."
    docker compose down
    echo -e "${GREEN}✓ 服务已停止${NC}"
}

# 显示使用说明
show_usage() {
    echo "使用方法:"
    echo "  $0 {build|start|stop|restart|test|health|logs|clean}"
    echo ""
    echo "命令说明:"
    echo "  build   - 构建 Docker 镜像"
    echo "  start   - 启动所有服务"
    echo "  stop    - 停止所有服务"
    echo "  restart - 重启所有服务"
    echo "  test    - 运行完整测试"
    echo "  health  - 执行健康检查"
    echo "  logs    - 查看服务日志"
    echo "  clean   - 清理容器和网络"
    echo ""
    echo "示例:"
    echo "  $0 build     # 构建镜像"
    echo "  $0 start     # 启动服务"
    echo "  $0 test      # 运行测试"
    echo "  $0 logs backend  # 查看后端日志"
}

# 主逻辑
main() {
    case "${1:-}" in
        build)
            check_docker
            build
            ;;
        start)
            check_docker
            start
            ;;
        stop)
            check_docker
            stop
            ;;
        restart)
            check_docker
            stop
            start
            ;;
        test)
            check_docker
            health_check
            test_api
            ;;
        health)
            check_docker
            health_check
            ;;
        logs)
            check_docker
            logs "${2:-}"
            ;;
        clean)
            check_docker
            cleanup
            ;;
        *)
            show_usage
            ;;
    esac
}

main "$@"
