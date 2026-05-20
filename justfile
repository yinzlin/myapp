set shell := ["nu", "-c"]

# 构建项目
build:
    cargo build --release

# 开发模式
dev:
    cargo run --bin backend

# 运行测试
test:
    cargo test --all

# 代码检查
lint:
    cargo clippy --all-targets -- -D warnings

# 格式化代码
fmt:
    cargo fmt --all

# 启动 Docker
docker-up:
    docker-compose -f docker/docker-compose.yml up -d
