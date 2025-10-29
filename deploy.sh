#!/bin/bash

# SpeedyNote 部署脚本
set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 SpeedyNote 部署脚本${NC}"
echo ""

# 部署选项
DEPLOY_ENV="${1:-local}"
IMAGE_TAG="${2:-latest}"

case $DEPLOY_ENV in
    "local")
        echo -e "${YELLOW}📦 部署到本地环境...${NC}"
        
        # 检查Docker Compose
        if ! command -v docker-compose &> /dev/null; then
            echo -e "${RED}❌ 未找到 docker-compose${NC}"
            exit 1
        fi
        
        # 停止现有服务
        echo "🛑 停止现有服务..."
        docker-compose down || true
        
        # 启动服务
        echo "🚀 启动服务..."
        docker-compose up -d
        
        # 等待服务启动
        echo "⏳ 等待服务启动..."
        sleep 10
        
        # 检查服务状态
        if curl -f http://localhost:3000/health > /dev/null 2>&1; then
            echo -e "${GREEN}✅ 服务启动成功！${NC}"
            echo -e "${BLUE}🌐 应用地址: http://localhost:3000${NC}"
        else
            echo -e "${RED}❌ 服务启动失败${NC}"
            docker-compose logs speedynote-app
            exit 1
        fi
        ;;
        
    "docker")
        echo -e "${YELLOW}🐳 部署到Docker环境...${NC}"
        
        # 运行单个容器
        docker run -d \
            --name speedynote \
            -p 3000:3000 \
            -v $(pwd)/data:/app/data \
            speedynote:latest
        
        echo -e "${GREEN}✅ Docker容器启动成功！${NC}"
        ;;
        
    "kubernetes")
        echo -e "${YELLOW}☸️ 部署到Kubernetes...${NC}"
        
        if ! command -v kubectl &> /dev/null; then
            echo -e "${RED}❌ 未找到 kubectl${NC}"
            exit 1
        fi
        
        # 应用Kubernetes配置
        kubectl apply -f kubernetes/
        
        echo -e "${GREEN}✅ Kubernetes部署完成！${NC}"
        echo "使用 'kubectl get pods' 查看部署状态"
        ;;
        
    "cloud")
        echo -e "${YELLOW}☁️ 部署到云平台...${NC}"
        
        # 这里可以添加云平台特定的部署逻辑
        # 例如 AWS ECS、Google Cloud Run、Azure Container Instances 等
        
        echo "请根据目标云平台配置部署脚本"
        echo "支持的平台: AWS, GCP, Azure, DigitalOcean等"
        ;;
        
    *)
        echo -e "${RED}❌ 未知的部署环境: $DEPLOY_ENV${NC}"
        echo ""
        echo -e "${BLUE}📋 可用选项:${NC}"
        echo "  local      - 本地Docker Compose部署"
        echo "  docker     - 单个Docker容器部署"
        echo "  kubernetes - Kubernetes集群部署"
        echo "  cloud      - 云平台部署"
        echo ""
        echo "示例:"
        echo "  ./deploy.sh local"
        echo "  ./deploy.sh kubernetes"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}🎉 部署完成！${NC}"
echo ""
echo -e "${BLUE}📊 监控命令:${NC}"
echo "  docker-compose logs -f speedynote-app    # 查看日志"
echo "  docker stats                              # 查看资源使用"
echo "  curl http://localhost:3000/health         # 健康检查"

# 如果是本地部署，显示访问信息
if [ "$DEPLOY_ENV" = "local" ]; then
    echo ""
    echo -e "${GREEN}🔗 快速访问:${NC}"
    echo "  Web界面: http://localhost:3000"
    echo "  API文档: http://localhost:3000/api/docs"
    echo "  健康检查: http://localhost:3000/health"
fi