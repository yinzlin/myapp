//! SQLite 仓储实现
//!
//! 提供 SQLite 数据库连接池和基础操作。

use anyhow::Result;
use chrono::{DateTime, Utc};
use domain::aggregate::{Order, User};
use domain::error::DomainError;
use domain::port::{OrderRepository, UserRepository};
use domain::value_object::*;
use serde_json;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// SQLite 数据库管理器
///
/// 封装 SQLite 连接池，提供数据库操作能力。
#[derive(Debug, Clone)]
pub struct SqliteManager {
    /// SQLite 连接池
    pub pool: Arc<SqlitePool>,
}

impl SqliteManager {
    /// 创建新的 SQLite 管理器
    ///
    /// # 参数
    /// * `url` - SQLite 数据库连接 URL，格式: `sqlite:./data/app.db`
    ///
    /// # 返回
    /// 成功返回 SqliteManager，失败返回错误
    pub async fn new(url: &str) -> Result<Self> {
        info!("正在连接 SQLite 数据库: {}", url);

        // 创建连接池
        let pool = SqlitePool::connect(url).await?;

        // 验证连接
        let result = sqlx::query("SELECT 1").fetch_one(&pool).await;
        match result {
            Ok(_) => info!("SQLite 连接成功"),
            Err(e) => warn!("SQLite 连接验证失败: {}", e),
        }

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 执行数据库迁移
    ///
    /// 应用 SQLx 迁移脚本到数据库。
    pub async fn migrate(&self) -> Result<()> {
        info!("正在执行 SQLite 数据库迁移");

        sqlx::migrate!("./migrations").run(&*self.pool).await?;

        info!("SQLite 数据库迁移完成");
        Ok(())
    }

    /// 检查数据库健康状态
    pub async fn health_check(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&*self.pool).await {
            Ok(_) => true,
            Err(e) => {
                warn!("SQLite 健康检查失败: {}", e);
                false
            }
        }
    }
}

// ==================== SQLite 用户仓储实现 ====================

/// SQLite 用户仓储
#[derive(Debug, Clone)]
pub struct SqliteUserRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteUserRepository {
    /// 创建新的用户仓储
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        SqliteUserRepository { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for SqliteUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let tags_json = serde_json::to_string(user.tags().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let metadata_json = serde_json::to_string(user.metadata().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, age, rating, is_active, tags, metadata, created_at, note)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.name().as_str())
        .bind(user.email().as_str())
        .bind(user.age().map(|a| a.value()))
        .bind(user.rating().map(|r| r.value()))
        .bind(user.is_active().value())
        .bind(tags_json)
        .bind(metadata_json)
        .bind(user.created_at().datetime())
        .bind(user.note().as_option())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, age, rating, is_active, tags, metadata, created_at, note
            FROM users WHERE id = ?
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(row.map(|r| r.into_user()).transpose()?)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, age, rating, is_active, tags, metadata, created_at, note
            FROM users WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(row.map(|r| r.into_user()).transpose()?)
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, age, rating, is_active, tags, metadata, created_at, note
            FROM users
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        let users: Result<Vec<_>, _> = rows.into_iter().map(|r| r.into_user()).collect();
        users
    }

    async fn delete(&self, id: UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.as_uuid())
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let tags_json = serde_json::to_string(user.tags().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let metadata_json = serde_json::to_string(user.metadata().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;

        sqlx::query(
            r#"
            UPDATE users
            SET name = ?, email = ?, age = ?, rating = ?, is_active = ?, tags = ?, metadata = ?, note = ?
            WHERE id = ?
            "#,
        )
        .bind(user.name().as_str())
        .bind(user.email().as_str())
        .bind(user.age().map(|a| a.value()))
        .bind(user.rating().map(|r| r.value()))
        .bind(user.is_active().value())
        .bind(tags_json)
        .bind(metadata_json)
        .bind(user.note().as_option())
        .bind(user.id().as_uuid())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(())
    }
}

/// 用户行记录
#[derive(sqlx::FromRow)]
struct UserRow {
    id: uuid::Uuid,
    name: String,
    email: String,
    age: Option<u8>,
    rating: Option<i8>,
    is_active: bool,
    tags: String,
    metadata: String,
    created_at: DateTime<Utc>,
    note: Option<String>,
}

impl UserRow {
    fn into_user(self) -> Result<User, DomainError> {
        let tags_vec: Vec<String> = serde_json::from_str(&self.tags)
            .map_err(|e| DomainError::BusinessRuleViolation(format!("反序列化失败: {}", e)))?;
        let metadata_map: HashMap<String, String> = serde_json::from_str(&self.metadata)
            .map_err(|e| DomainError::BusinessRuleViolation(format!("反序列化失败: {}", e)))?;

        Ok(User::reconstruct(
            UserId::from_uuid(self.id),
            UserName::new(self.name)?,
            Email::new(self.email)?,
            self.age.map(|a| Age::new(a)).transpose()?,
            self.rating.map(|r| Rating::new(r)).transpose()?,
            IsActive::new(self.is_active),
            Tags::new(tags_vec),
            Metadata::from_hashmap(metadata_map),
            CreatedAt::from_datetime(self.created_at),
            self.note
                .map(OptionalNote::some)
                .unwrap_or_else(OptionalNote::none),
        ))
    }
}

// ==================== SQLite 订单仓储实现 ====================

/// SQLite 订单仓储
#[derive(Debug, Clone)]
pub struct SqliteOrderRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteOrderRepository {
    /// 创建新的订单仓储
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        SqliteOrderRepository { pool }
    }
}

