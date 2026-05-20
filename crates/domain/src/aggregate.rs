//! 聚合根模块
//!
//! 包含业务聚合根的定义，采用DDD充血模型

use crate::error::DomainError;
use crate::value_object::*;
use std::vec::Vec;

// ==================== 用户聚合根 ====================

/// 用户聚合根
///
/// 包含用户的核心信息和业务方法。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    /// 用户ID
    id: UserId,
    /// 用户名
    name: UserName,
    /// 邮箱
    email: Email,
    /// 年龄（可选）
    age: Option<Age>,
    /// 评分（可选）
    rating: Option<Rating>,
    /// 是否激活
    is_active: IsActive,
    /// 标签
    tags: Tags,
    /// 元数据
    metadata: Metadata,
    /// 创建时间
    created_at: CreatedAt,
    /// 备注（可选）
    note: OptionalNote,
}

impl User {
    /// 创建新用户
    ///
    /// # 参数
    /// * `name` - 用户名
    /// * `email` - 邮箱
    ///
    /// # 返回
    /// 成功返回User，失败返回DomainError
    pub fn new(name: UserName, email: Email) -> Result<Self, DomainError> {
        Ok(User {
            id: UserId::generate(),
            name,
            email,
            age: None,
            rating: None,
            is_active: IsActive::new(true),
            tags: Tags::new(Vec::new()),
            metadata: Metadata::new(),
            created_at: CreatedAt::now(),
            note: OptionalNote::none(),
        })
    }

    /// 重建用户（从数据库加载时使用）
    pub fn reconstruct(
        id: UserId,
        name: UserName,
        email: Email,
        age: Option<Age>,
        rating: Option<Rating>,
        is_active: IsActive,
        tags: Tags,
        metadata: Metadata,
        created_at: CreatedAt,
        note: OptionalNote,
    ) -> Self {
        User {
            id,
            name,
            email,
            age,
            rating,
            is_active,
            tags,
            metadata,
            created_at,
            note,
        }
    }

    /// 获取用户ID
    pub fn id(&self) -> UserId {
        self.id
    }

    /// 获取用户名
    pub fn name(&self) -> &UserName {
        &self.name
    }

    /// 获取邮箱
    pub fn email(&self) -> &Email {
        &self.email
    }

    /// 获取年龄
    pub fn age(&self) -> Option<Age> {
        self.age
    }

    /// 获取评分
    pub fn rating(&self) -> Option<Rating> {
        self.rating
    }

    /// 获取激活状态
    pub fn is_active(&self) -> IsActive {
        self.is_active
    }

    /// 获取标签
    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    /// 获取元数据
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// 获取创建时间
    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    /// 获取备注
    pub fn note(&self) -> &OptionalNote {
        &self.note
    }

    /// 更新用户名
    pub fn update_name(&mut self, new_name: UserName) {
        self.name = new_name;
    }

    /// 更新邮箱
    pub fn update_email(&mut self, new_email: Email) {
        self.email = new_email;
    }

    /// 设置年龄
    pub fn set_age(&mut self, age: Age) {
        self.age = Some(age);
    }

    /// 设置评分
    pub fn set_rating(&mut self, rating: Rating) {
        self.rating = Some(rating);
    }

    /// 激活用户
    pub fn activate(&mut self) {
        self.is_active = IsActive::new(true);
    }

    /// 禁用用户
    pub fn deactivate(&mut self) {
        self.is_active = IsActive::new(false);
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        self.tags.add(tag);
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.set(key, value);
    }

    /// 设置备注
    pub fn set_note(&mut self, note: String) {
        self.note = OptionalNote::some(note);
    }

    /// 清除备注
    pub fn clear_note(&mut self) {
        self.note = OptionalNote::none();
    }
}

// ==================== 订单聚合根 ====================

/// 订单聚合根
///
/// 包含订单的核心信息和业务方法。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Order {
    /// 订单ID
    id: OrderId,
    /// 用户ID
    user_id: UserId,
    /// 订单状态
    status: OrderStatus,
    /// 订单项列表
    items: Vec<OrderItem>,
    /// 折扣率
    discount_rate: DiscountRate,
    /// 订单总金额
    total_amount: Money,
    /// 是否激活
    is_active: IsActive,
    /// 标签
    tags: Tags,
    /// 元数据
    metadata: Metadata,
    /// 创建时间
    created_at: CreatedAt,
    /// 备注（可选）
    note: OptionalNote,
}

