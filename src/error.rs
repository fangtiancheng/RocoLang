//! Error types for RocoLang.

use std::fmt;

pub type Result<T> = std::result::Result<T, RocoError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RocoScriptError {
    pub kind: String,
    pub message: String,
    pub source: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum RocoError {
    ScriptError(RocoScriptError),
    StdLibError(String),
    InvalidParam(String),
    NetworkError(String),
    TimeoutError(String),
    ServerRejected(String),
    AssertionError(String),
    Other(String),
}

impl fmt::Display for RocoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocoError::ScriptError(error) => {
                write!(f, "Script error")?;
                if let Some(source) = &error.source {
                    write!(f, " in {}", source)?;
                }
                if let Some(line) = error.line {
                    write!(f, ":{}", line)?;
                    if let Some(column) = error.column {
                        write!(f, ":{}", column)?;
                    }
                }
                write!(f, ": [{}] {}", error.kind, error.message)
            }
            RocoError::StdLibError(msg) => write!(f, "StdLib error: {}", msg),
            RocoError::InvalidParam(msg) => write!(f, "Invalid parameter: {}", msg),
            RocoError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RocoError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            RocoError::ServerRejected(msg) => write!(f, "Server rejected: {}", msg),
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
