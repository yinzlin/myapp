//! 路由定义
//!
//! 定义后端 API 的路由结构和端点映射。

// 导入 Axum 路由器类型
use crate::di::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

// 导入处理器
use super::handlers::*;

/// 创建 API 路由
///
/// 构建所有 API 端点的路由树。
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // 健康检查端点
        .route("/health", get(health_check))
        // 用户API
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        // 订单API
        .route("/orders", post(create_order))
        .route("/orders", get(get_all_orders))
        .route("/orders/{id}", get(get_order))
        .route("/orders/{id}", delete(delete_order))
        .route("/orders/{id}/items", post(add_order_item))
        .route("/orders/{id}/discount", post(set_discount))
        .route("/orders/{id}/status", post(update_order_status))
        .route("/users/{user_id}/orders", get(get_user_orders))
}
