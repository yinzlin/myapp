//! 数据库适配器
//!
//! 实现仓储接口，提供数据持久化能力

pub mod postgres;
pub mod redis;
pub mod sqlite;

// 导出核心类型
pub use sqlite::SqliteManager;
pub use sqlite::SqliteOrderRepository;
pub use sqlite::SqliteUserRepository;
