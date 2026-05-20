//! 领域错误枚举
//!
//! 定义领域层的错误类型，用于封装业务逻辑错误。

// 导入错误处理宏
use thiserror::Error;

/// 领域错误枚举
/// 
/// 表示领域层中可能发生的业务逻辑错误。
#[derive(Debug, Error)]
pub enum DomainError {
    /// 无效的名称
    /// 
    /// 当名称为空或不符合业务规则时抛出。
    #[error("无效的名称")]
    InvalidName,

    /// 无效的邮箱
    /// 
    /// 当邮箱格式不正确时抛出。
    #[error("无效的邮箱地址")]
    InvalidEmail,

    /// 实体未找到
    /// 
    /// 当根据 ID 查询实体但不存在时抛出。
    #[error("实体未找到")]
    NotFound,

    /// 业务规则违反
    /// 
    /// 当违反特定业务规则时抛出，包含具体错误信息。
    #[error("业务规则违反: {0}")]
    BusinessRuleViolation(String),
}
