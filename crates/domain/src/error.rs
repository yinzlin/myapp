//! 领域错误枚举
//!
//! 定义领域层的错误类型

use thiserror::Error;

/// 领域错误
#[derive(Debug, Error)]
pub enum DomainError {
    /// 无效的名称
    #[error("无效的名称")]
    InvalidName,

    /// 无效的邮箱
    #[error("无效的邮箱地址")]
    InvalidEmail,

    /// 实体未找到
    #[error("实体未找到")]
    NotFound,

    /// 业务规则违反
    #[error("业务规则违反: {0}")]
    BusinessRuleViolation(String),
}
