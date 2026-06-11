//! Error types for RocoLang.

use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = std::result::Result<T, RocoError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptErrorKind {
    Parse,
    Runtime,
    FunctionCall,
    Module,
    Terminated,
    Eval,
    Other,
}

impl RocoScriptErrorKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Runtime => "runtime",
            Self::FunctionCall => "function_call",
            Self::Module => "module",
            Self::Terminated => "terminated",
            Self::Eval => "eval",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for RocoScriptErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptPosition {
    Line { line: usize },
    LineColumn { line: usize, column: usize },
}

impl RocoScriptPosition {
    pub const fn line(&self) -> usize {
        match self {
            Self::Line { line } | Self::LineColumn { line, .. } => *line,
        }
    }

    pub const fn column(&self) -> Option<usize> {
        match self {
            Self::Line { .. } => None,
            Self::LineColumn { column, .. } => Some(*column),
        }
    }
}

impl fmt::Display for RocoScriptPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line { line } => write!(f, "{line}"),
            Self::LineColumn { line, column } => write!(f, "{line}:{column}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptLocation {
    Unknown,
    Anonymous {
        position: RocoScriptPosition,
    },
    Source {
        source: String,
        position: RocoScriptPosition,
    },
}

impl RocoScriptLocation {
    pub const fn position(&self) -> Option<&RocoScriptPosition> {
        match self {
            Self::Unknown => None,
            Self::Anonymous { position } | Self::Source { position, .. } => Some(position),
        }
    }

    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Source { source, .. } => Some(source),
            Self::Unknown | Self::Anonymous { .. } => None,
        }
    }

    pub fn parts(&self) -> (Option<&str>, Option<usize>, Option<usize>) {
        match self {
            Self::Unknown => (None, None, None),
            Self::Anonymous { position } => (None, Some(position.line()), position.column()),
            Self::Source { source, position } => (
                Some(source.as_str()),
                Some(position.line()),
                position.column(),
            ),
        }
    }
}

impl fmt::Display for RocoScriptLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => f.write_str("unknown"),
            Self::Anonymous { position } => write!(f, "{position}"),
            Self::Source { source, position } => write!(f, "{source}:{position}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoScriptError {
    pub kind: RocoScriptErrorKind,
    pub message: String,
    pub location: RocoScriptLocation,
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
                match &error.location {
                    RocoScriptLocation::Unknown => {}
                    RocoScriptLocation::Anonymous { position } => write!(f, " at {}", position)?,
                    RocoScriptLocation::Source { source, position } => {
                        write!(f, " in {}:{}", source, position)?
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
