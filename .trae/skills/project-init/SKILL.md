---
name: "myapp"
description: "初始化新的 Rust 全栈项目脚手架。Invoke when user wants to create a new project or reset from template."
---

# Rust 全栈项目初始化

本 Skill 用于从模板初始化新的 Rust 全栈项目，确保工程化规范开箱即用。

## 触发场景

当用户出现以下情况时，必须调用此 Skill：

1. **创建新项目** - 从零开始新的 Rust 全栈项目
2. **重置模板** - 基于现有模板重置项目
3. **环境重建** - 重新搭建开发环境
4. **模板更新** - 将最新规范同步到现有项目

## 初始化流程

### 第一步：环境检查

```bash
# 检查 Rust 版本（必须 >= 1.85）
rustc --version

# 检查 Cargo 版本
cargo --version

# 检查 Docker 版本
docker --version

# 检查 nushell（推荐）
nu --version
```

### 第二步：项目骨架创建

```bash
# 1. 创建项目目录
mkdir -p myapp
cd myapp

# 2. 创建 Rust workspace 结构
mkdir -p crates/{domain,application,backend,database,frontend,common}
mkdir -p docker
mkdir -p .github/workflows
mkdir -p migrations

# 3. 创建 .trae 规则目录
mkdir -p .trae/rules
mkdir -p .trae/skills
```

### 第三步：核心配置文件

必须包含以下配置文件：

| 文件 | 作用 | 必须性 |
|------|------|--------|
| `Cargo.toml` | Workspace 根配置 | ✅ 必须 |
| `rust-toolchain.toml` | MSRV 定义 | ✅ 必须 |
| `.clippy.toml` | Clippy 配置 | ✅ 必须 |
| `.rustfmt.toml` | 格式化配置 | ✅ 必须 |
| `.editorconfig` | 编辑器配置 | ✅ 必须 |
| `justfile` | 开发任务自动化 | ✅ 必须 |
| `docker/Dockerfile` | 多阶段构建 | ✅ 必须 |
| `docker/docker-compose.yml` | 本地开发环境 | ✅ 必须 |
| `.github/workflows/ci.yml` | CI 工作流 | ✅ 必须 |
| `.pre-commit-config.yaml` | Git Hooks | 推荐 |

### 第四步：Workspace 配置模板

```toml
# Cargo.toml
[workspace]
members = [
    "crates/domain",
    "crates/application",
    "crates/backend",
    "crates/database",
    "crates/frontend",
    "crates/common",
]
resolver = "3"

[workspace.package]
edition = "2024"
version = "0.1.0"
authors = ["Dreamer <835831088@qq.com>"]
license = "MIT"

[workspace.dependencies]
# 根配置只使用 * 版本号，features 在子 crate 中按需开启
tokio = "*"
axum = "*"
tower = "*"
tower-http = "*"
sqlx = "*"
redis = "*"
serde = "*"
thiserror = "*"
async-trait = "*"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 第五步：规则文件同步

必须创建以下规则文件：

```
.trae/
├── rules/
│   ├── 架构方案.md       # 从模板复制
│   ├── 工程化规范.md     # 从模板复制
│   └── 开发流程规范.md   # 从模板复制
└── skills/
    ├── rust-engineering/ # 从模板复制
    └── project-init/     # 从模板复制
```

### 第六步：开发环境验证

```bash
# 1. 格式化检查
cargo fmt --all -- --check

# 2. Clippy 检查
cargo clippy -- -D warnings

# 3. 编译检查
cargo check --all-targets

# 4. 运行测试
cargo test --all

# 5. Docker 环境验证
docker-compose -f docker/docker-compose.yml up -d
docker-compose ps
```

## 快速初始化脚本

```bash
#!/bin/bash
# init-project.sh

set -e

PROJECT_NAME=${1:-new-project}

# 克隆模板
git clone https://github.com/yinzlin/myapp.git $PROJECT_NAME
cd $PROJECT_NAME

# 重置 Git
rm -rf .git
git init

# 更新项目名称
sed -i "s/myapp/$PROJECT_NAME/g" Cargo.toml
sed -i "s/myapp/$PROJECT_NAME/g" docker/docker-compose.yml

# 初始化子模块
git add .
git commit -m "feat: initialize project from template"

echo "✅ 项目 $PROJECT_NAME 初始化完成！"
echo "📁 cd $PROJECT_NAME && just dev"
```

## 初始化检查清单

- [ ] Rust 版本 >= 1.96
- [ ] Cargo.lock 已生成
- [ ] `cargo check` 通过
- [ ] `cargo clippy -- -D warnings` 零警告
- [ ] `cargo test --all` 通过
- [ ] Docker 服务可启动
- [ ] Git 仓库已初始化
- [ ] `.trae/rules/` 规则文件已就位
- [ ] `.trae/skills/` 技能文件已就位

## 常见问题处理

### 问题：依赖下载失败

```bash
# 使用国内镜像
cargo install cargo-chef --locked
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
```

### 问题：Docker 构建失败

```bash
# 清理缓存重试
docker system prune -af
docker build -f docker/Dockerfile -t myapp:latest .
```

### 问题：MSRV 版本不匹配

```bash
# 更新 Rust
rustup update stable
rustup show
```

---

**调用方式**：当用户要求创建新项目或初始化模板时，AI 应自动触发此 Skill 并按照上述流程执行。
