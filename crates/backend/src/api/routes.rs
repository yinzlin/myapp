//! 路由定义
//!
//! 定义后端 API 的路由结构和端点映射。

// 导入 Axum 路由器类型
use axum::{Router, routing::get};

// 导入处理器
use super::handlers::health_check;

/// 创建 API 路由
///
/// 构建所有 API 端点的路由树。
pub fn create_routes() -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(health_check))
}
