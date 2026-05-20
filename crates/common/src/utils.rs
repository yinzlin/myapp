//! 工具函数
//! 
//! 提供项目中各模块共享的工具函数。

// 导入自定义时间戳类型
use crate::types::Timestamp;
// 导入 UTC 时间模块
use chrono::Utc;

/// 获取当前 UTC 时间戳
/// 
/// # 返回
/// 当前 UTC 时间的 Timestamp 类型
pub fn now() -> Timestamp {
    // 获取当前 UTC 时间
    Utc::now()
}
