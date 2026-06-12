//! 值对象模块
//!
//! 包含业务值对象的定义，覆盖Rust完整数据类型

use crate::error::DomainError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

// ==================== 基础值对象 ====================

/// 用户ID值对象
///
/// 使用Uuid类型，作为用户的唯一标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    /// 生成新的用户ID
    pub fn generate() -> Self {
        UserId(Uuid::new_v4())
    }

    /// 从Uuid创建用户ID
    pub fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    /// 从字符串解析用户ID
    pub fn from_string(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(UserId)
            .map_err(|_| DomainError::BusinessRuleViolation("无效的用户ID格式".into()))
    }

    /// 获取Uuid引用
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

/// 订单ID值对象
///
/// 使用Uuid类型，作为订单的唯一标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OrderId(Uuid);

impl OrderId {
    /// 生成新的订单ID
    pub fn generate() -> Self {
        OrderId(Uuid::new_v4())
    }

    /// 从Uuid创建订单ID
    pub fn from_uuid(uuid: Uuid) -> Self {
        OrderId(uuid)
    }

    /// 从字符串解析订单ID
    pub fn from_string(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(OrderId)
            .map_err(|_| DomainError::BusinessRuleViolation("无效的订单ID格式".into()))
    }

    /// 获取Uuid引用
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

/// 用户名值对象
///
/// 使用String类型，表示用户名称。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserName(String);

impl UserName {
    /// 创建新的用户名
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidName);
        }
        if name.len() > 50 {
            return Err(DomainError::BusinessRuleViolation(
                "用户名不能超过50个字符".into(),
            ));
        }
        Ok(UserName(name))
    }

    /// 获取用户名引用
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 邮箱值对象
///
/// 使用String类型，验证邮箱格式。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Email(String);

impl Email {
    /// 创建新的邮箱
    pub fn new(email: String) -> Result<Self, DomainError> {
        if !email.contains('@') {
            return Err(DomainError::InvalidEmail);
        }
        Ok(Email(email))
    }

    /// 获取邮箱引用
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 年龄值对象
///
/// 使用u8类型，表示用户年龄（0-150）。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Age(u8);

impl Age {
    /// 创建新的年龄
    pub fn new(age: u8) -> Result<Self, DomainError> {
        if age > 150 {
            return Err(DomainError::BusinessRuleViolation(
                "年龄不能超过150岁".into(),
            ));
        }
        Ok(Age(age))
    }

    /// 获取年龄值
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// 金额值对象
///
/// 使用i64类型（以分为单位），表示金额。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Money(i64);

impl Money {
    /// 创建新的金额（单位：分）
    pub fn from_cents(cents: i64) -> Result<Self, DomainError> {
        if cents < 0 {
            return Err(DomainError::BusinessRuleViolation("金额不能为负数".into()));
        }
        Ok(Money(cents))
    }

    /// 从元创建金额
    pub fn from_yuan(yuan: f64) -> Result<Self, DomainError> {
        if yuan < 0.0 {
            return Err(DomainError::BusinessRuleViolation("金额不能为负数".into()));
        }
        let cents = (yuan * 100.0).round() as i64;
        Ok(Money(cents))
    }

    /// 获取金额（单位：分）
    pub fn cents(&self) -> i64 {
        self.0
    }

    /// 获取金额（单位：元）
    pub fn yuan(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// 金额加法
    pub fn add_money(self, other: Self) -> Self {
        Money(self.0 + other.0)
    }
}

/// 数量值对象
///
/// 使用u32类型，表示商品数量。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Quantity(u32);

impl Quantity {
    /// 创建新的数量
    pub fn new(qty: u32) -> Result<Self, DomainError> {
        if qty == 0 {
            return Err(DomainError::BusinessRuleViolation("数量必须大于0".into()));
        }
        Ok(Quantity(qty))
    }

