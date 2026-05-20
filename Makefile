.PHONY: dev build test lint fmt migrate clean docker-up docker-down help

help:
	@echo "可用命令:"
	@echo "  make dev          - 启动开发环境"
	@echo "  make build        - 构建生产版本"
	@echo "  make test         - 运行全量测试"
	@echo "  make lint         - 运行 Clippy 检查"
	@echo "  make fmt          - 格式化代码"
	@echo "  make migrate      - 运行数据库迁移"
	@echo "  make clean        - 清理构建产物"
	@echo "  make docker-up    - 启动 Docker 服务"
	@echo "  make docker-down  - 停止 Docker 服务"

dev:
	cargo watch -x "run --bin backend"

build:
	cargo build --release

test:
	cargo test --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

migrate:
	sqlx migrate run

clean:
	cargo clean

docker-up:
	docker-compose -f docker/docker-compose.yml up -d

docker-down:
	docker-compose -f docker/docker-compose.yml down

check: fmt lint test
