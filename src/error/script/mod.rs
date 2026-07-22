use serde::{Deserialize, Serialize};
use std::fmt;

mod eval;
mod location;
mod parse;
mod runtime;

pub use eval::*;
pub use location::*;
pub use parse::*;
pub use runtime::*;

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
pub struct RocoScriptError {
    pub location: RocoScriptLocation,
    pub source: RocoScriptErrorSource,
}

impl RocoScriptError {
    pub const fn kind(&self) -> RocoScriptErrorKind {
        self.source.kind()
    }

    pub fn message(&self) -> String {
        self.source.message()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptErrorSource {
    Parse(RocoScriptParseErrorSource),
    Eval(RocoScriptEvalErrorSource),
}

impl RocoScriptErrorSource {
    pub const fn kind(&self) -> RocoScriptErrorKind {
        match self {
            Self::Parse(_) => RocoScriptErrorKind::Parse,
            Self::Eval(RocoScriptEvalErrorSource::FunctionCall { .. }) => {
                RocoScriptErrorKind::FunctionCall
            }
            Self::Eval(RocoScriptEvalErrorSource::Module { .. }) => RocoScriptErrorKind::Module,
            Self::Eval(RocoScriptEvalErrorSource::Terminated) => RocoScriptErrorKind::Terminated,
            Self::Eval(RocoScriptEvalErrorSource::Runtime { .. }) => RocoScriptErrorKind::Runtime,
            Self::Eval(_) => RocoScriptErrorKind::Eval,
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Self::Parse(source) => source.code(),
            Self::Eval(source) => source.code(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Parse(source) => source.message(),
            Self::Eval(source) => source.message(),
        }
    }
}
