use serde::{Deserialize, Serialize};

use crate::{RocoError, RocoErrorInfo};

use super::Engine;

/// Standard result shape for operation-style `try_*` APIs.
///
/// `try_*` functions should not raise expected business failures such as
/// unavailable actions or server rejections. They should return `ok = false`
/// with a non-zero `code` and a readable `message`. Programming errors such as
/// invalid argument types may still be raised by the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
}

impl ActionResult {
    pub fn ok() -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
        }
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        Self {
            ok: false,
            code: 2,
            message: error.message(),
            error: Some(error.info()),
        }
    }
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ActionResult, ok, code, message, error);
    register_error_info_getters!(engine, ActionResult);
}
