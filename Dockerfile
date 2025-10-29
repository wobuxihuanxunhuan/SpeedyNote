# 多阶段构建：构建阶段
FROM rust:1.70 AS builder

# 安装系统依赖（包括Tauri需要的依赖）
RUN apt-get update && apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libpq-dev \
    pkg-config \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# 创建工作目录
WORKDIR /app

# 复制所有项目文件
COPY . .

# 生成Cargo.lock文件并构建应用
RUN cargo generate-lockfile && cargo build --release

# 剥离调试符号以减小镜像大小
RUN strip target/release/speedynote

# 运行时阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libwebkit2gtk-4.0-37 \
    librsvg2-2 \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN groupadd -r speedynote && useradd -r -g speedynote speedynote

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/speedynote ./

# 创建数据目录
RUN mkdir -p /app/data && chown -R speedynote:speedynote /app

# 切换到非root用户
USER speedynote

# 暴露端口
EXPOSE 3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# 设置环境变量
ENV RUST_LOG=info
ENV PORT=3000
ENV RUST_BACKTRACE=1

# 启动命令
CMD ["./speedynote"]