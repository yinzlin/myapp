//! Axum 后端应用入口
//!
//! 这是整个后端服务的启动入口，负责初始化日志、路由和中间件。

// 导入 Axum 核心路由器
use axum::Router;
// 导入网络地址类型
use std::net::SocketAddr;
// 导入 CORS 跨域中间件
use tower_http::cors::{Any, CorsLayer};
// 导入 HTTP 请求追踪中间件
use tower_http::trace::TraceLayer;
// 导入日志订阅器
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 模块导入：API 路由定义
mod api;
// 模块导入：依赖注入配置
mod di;
// 模块导入：自定义中间件
mod middleware;

/// 异步主函数，使用 Tokio 运行时
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志订阅器
    tracing_subscriber::registry()
        // 设置日志级别过滤器：优先从环境变量读取，否则使用默认值
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        // 添加格式化输出层
        .with(tracing_subscriber::fmt::layer())
        // 初始化日志系统
        .init();

    // 记录服务启动日志
    tracing::info!("正在启动后端服务...");

    // 创建应用状态
    let app_state = di::create_app_state().await;

    // 创建路由实例
    let app = Router::new()
        // 嵌套 API 路由到 /api 前缀下
        .nest("/api", api::routes::create_routes())
        // 添加应用状态
        .with_state(app_state)
        // 添加 CORS 中间件：允许所有来源和方法
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        // 添加请求追踪中间件：记录 HTTP 请求日志
        .layer(TraceLayer::new_for_http());

    // 定义服务监听地址：0.0.0.0:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // 记录监听地址日志
    tracing::info!("后端服务监听地址: {}", addr);

    // 绑定 TCP 监听端口
    let listener = tokio::net::TcpListener::bind(addr).await?;
    // 启动 Axum HTTP 服务
    axum::serve(listener, app).await?;

    // 返回成功结果
    Ok(())
}
