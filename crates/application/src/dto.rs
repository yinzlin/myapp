//! DTO 模块
//!
//! 定义请求/响应数据传输对象

use domain::aggregate::{Order, User};
use domain::value_object::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== 用户相关 DTO ====================

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u8>,
    pub rating: Option<i8>,
    pub note: Option<String>,
}

/// 用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
    pub rating: Option<i8>,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub note: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id().as_uuid().to_string(),
            name: user.name().as_str().to_string(),
            email: user.email().as_str().to_string(),
            age: user.age().map(|a| a.value()),
            rating: user.rating().map(|r| r.value()),
            is_active: user.is_active().value(),
            tags: user.tags().all().clone(),
            metadata: user.metadata().all().clone(),
            created_at: user.created_at().datetime().to_rfc3339(),
            note: user.note().as_option().clone(),
        }
    }
}

// ==================== 订单相关 DTO ====================

/// 创建订单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: String,
}

/// 订单项请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price_yuan: f64,
}

/// 添加订单项请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOrderItemRequest {
    pub item: OrderItemRequest,
}

/// 设置折扣请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDiscountRequest {
    pub discount_rate: f32,
}

/// 更新订单状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

/// 订单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub items: Vec<OrderItemResponse>,
    pub discount_rate: f32,
    pub total_amount_yuan: f64,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub note: Option<String>,
}

/// 订单项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price_yuan: f64,
    pub subtotal_yuan: f64,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        let items: Vec<OrderItemResponse> = order
            .items()
            .iter()
            .map(|item| OrderItemResponse {
                product_name: item.product_name.clone(),
                quantity: item.quantity.value(),
                unit_price_yuan: item.unit_price.yuan(),
                subtotal_yuan: item.subtotal().yuan(),
            })
            .collect();

        let status_str = match order.status() {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Completed => "completed",
            OrderStatus::Cancelled => "cancelled",
        }
        .to_string();

        OrderResponse {
            id: order.id().as_uuid().to_string(),
            user_id: order.user_id().as_uuid().to_string(),
            status: status_str,
            items,
            discount_rate: order.discount_rate().value(),
            total_amount_yuan: order.total_amount().yuan(),
            is_active: order.is_active().value(),
            tags: order.tags().all().clone(),
            metadata: order.metadata().all().clone(),
            created_at: order.created_at().datetime().to_rfc3339(),
            note: order.note().as_option().clone(),
        }
    }
}
