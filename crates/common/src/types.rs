//! 通用类型定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 通用 ID 类型
///
/// 使用 UUID v4 作为唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    /// 生成新的 ID
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// 从 UUID 字符串解析
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// 获取内部 UUID
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::generate()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 时间戳类型
///
/// 使用 UTC 时区的日期时间
pub type Timestamp = DateTime<Utc>;
