//! 路由定义
//! 
//! 定义后端 API 的路由结构和端点映射。

// 导入 Axum 路由器类型
use axum::Router;

/// 创建 API 路由
/// 
/// 构建所有 API 端点的路由树。
pub fn create_routes() -> Router {
    // 创建空路由（后续添加具体端点）
    Router::new()
}
