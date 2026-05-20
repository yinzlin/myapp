//! 通用类型定义
//! 
//! 定义项目中各模块共享的基础数据类型。

// 导入日期时间库
use chrono::{DateTime, Utc};
// 导入序列化/反序列化库
use serde::{Deserialize, Serialize};
// 导入 UUID 库
use uuid::Uuid;

/// 通用 ID 类型
///
/// 使用 UUID v4 作为唯一标识符，封装在新类型中以提供类型安全。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    /// 生成新的 ID（使用 UUID v4）
    pub fn generate() -> Self {
        // 创建新的 UUID v4
        Self(Uuid::new_v4())
    }

    /// 从 UUID 字符串解析 ID
    /// 
    /// # 参数
    /// * `s` - UUID 字符串表示
    /// 
    /// # 返回
    /// 成功返回 Id，失败返回 uuid::Error
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        // 解析字符串为 UUID 并包装
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// 获取内部 UUID 值
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

/// Default 实现：默认生成新 ID
impl Default for Id {
    fn default() -> Self {
        Self::generate()
    }
}

/// Display 实现：将 ID 转换为字符串
impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 格式化输出 UUID 字符串
        write!(f, "{}", self.0)
    }
}

/// 时间戳类型
///
/// 使用 UTC 时区的日期时间，确保跨时区一致性。
pub type Timestamp = DateTime<Utc>;
