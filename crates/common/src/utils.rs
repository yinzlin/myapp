//! 工具函数

use crate::types::Timestamp;
use chrono::Utc;

/// 获取当前时间戳
pub fn now() -> Timestamp {
    Utc::now()
}
