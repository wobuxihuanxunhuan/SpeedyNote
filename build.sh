#!/bin/bash

# SpeedyNote 构建脚本
set -e

echo "🚀 开始构建 SpeedyNote..."

# 检查依赖
if ! command -v cargo &> /dev/null; then
    echo "❌ 未找到 Rust 和 Cargo，请先安装 Rust"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo "❌ 未找到 Docker，请先安装 Docker"
    exit 1
fi

# 清理之前的构建
echo "🧹 清理构建缓存..."
cargo clean

# 构建 Rust 项目
echo "🔨 构建 Rust 项目..."
cargo build --release

# 构建 Docker 镜像
echo "🐳 构建 Docker 镜像..."
docker build -t speedynote:latest .

# 创建数据目录
mkdir -p data

echo "✅ 构建完成！"
echo ""
echo "📋 可用命令："
echo "   docker-compose up -d    # 启动服务"
echo "   docker-compose down     # 停止服务"
echo "   ./deploy.sh            # 部署到云环境"

# 设置执行权限
chmod +x deploy.sh