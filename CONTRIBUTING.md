# 贡献指南

感谢您对本项目的关注！我们欢迎任何形式的贡献。

## 如何贡献

### 报告问题

- 使用 GitHub Issues 报告 Bug 或功能请求
- 描述问题时提供复现步骤和环境信息
- 搜索现有问题避免重复

### 提交代码

1. **Fork 本仓库**
2. **创建特性分支**: `git checkout -b feature/your-feature-name`
3. **提交更改**: `git commit -m 'feat: 添加新功能'`
4. **推送分支**: `git push origin feature/your-feature-name`
5. **创建 Pull Request**

### Commit Message 规范

本项目使用 Commitizen 和 commitlint 规范 commit message：

```
<type>: <subject>

<body>
```

**Type 类型**:
- `feat`: 新功能
- `fix`: 修复 Bug
- `docs`: 文档变更
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `build`: 构建系统或依赖变更
- `ci`: CI 配置变更
- `chore`: 其他变更

**示例**:
```
feat: 添加用户订单查询功能

- 支持按时间范围查询
- 支持分页展示
```

## 开发环境

### 环境要求

- Rust 1.85.0 或更高版本
- Docker 和 Docker Compose
- 推荐的编辑器：VSCode + rust-analyzer

### 快速开始

```bash
# 克隆仓库
git clone https://github.com/yinzlin/myapp.git
cd myapp

# 安装开发依赖
just setup

# 启动数据库
just db-up

# 运行开发服务器
just dev

# 运行检查
just check-all
```

### 常用命令

| 命令 | 说明 |
|------|------|
| `just dev` | 启动开发服务器 |
| `just test` | 运行所有测试 |
| `just check-all` | 运行所有代码检查 |
| `just docker-up` | 启动 Docker 服务 |

## 代码规范

- 遵循 Rust 官方代码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 所有公共 API 必须有文档注释
- 提交前运行 `just check-all`

## 测试策略

- 新功能必须包含单元测试
- Bug 修复必须包含回归测试
- 使用 `just test-crate <name>` 测试特定模块

## 分支管理

- `main`: 生产就绪代码
- `develop`: 开发分支
- `feature/*`: 特性分支
- `fix/*`: 修复分支

## 许可

本项目基于 MIT 许可证开源。提交代码即表示您同意将您的贡献以 MIT 许可证开源。
