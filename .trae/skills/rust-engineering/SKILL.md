---
name: "rust-engineering"
description: "Rust 全栈工程化实践指南。Invoke when starting new Rust project, adding dependencies, or following DDD architecture patterns."
---

# Rust 全栈工程化实践

本 Skill 基于 Axum + Dioxus + SQLx + PostgreSQL + Redis 技术栈，提供工程化最佳实践指导。

## 触发场景

当用户出现以下情况时，必须调用此 Skill：

1. **新项目初始化** - 创建新的 Rust 全栈项目
2. **依赖变更** - 添加或修改 workspace dependencies
3. **架构设计** - 实现 DDD 分层架构
4. **代码审查** - 检查代码是否符合工程化规范
5. **Docker 部署** - 构建或优化容器镜像
6. **CI/CD 配置** - 修改 GitHub Actions 工作流

## 核心规范速查

### 依赖管理（必须先读取）

```toml
# workspace.dependencies 格式
[workspace.dependencies]
tokio = { version = "1.43", features = ["full"] }
axum = "0.8"
tower-http = { version = "0.6", features = ["cors", "trace", "fs"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "sqlite", "chrono", "uuid"] }
redis = { version = "0.27", default-features = false, features = ["tokio", "tokio-comp", "connection-manager"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "2.0"
```

### 子 crate Cargo.toml 格式

```toml
[dependencies]
domain.workspace = true
application.workspace = true
tokio.workspace = true
axum.workspace = true
```

## 项目结构（六边形架构）

```
crates/
├── domain/          # 领域层：纯业务模型，无外部依赖
│   ├── src/
│   │   ├── aggregate/   # 聚合根
│   │   ├── value_object/# 值对象
│   │   ├── port/        # 出站端口（Trait）
│   │   └── error/       # 领域错误
│   └── Cargo.toml
├── application/     # 应用层：用例编排
│   ├── src/
│   │   ├── service/     # 应用服务
│   │   ├── dto/        # 数据传输对象
│   │   └── port/       # 入站端口
│   └── Cargo.toml
├── backend/         # HTTP 适配器（Axum）
├── database/       # 数据持久化适配器
├── frontend/       # 前端适配器（Dioxus）
└── common/         # 通用工具
```

## 行级TDD开发流程

### 任务拆解层级

```
Sprint → Story → Module → Function → Code Block → Line-level Task
```

### 开发节奏

1. **任务拆分** → 按层级拆解到行级子任务
2. **骨架创建** → 创建空函数签名
3. **逐行实现** → 每次只写一行/一块极简代码
4. **cargo check** → 每行校验后才继续
5. **单元测试** → 骨架完成后编写测试

### 输出格式

```markdown
【当前行级子任务】
- 描述
- 实现思路

【代码】
```rust
// 单行或极简代码块
```

【说明】
- 本行逻辑
```

## 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 类型/结构体 | 大驼峰 | `UserOrder`, `HttpClient` |
| 变量/函数 | 蛇形 | `user_id`, `create_order` |
| 常量 | 大写蛇形 | `MAX_RETRY_COUNT` |
| 数据库表 | 蛇形 | `user_orders` |
| API 路由 | 蛇形 | `/api/v1/user-orders` |

## 代码质量门槛

| 检查项 | 命令 | 门槛 |
|--------|------|------|
| 格式 | `cargo fmt --all` | 必须通过 |
| 检查 | `cargo clippy -- -D warnings` | 零警告 |
| 测试 | `cargo test --all` | 全部通过 |
| 编译 | `cargo check --all-targets` | 必须通过 |

## 错误处理规范

```rust
// 领域层：使用 thiserror
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("无效业务规则: {0}")]
    InvalidRule(String),
}

// 应用层：使用 thiserror
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("资源不存在: {resource_id}")]
    NotFound { resource_id: String },
}
```

## Docker 多阶段构建

```dockerfile
# 阶段1：chef（准备）
FROM rust:1.85-slim AS chef

# 阶段2：planner（分析依赖）
FROM chef AS planner

# 阶段3：builder（编译）
FROM chef AS builder

# 阶段4：runtime（运行）
FROM debian:bookworm-slim AS runtime
# 必须使用 non-root 用户
USER appuser
```

## 相关规范文件

- `.trae/rules/架构方案.md` - 六边形架构定义
- `.trae/rules/工程化规范.md` - 工程化标准
- `.trae/rules/开发流程规范.md` - TDD 开发流程

## 使用示例

**场景：添加新依赖**

```
用户：需要添加一个 HTTP 客户端库

AI（触发 rust-engineering skill）：
→ 检查现有依赖（reqwest 已存在）
→ 建议复用 reqwest 或按需版本升级
→ 演示正确的 workspace.dependencies 格式
```

**场景：创建新模块**

```
用户：要实现用户订单查询功能

AI（触发 rust-engineering skill）：
→ 按六边形架构定义领域模型
→ 创建 UserOrder 聚合根
→ 定义出站端口
→ 实现应用服务
→ 实现数据库适配器
→ 创建 HTTP 控制器
→ 注册路由
```

---

**调用方式**：当遇到 Rust 全栈项目开发相关问题时，AI 应自动触发此 Skill 并参照上述规范执行。
