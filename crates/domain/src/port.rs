//! 出站端口模块
//!
//! 定义仓储接口（Trait）

use crate::aggregate::{Order, User};
use crate::error::DomainError;
use crate::value_object::{OrderId, UserId};
use async_trait::async_trait;

// ==================== 用户仓储接口 ====================

/// 用户仓储接口
///
/// 定义用户数据持久化的操作。
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 保存用户
    async fn save(&self, user: &User) -> Result<(), DomainError>;

    /// 根据ID查找用户
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;

    /// 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;

    /// 查找所有用户
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;

    /// 删除用户
    async fn delete(&self, id: UserId) -> Result<(), DomainError>;

    /// 更新用户
    async fn update(&self, user: &User) -> Result<(), DomainError>;
}

// ==================== 订单仓储接口 ====================

/// 订单仓储接口
///
/// 定义订单数据持久化的操作。
#[async_trait]
pub trait OrderRepository: Send + Sync {
    /// 保存订单
    async fn save(&self, order: &Order) -> Result<(), DomainError>;

    /// 根据ID查找订单
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, DomainError>;

    /// 根据用户ID查找订单
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Order>, DomainError>;

    /// 查找所有订单
    async fn find_all(&self) -> Result<Vec<Order>, DomainError>;

    /// 删除订单
    async fn delete(&self, id: OrderId) -> Result<(), DomainError>;

    /// 更新订单
    async fn update(&self, order: &Order) -> Result<(), DomainError>;
}
