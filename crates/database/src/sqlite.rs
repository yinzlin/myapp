//! SQLite 仓储实现
//!
//! 提供 SQLite 数据库连接池和基础操作。

use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::{info, warn};

/// SQLite 数据库管理器
///
/// 封装 SQLite 连接池，提供数据库操作能力。
#[derive(Debug, Clone)]
pub struct SqliteManager {
    /// SQLite 连接池
    pool: Arc<SqlitePool>,
}

impl SqliteManager {
    /// 创建新的 SQLite 管理器
    ///
    /// # 参数
    /// * `url` - SQLite 数据库连接 URL，格式: `sqlite:./data/app.db`
    ///
    /// # 返回
    /// 成功返回 SqliteManager，失败返回错误
    pub async fn new(url: &str) -> Result<Self> {
        info!("正在连接 SQLite 数据库: {}", url);

        // 创建连接池
        let pool = SqlitePool::connect(url).await?;

        // 验证连接
        let result = sqlx::query("SELECT 1").fetch_one(&pool).await;
        match result {
            Ok(_) => info!("SQLite 连接成功"),
            Err(e) => warn!("SQLite 连接验证失败: {}", e),
        }

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 执行数据库迁移
    ///
    /// 应用 SQLx 迁移脚本到数据库。
    pub async fn migrate(&self) -> Result<()> {
        info!("正在执行 SQLite 数据库迁移");

        sqlx::migrate!("./migrations").run(&*self.pool).await?;

        info!("SQLite 数据库迁移完成");
        Ok(())
    }

    /// 检查数据库健康状态
    pub async fn health_check(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&*self.pool).await {
            Ok(_) => true,
            Err(e) => {
                warn!("SQLite 健康检查失败: {}", e);
                false
            }
        }
    }
}
