//! 请求处理器
//!
//! 定义 HTTP 请求的处理函数，负责接收请求、调用应用服务、返回响应。

use axum::{Json, http::StatusCode};
use serde::Serialize;

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
