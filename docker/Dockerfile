# =============================================================================
# Rust 应用多阶段构建 Dockerfile
# 优化点：
#   1. 使用 cargo-chef 缓存依赖层
#   2. 分离依赖安装和代码编译，充分利用 Docker 缓存
#   3. 精简最终镜像，只包含运行时必要组件
#   4. 使用 non-root 用户运行，提升安全性
# =============================================================================

# ============ 阶段 1：依赖安装 ============
FROM rust:1.85-slim AS chef

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /bin/bash builder

# 安装 cargo-chef（构建加速工具）
RUN cargo install cargo-chef

USER builder

# ============ 阶段 2：依赖缓存 ============
FROM chef AS planner

WORKDIR /app

# 复制源码用于分析依赖
COPY --chown=builder:builder . .

# 分析依赖并生成缓存计划
RUN cargo chef prepare --recipe-path /app/recipe.json

# ============ 阶段 3：依赖构建 ============
FROM chef AS builder

WORKDIR /app

# 复制依赖缓存计划
COPY --from=planner --chown=builder:builder /app/recipe.json .

# 使用缓存安装依赖（这一步会被 Docker 缓存）
RUN cargo chef cook --recipe-path /app/recipe.json

# 复制源码
COPY --chown=builder:builder . .

# 编译发布版本
RUN cargo build --release --bin backend

# ============ 阶段 4：运行时镜像 ============
FROM debian:bookworm-slim AS runtime

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /bin/bash appuser

WORKDIR /app

# 从 builder 阶段复制二进制文件
COPY --from=builder --chown=appuser:appuser /app/target/release/backend /usr/local/bin/backend

# 设置环境变量
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV RUST_LOG=info

# 切换到非 root 用户
USER appuser

# 健康检查
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://${SERVER_HOST:-localhost}:${SERVER_PORT:-3000}/api/health || exit 1

EXPOSE 3000

# 启动命令
CMD ["backend"]
