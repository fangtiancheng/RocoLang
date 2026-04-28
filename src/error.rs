//! 错误类型定义

use std::fmt;

pub type Result<T> = std::result::Result<T, RocoError>;

/// RocoLang 错误类型
#[derive(Debug, Clone)]
pub enum RocoError {
    /// 脚本执行错误
    ScriptError(String),

    /// 标准库函数调用错误
    StdLibError(String),

    /// 网络错误
    NetworkError(String),

    /// 超时错误
    TimeoutError(String),

    /// 断言失败
    AssertionError(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for RocoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocoError::ScriptError(msg) => write!(f, "Script error: {}", msg),
            RocoError::StdLibError(msg) => write!(f, "StdLib error: {}", msg),
            RocoError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RocoError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            RocoError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
            RocoError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RocoError {}

impl From<String> for RocoError {
    fn from(s: String) -> Self {
        RocoError::Other(s)
    }
}

impl From<&str> for RocoError {
    fn from(s: &str) -> Self {
        RocoError::Other(s.to_string())
    }
}
