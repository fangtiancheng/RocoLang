use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptRuntimeErrorValue {
    Message {
        message: String,
    },
    RocoTypeJsonSerialize {
        type_name: String,
        source: Box<RocoJsonErrorInfo>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoJsonErrorInfo {
    pub category: RocoJsonErrorCategory,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoJsonErrorCategory {
    Io,
    Syntax,
    Data,
    Eof,
}

impl RocoScriptRuntimeErrorValue {
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message {
            message: message.into(),
        }
    }

    pub fn roco_type_json_serialize<T>(error: &serde_json::Error) -> Self {
        Self::RocoTypeJsonSerialize {
            type_name: std::any::type_name::<T>().to_string(),
            source: Box::new(RocoJsonErrorInfo::from_serde_json_error(error)),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Message { .. } => "message",
            Self::RocoTypeJsonSerialize { .. } => "roco_type_json_serialize",
        }
    }

    pub fn message_text(&self) -> String {
        match self {
            Self::Message { message } => message.clone(),
            Self::RocoTypeJsonSerialize { type_name, source } => {
                format!(
                    "failed to serialize roco type {type_name} as JSON [{} at {}:{}]: {}",
                    source.category.code(),
                    source.line,
                    source.column,
                    source.message
                )
            }
        }
    }

    pub fn json_error_category_code(&self) -> String {
        match self {
            Self::RocoTypeJsonSerialize { source, .. } => source.category.code().to_string(),
            Self::Message { .. } => String::new(),
        }
    }

    pub fn roco_type_name(&self) -> String {
        match self {
            Self::RocoTypeJsonSerialize { type_name, .. } => type_name.clone(),
            Self::Message { .. } => String::new(),
        }
    }
}

impl RocoJsonErrorInfo {
    fn from_serde_json_error(error: &serde_json::Error) -> Self {
        Self {
            category: RocoJsonErrorCategory::from_serde_json_category(error.classify()),
            line: error.line(),
            column: error.column(),
            message: error.to_string(),
        }
    }
}

impl RocoJsonErrorCategory {
    fn from_serde_json_category(category: serde_json::error::Category) -> Self {
        match category {
            serde_json::error::Category::Io => Self::Io,
            serde_json::error::Category::Syntax => Self::Syntax,
            serde_json::error::Category::Data => Self::Data,
            serde_json::error::Category::Eof => Self::Eof,
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::Io => "io",
            Self::Syntax => "syntax",
            Self::Data => "data",
            Self::Eof => "eof",
        }
    }
}
