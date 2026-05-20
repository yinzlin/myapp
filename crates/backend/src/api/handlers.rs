//! 请求处理器
//!
//! 定义 HTTP 请求的处理函数，负责接收请求、调用应用服务、返回响应。

use crate::di::AppState;
use application::dto::*;
use application::error::ApplicationError;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

// ==================== 健康检查相关 ====================

/// 组件健康状态结构体
#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    /// API 组件状态
    pub api: String,
    /// 数据库组件状态
    pub database: String,
    /// Redis 组件状态
    pub redis: String,
}

/// 健康检查响应结构体
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 服务名称
    pub service: String,
    /// 当前时间戳
    pub timestamp: String,
    /// 各个组件的健康状态
    pub components: ComponentHealth,
}

/// 健康检查处理器
///
/// 返回服务的健康状态信息。
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    // 获取当前时间
    let timestamp = chrono::Utc::now().to_rfc3339();

    // 构建响应
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "backend".to_string(),
        timestamp,
        components: ComponentHealth {
            api: "healthy".to_string(),
            database: "healthy".to_string(),
            redis: "healthy".to_string(),
        },
    };

    (StatusCode::OK, Json(response))
}

// ==================== 错误响应 ====================

/// 错误响应结构体
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// 错误信息
    pub error: String,
}

/// 转换应用错误为HTTP响应
fn map_error(err: ApplicationError) -> Response {
    match err {
        ApplicationError::Domain(domain_err) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: domain_err.to_string(),
            }),
        )
            .into_response(),
        ApplicationError::Infrastructure(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: msg }),
        )
            .into_response(),
        ApplicationError::Validation(msg) => {
            (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg })).into_response()
        }
    }
}

// ==================== 用户API处理器 ====================

/// 创建用户
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Response {
    match state.user_service.create_user(req).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 获取用户
pub async fn get_user(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match state.user_service.get_user(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 获取所有用户
pub async fn get_all_users(State(state): State<AppState>) -> Response {
    match state.user_service.get_all_users().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 更新用户
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateUserRequest>,
) -> Response {
    match state.user_service.update_user(id, req).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 删除用户
pub async fn delete_user(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match state.user_service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => map_error(err),
    }
}

// ==================== 订单API处理器 ====================

/// 创建订单
pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Response {
    match state.order_service.create_order(req).await {
        Ok(order) => (StatusCode::CREATED, Json(order)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 获取订单
pub async fn get_order(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match state.order_service.get_order(id).await {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 获取用户订单
pub async fn get_user_orders(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Response {
    match state.order_service.get_user_orders(user_id).await {
        Ok(orders) => (StatusCode::OK, Json(orders)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 获取所有订单
pub async fn get_all_orders(State(state): State<AppState>) -> Response {
    match state.order_service.get_all_orders().await {
        Ok(orders) => (StatusCode::OK, Json(orders)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 添加订单项
pub async fn add_order_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AddOrderItemRequest>,
) -> Response {
    match state.order_service.add_order_item(id, req).await {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 设置折扣
pub async fn set_discount(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<SetDiscountRequest>,
) -> Response {
    match state.order_service.set_discount(id, req).await {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 更新订单状态
pub async fn update_order_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateOrderStatusRequest>,
) -> Response {
    match state.order_service.update_order_status(id, req).await {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(err) => map_error(err),
    }
}

/// 删除订单
pub async fn delete_order(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match state.order_service.delete_order(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => map_error(err),
    }
}