    /// 获取数量值
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// 折扣率值对象
///
/// 使用f32类型，表示折扣率（0.0-1.0）。
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountRate(f32);

impl DiscountRate {
    /// 创建新的折扣率
    pub fn new(rate: f32) -> Result<Self, DomainError> {
        if !(0.0..=1.0).contains(&rate) {
            return Err(DomainError::BusinessRuleViolation(
                "折扣率必须在0.0到1.0之间".into(),
            ));
        }
        Ok(DiscountRate(rate))
    }

    /// 获取折扣率值
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// 是否激活值对象
///
/// 使用bool类型，表示用户/订单是否激活。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IsActive(bool);

impl IsActive {
    /// 创建新的激活状态
    pub fn new(active: bool) -> Self {
        IsActive(active)
    }

    /// 获取激活状态
    pub fn value(&self) -> bool {
        self.0
    }
}

/// 评分值对象
///
/// 使用i8类型，表示评分（1-5星）。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Rating(i8);

impl Rating {
    /// 创建新的评分
    pub fn new(rating: i8) -> Result<Self, DomainError> {
        if !(1..=5).contains(&rating) {
            return Err(DomainError::BusinessRuleViolation(
                "评分必须在1到5之间".into(),
            ));
        }
        Ok(Rating(rating))
    }

    /// 获取评分值
    pub fn value(&self) -> i8 {
        self.0
    }
}

/// 订单状态枚举
///
/// 表示订单的不同状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OrderStatus {
    /// 待支付
    Pending,
    /// 已支付
    Paid,
    /// 已发货
    Shipped,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 商品项值对象
///
/// 复合类型，包含商品名称、数量、单价。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OrderItem {
    /// 商品名称
    pub product_name: String,
    /// 商品数量
    pub quantity: Quantity,
    /// 商品单价
    pub unit_price: Money,
}

impl OrderItem {
    /// 创建新的订单项
    pub fn new(product_name: String, quantity: Quantity, unit_price: Money) -> Self {
        OrderItem {
            product_name,
            quantity,
            unit_price,
        }
    }

    /// 计算订单项小计
    pub fn subtotal(&self) -> Money {
        let cents = self.unit_price.cents() * (self.quantity.value() as i64);
        Money::from_cents(cents).unwrap()
    }
}

/// 标签集合值对象
///
/// 使用Vec<String>类型，表示多个标签。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tags(Vec<String>);

impl Tags {
    /// 创建新的标签集合
    pub fn new(tags: Vec<String>) -> Self {
        Tags(tags)
    }

    /// 获取标签引用
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// 获取所有标签
    pub fn all(&self) -> &Vec<String> {
        &self.0
    }

    /// 添加标签
    pub fn add(&mut self, tag: String) {
        if !self.0.contains(&tag) {
            self.0.push(tag);
        }
    }
}

/// 元数据值对象
///
/// 使用HashMap<String, String>类型，表示键值对元数据。
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata(HashMap<String, String>);

impl Metadata {
    /// 创建新的元数据
    pub fn new() -> Self {
        Metadata(HashMap::new())
    }

    /// 设置元数据
    pub fn set(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    /// 获取元数据
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// 获取所有元数据
    pub fn all(&self) -> &HashMap<String, String> {
        &self.0
    }

    /// 从HashMap创建
    pub fn from_hashmap(map: HashMap<String, String>) -> Self {
        Metadata(map)
    }
}

/// 创建时间值对象
///
/// 使用DateTime<Utc>类型，表示创建时间。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CreatedAt(DateTime<Utc>);

impl CreatedAt {
    /// 创建新的创建时间（当前时间）
    pub fn now() -> Self {
        CreatedAt(Utc::now())
    }

    /// 从DateTime创建
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        CreatedAt(dt)
    }

    /// 获取DateTime引用
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

/// 可选备注值对象
///
/// 使用Option<String>类型，表示可选的备注信息。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OptionalNote(Option<String>);

impl OptionalNote {
    /// 创建空备注
    pub fn none() -> Self {
        OptionalNote(None)
    }

    /// 创建带备注
    pub fn some(note: String) -> Self {
        OptionalNote(Some(note))
    }

    /// 获取备注引用
    pub fn as_option(&self) -> &Option<String> {
        &self.0
    }
}
