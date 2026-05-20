//! 应用错误枚举
//!
//! 定义应用层的错误类型

use thiserror::Error;

/// 应用错误
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// 领域错误
    #[error("领域错误: {0}")]
    Domain(#[from] domain::error::DomainError),

    /// 基础设施错误
    #[error("基础设施错误: {0}")]
    Infrastructure(String),

    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),
}
