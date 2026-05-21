# =============================================================================
# Rust 项目开发任务自动化
# 使用方法：just <recipe>
# 参考文档：https://just.systems/
# =============================================================================

# 设置 shell（使用 nushell）
set shell := ["nu", "-c"]

# 默认任务：显示帮助
default:
    @just --list

# ============ 构建相关 ============

# 构建发布版本
build:
    cargo build --release --all-targets

# 构建指定 crate
build-crate name:
    cargo build --release -p {{name}}

# 开发模式运行（热重载推荐使用 cargo-watch）
dev:
    cargo run --bin backend

# 清理构建缓存
clean:
    cargo clean --all
    docker system prune -f

# ============ 代码质量 ============

# 代码格式化
fmt:
    cargo fmt --all

# 代码检查（Clippy）
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# 检查代码（编译检查）
check:
    cargo check --all-targets --all-features

# 运行 Clippy 自动修复（需人工确认）
fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# 运行所有质量检查（fmt + check + lint）
check-all: fmt check lint
    echo "✅ 所有检查通过！"

# ============ 测试相关 ============

# 运行所有测试
test:
    cargo test --all --verbose

# 运行指定 crate 的测试
test-crate name:
    cargo test -p {{name}} --verbose

# 运行测试并显示输出
test-with-output:
    cargo test --all -- --nocapture

# 运行文档测试
doc-test:
    cargo test --doc --all

# ============ 开发辅助 ============

# 安装开发依赖
setup:
    rustup component add rustfmt clippy rust-src
    cargo install cargo-watch

# 进入开发模式（热重载）
watch:
    cargo watch -x check -x test

# 生成项目文档
doc:
    cargo doc --all --no-deps --open

# 打开项目 README
readme:
    @bat README.md

# ============ 数据库相关 ============

# 启动数据库服务
db-up:
    docker-compose -f docker/docker-compose.yml up -d postgres redis

# 停止数据库服务
db-down:
    docker-compose -f docker/docker-compose.yml down

# 运行数据库迁移
migrate:
    sqlx migrate run

# 重置数据库
db-reset:
    docker-compose -f docker/docker-compose.yml down -v
    docker-compose -f docker/docker-compose.yml up -d

# ============ Docker 相关 ============

# 启动所有服务
docker-up:
    docker-compose -f docker/docker-compose.yml up -d

# 停止所有服务
docker-down:
    docker-compose -f docker/docker-compose.yml down

# 构建 Docker 镜像
docker-build:
    docker build -f docker/Dockerfile -t myapp:latest .

# 查看 Docker 日志
docker-logs service="backend":
    docker logs -f myapp_{{service}}

# 清理 Docker 资源
docker-clean:
    docker system prune -af
    docker volume prune -f

# ============ CI/CD 模拟 ============

# 模拟 CI 检查流程
ci-check: fmt check lint
    echo "✅ CI 检查通过！"

# 模拟完整 CI 流程（包括测试）
ci-full: check-all test
    echo "✅ CI 完整流程通过！"

# ============ 其他 ============

# 显示环境信息
info:
    @echo "=== Rust 环境 ==="
    rustc --version
    cargo --version
    @echo "=== 项目信息 ==="
    cargo metadata --format-version 1 | nu -c "open - | get packages | each { |p| $p.name + ' ' + $p.version } | str join (char newline)"

# 列出所有任务
help:
    @just --list --list-heading $'🛠️  可用任务:\n'
