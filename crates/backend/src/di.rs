//! 依赖注入配置
//!
//! 配置应用服务与仓储的依赖关系，实现依赖倒置原则。
//!
//! 本项目使用 Rust 类型系统实现手动依赖注入，无需额外依赖注入框架。

use application::service::{OrderService, UserService};
use database::{SqliteManager, SqliteOrderRepository, SqliteUserRepository};
use std::sync::Arc;

/// 应用状态
///
/// 包含所有需要在请求处理器间共享的服务实例。
#[derive(Clone)]
pub struct AppState {
    /// 用户服务
    pub user_service: UserService,
    /// 订单服务
    pub order_service: OrderService,
    /// 数据库管理器
    pub db_manager: SqliteManager,
}

/// 创建应用状态
///
/// 初始化所有服务和仓储的依赖关系。
pub async fn create_app_state() -> AppState {
    // 初始化数据库管理器
    let db_manager = SqliteManager::new("sqlite:./data/app.db?mode=rwc")
        .await
        .expect("无法连接到数据库");

    // 执行数据库迁移
    db_manager
        .migrate()
        .await
        .expect("无法执行数据库迁移");

    // 创建仓储实例
    let user_repo = Arc::new(SqliteUserRepository::new(Arc::clone(&db_manager.pool)));
    let order_repo = Arc::new(SqliteOrderRepository::new(Arc::clone(&db_manager.pool)));

    // 创建应用服务
    let user_service = UserService::new(Arc::clone(&user_repo) as Arc<_>);
    let order_service = OrderService::new(
        Arc::clone(&order_repo) as Arc<_>,
        Arc::clone(&user_repo) as Arc<_>,
    );

    AppState {
        user_service,
        order_service,
        db_manager,
    }
}
