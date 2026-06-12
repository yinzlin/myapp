# Rust DDD 用户订单管理系统 - 示例

## 项目概述
这是一个完整的 Rust 全栈项目，采用六边形架构（Hexagonal Architecture）和领域驱动设计（DDD）。
项目展示了如何实现覆盖完整 Rust 数据类型的 CRUD 操作。

## 技术栈
- **后端**：Axum（异步 Web 框架）
- **数据库**：SQLite（轻量级存储）
- **ORM**：SQLx（类型安全的 SQL 查询）
- **日志**：Tracing（结构化日志）

## 项目架构
```
myapp/
├── crates/
│   ├── domain/          # 领域层（纯业务逻辑）
│   ├── application/     # 应用层（服务编排）
│   ├── database/        # 数据库层（仓储实现）
│   ├── backend/         # 后端层（API接口）
│   └── common/          # 通用工具
└── migrations/          # 数据库迁移
```

## 启动应用
```bash
# 1. 创建数据目录（首次运行）
mkdir -p data

# 2. 启动服务
cargo run --bin backend
```

服务将在 http://localhost:3000 启动

## API 端点

### 健康检查
```bash
GET /api/health
```

### 用户 API

#### 创建用户
```bash
POST /api/users
Content-Type: application/json

{
  "name": "张三",
  "email": "zhangsan@example.com"
}
```

#### 获取用户
```bash
GET /api/users/{id}
```

#### 获取所有用户
```bash
GET /api/users
```

#### 更新用户
```bash
PUT /api/users/{id}
Content-Type: application/json

{
  "name": "张三三",
  "email": "zhangsan3@example.com",
  "age": 30,
  "rating": 4.5,
  "note": "这个用户很重要"
}
```

#### 删除用户
```bash
DELETE /api/users/{id}
```

### 订单 API

#### 创建订单
```bash
POST /api/orders
Content-Type: application/json

{
  "user_id": "用户ID"
}
```

#### 获取订单
```bash
GET /api/orders/{id}
```

#### 获取所有订单
```bash
GET /api/orders
```

#### 获取用户的所有订单
```bash
GET /api/users/{user_id}/orders
```

#### 添加订单项
```bash
POST /api/orders/{id}/items
Content-Type: application/json

{
  "item": {
    "product_name": "iPhone 15",
    "quantity": 2,
    "unit_price_yuan": 8999.0
  }
}
```

#### 设置折扣
```bash
POST /api/orders/{id}/discount
Content-Type: application/json

{
  "discount_rate": 0.1
}
```

#### 更新订单状态
```bash
POST /api/orders/{id}/status
Content-Type: application/json

{
  "status": "pay"  // 可选: pay, ship, complete, cancel
}
```

#### 删除订单
```bash
DELETE /api/orders/{id}
```

## 数据类型覆盖
这个项目展示了如何在 DDD 中使用各种 Rust 数据类型：

### 值对象（Value Objects）
1. **UserId/OrderId**：使用 `Uuid` 类型
2. **UserName**：`String` 类型，带业务校验（2-50 字符）
3. **Email**：`String` 类型，带邮箱格式校验
4. **Age**：`u8` 类型，限制范围（0-150）
5. **Money**：`i64` 类型，以分为单位，避免浮点精度问题
6. **Quantity**：`u32` 类型，商品数量
7. **DiscountRate**：`f64` 类型，折扣率（0-1）
8. **IsActive**：`bool` 类型
9. **Rating**：`f32` 类型，评分（1-5）
10. **OrderStatus**：枚举类型，状态机模式
11. **Tags**：`Vec<String>` 类型
12. **Metadata**：`HashMap<String, String>` 类型
13. **CreatedAt**：`DateTime<Utc>` 类型，时间戳
14. **OptionalNote**：`Option<String>` 类型，可选字段

## DDD 概念

### 聚合根（Aggregate Root）
1. **User**：用户聚合，管理用户信息和状态
2. **Order**：订单聚合，管理订单生命周期

### 充血模型（Rich Domain Model）
- 业务逻辑封装在领域对象内部
- 例如：订单有 `pay()`、`ship()`、`complete()`、`cancel()` 方法

### 领域服务（Domain Service）
- 跨多个聚合的业务逻辑

### 仓储（Repository）
- `UserRepository`：用户仓储接口
- `OrderRepository`：订单仓储接口
- `SqliteUserRepository` / `SqliteOrderRepository`：SQLite 实现

### 应用服务（Application Service）
- 编排多个领域对象
- 处理事务边界

## 下一步扩展
1. 添加 Postgres 支持
2. 添加 Redis 缓存
3. 添加前端（Dioxus）
4. 添加 Docker 容器化
5. 添加 CI/CD 流程
6. 添加单元测试和集成测试