impl Order {
    /// 创建新订单
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回
    /// 成功返回Order，失败返回DomainError
    pub fn new(user_id: UserId) -> Result<Self, DomainError> {
        Ok(Order {
            id: OrderId::generate(),
            user_id,
            status: OrderStatus::Pending,
            items: Vec::new(),
            discount_rate: DiscountRate::new(0.0)?,
            total_amount: Money::from_cents(0)?,
            is_active: IsActive::new(true),
            tags: Tags::new(Vec::new()),
            metadata: Metadata::new(),
            created_at: CreatedAt::now(),
            note: OptionalNote::none(),
        })
    }

    /// 重建订单（从数据库加载时使用）
    pub fn reconstruct(
        id: OrderId,
        user_id: UserId,
        status: OrderStatus,
        items: Vec<OrderItem>,
        discount_rate: DiscountRate,
        total_amount: Money,
        is_active: IsActive,
        tags: Tags,
        metadata: Metadata,
        created_at: CreatedAt,
        note: OptionalNote,
    ) -> Self {
        Order {
            id,
            user_id,
            status,
            items,
            discount_rate,
            total_amount,
            is_active,
            tags,
            metadata,
            created_at,
            note,
        }
    }

    /// 获取订单ID
    pub fn id(&self) -> OrderId {
        self.id
    }

    /// 获取用户ID
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// 获取订单状态
    pub fn status(&self) -> OrderStatus {
        self.status
    }

    /// 获取订单项
    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    /// 获取折扣率
    pub fn discount_rate(&self) -> DiscountRate {
        self.discount_rate
    }

    /// 获取订单总金额
    pub fn total_amount(&self) -> Money {
        self.total_amount
    }

    /// 获取激活状态
    pub fn is_active(&self) -> IsActive {
        self.is_active
    }

    /// 获取标签
    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    /// 获取元数据
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// 获取创建时间
    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    /// 获取备注
    pub fn note(&self) -> &OptionalNote {
        &self.note
    }

    /// 添加订单项
    pub fn add_item(&mut self, item: OrderItem) {
        self.items.push(item);
        self.recalculate_total();
    }

    /// 移除订单项
    pub fn remove_item(&mut self, index: usize) -> Result<(), DomainError> {
        if index >= self.items.len() {
            return Err(DomainError::BusinessRuleViolation("订单项索引无效".into()));
        }
        self.items.remove(index);
        self.recalculate_total();
        Ok(())
    }

    /// 设置折扣率
    pub fn set_discount_rate(&mut self, rate: DiscountRate) {
        self.discount_rate = rate;
        self.recalculate_total();
    }

    /// 计算订单总金额（内部方法）
    fn recalculate_total(&mut self) {
        let subtotal = self
            .items
            .iter()
            .fold(Money::from_cents(0).unwrap(), |acc, item| {
                acc.add(item.subtotal())
            });

        let discount = (subtotal.cents() as f64 * self.discount_rate.value() as f64).round() as i64;
        let total = subtotal.cents() - discount;

        self.total_amount = Money::from_cents(total).unwrap();
    }

    /// 支付订单
    pub fn pay(&mut self) -> Result<(), DomainError> {
        match self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Paid;
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation(
                "只有待支付订单可以支付".into(),
            )),
        }
    }

    /// 发货
    pub fn ship(&mut self) -> Result<(), DomainError> {
        match self.status {
            OrderStatus::Paid => {
                self.status = OrderStatus::Shipped;
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation(
                "只有已支付订单可以发货".into(),
            )),
        }
    }

    /// 完成订单
    pub fn complete(&mut self) -> Result<(), DomainError> {
        match self.status {
            OrderStatus::Shipped => {
                self.status = OrderStatus::Completed;
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation(
                "只有已发货订单可以完成".into(),
            )),
        }
    }

    /// 取消订单
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        match self.status {
            OrderStatus::Pending | OrderStatus::Paid => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation(
                "只能取消待支付或已支付订单".into(),
            )),
        }
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        self.tags.add(tag);
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.set(key, value);
    }

    /// 设置备注
    pub fn set_note(&mut self, note: String) {
        self.note = OptionalNote::some(note);
    }
}
