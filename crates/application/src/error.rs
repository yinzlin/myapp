//! 应用错误枚举
//!
//! 定义应用层的错误类型，用于封装用例执行过程中的错误。

// 导入错误处理宏
use thiserror::Error;

/// 应用错误枚举
///
/// 表示应用层中可能发生的错误，涵盖领域错误和基础设施错误。
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// 领域错误
    ///
    /// 由领域层抛出的业务逻辑错误，自动从 DomainError 转换。
    #[error("领域错误: {0}")]
    Domain(#[from] domain::error::DomainError),

    /// 基础设施错误
    ///
    /// 由数据库、网络等基础设施层抛出的错误。
    #[error("基础设施错误: {0}")]
    Infrastructure(String),

    /// 验证错误
    ///
    /// 由 DTO 验证失败产生的错误。
    #[error("验证错误: {0}")]
    Validation(String),
}