#[async_trait::async_trait]
impl OrderRepository for SqliteOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), DomainError> {
        let items_json = serde_json::to_string(order.items())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let tags_json = serde_json::to_string(order.tags().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let metadata_json = serde_json::to_string(order.metadata().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;

        let status_str = match order.status() {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Completed => "completed",
            OrderStatus::Cancelled => "cancelled",
        };

        sqlx::query(
            r#"
            INSERT INTO orders (id, user_id, status, items, discount_rate, total_amount, is_active, tags, metadata, created_at, note)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(order.id().as_uuid())
        .bind(order.user_id().as_uuid())
        .bind(status_str)
        .bind(items_json)
        .bind(order.discount_rate().value())
        .bind(order.total_amount().cents())
        .bind(order.is_active().value())
        .bind(tags_json)
        .bind(metadata_json)
        .bind(order.created_at().datetime())
        .bind(order.note().as_option())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, DomainError> {
        let row = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT id, user_id, status, items, discount_rate, total_amount, is_active, tags, metadata, created_at, note
            FROM orders WHERE id = ?
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(row.map(|r| r.into_order()).transpose()?)
    }

    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Order>, DomainError> {
        let rows = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT id, user_id, status, items, discount_rate, total_amount, is_active, tags, metadata, created_at, note
            FROM orders WHERE user_id = ?
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        let orders: Result<Vec<_>, _> = rows.into_iter().map(|r| r.into_order()).collect();
        orders
    }

    async fn find_all(&self) -> Result<Vec<Order>, DomainError> {
        let rows = sqlx::query_as::<_, OrderRow>(
            r#"
            SELECT id, user_id, status, items, discount_rate, total_amount, is_active, tags, metadata, created_at, note
            FROM orders
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        let orders: Result<Vec<_>, _> = rows.into_iter().map(|r| r.into_order()).collect();
        orders
    }

    async fn delete(&self, id: OrderId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM orders WHERE id = ?")
            .bind(id.as_uuid())
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;
        Ok(())
    }

    async fn update(&self, order: &Order) -> Result<(), DomainError> {
        let items_json = serde_json::to_string(order.items())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let tags_json = serde_json::to_string(order.tags().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;
        let metadata_json = serde_json::to_string(order.metadata().all())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("序列化失败: {}", e)))?;

        let status_str = match order.status() {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Completed => "completed",
            OrderStatus::Cancelled => "cancelled",
        };

        sqlx::query(
            r#"
            UPDATE orders
            SET status = ?, items = ?, discount_rate = ?, total_amount = ?, is_active = ?, tags = ?, metadata = ?, note = ?
            WHERE id = ?
            "#,
        )
        .bind(status_str)
        .bind(items_json)
        .bind(order.discount_rate().value())
        .bind(order.total_amount().cents())
        .bind(order.is_active().value())
        .bind(tags_json)
        .bind(metadata_json)
        .bind(order.note().as_option())
        .bind(order.id().as_uuid())
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::BusinessRuleViolation(format!("数据库错误: {}", e)))?;

        Ok(())
    }
}

/// 订单行记录
#[derive(sqlx::FromRow)]
struct OrderRow {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    status: String,
    items: String,
    discount_rate: f32,
    total_amount: i64,
    is_active: bool,
    tags: String,
    metadata: String,
    created_at: DateTime<Utc>,
    note: Option<String>,
}

impl OrderRow {
    fn into_order(self) -> Result<Order, DomainError> {
        let items_vec: Vec<OrderItem> = serde_json::from_str(&self.items)
            .map_err(|e| DomainError::BusinessRuleViolation(format!("反序列化失败: {}", e)))?;
        let tags_vec: Vec<String> = serde_json::from_str(&self.tags)
            .map_err(|e| DomainError::BusinessRuleViolation(format!("反序列化失败: {}", e)))?;
        let metadata_map: HashMap<String, String> = serde_json::from_str(&self.metadata)
            .map_err(|e| DomainError::BusinessRuleViolation(format!("反序列化失败: {}", e)))?;

        let status = match self.status.as_str() {
            "pending" => OrderStatus::Pending,
            "paid" => OrderStatus::Paid,
            "shipped" => OrderStatus::Shipped,
            "completed" => OrderStatus::Completed,
            "cancelled" => OrderStatus::Cancelled,
            _ => return Err(DomainError::BusinessRuleViolation("无效的订单状态".into())),
        };

        Ok(Order::reconstruct(
            OrderId::from_uuid(self.id),
            UserId::from_uuid(self.user_id),
            status,
            items_vec,
            DiscountRate::new(self.discount_rate)?,
            Money::from_cents(self.total_amount)?,
            IsActive::new(self.is_active),
            Tags::new(tags_vec),
            Metadata::from_hashmap(metadata_map),
            CreatedAt::from_datetime(self.created_at),
            self.note
                .map(OptionalNote::some)
                .unwrap_or_else(OptionalNote::none),
        ))
    }
}
