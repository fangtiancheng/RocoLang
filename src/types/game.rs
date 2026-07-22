use serde::{Deserialize, Serialize};

use crate::{RocoError, RocoErrorInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameRewardItem {
    pub id: i64,
    pub count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameExtraField {
    pub key: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameSubmitResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub game_id: i64,
    pub score: i64,
    pub game_type: i64,
    pub items: Vec<MiniGameRewardItem>,
    pub extra_fields: Vec<MiniGameExtraField>,
}

impl MiniGameSubmitResult {
    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            game_id: 0,
            score: 0,
            game_type: 0,
            items: Vec::new(),
            extra_fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameSubmitTryResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: MiniGameSubmitResult,
}

impl MiniGameSubmitTryResult {
    pub const CODE_NETWORK_ERROR: i64 = 1001;

    pub fn ok(result: MiniGameSubmitResult) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self::failed_with_code(2, message)
    }

    pub fn network_error_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self::failed_with_code_and_error(Self::CODE_NETWORK_ERROR, message, Some(error.info()))
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self::failed_with_code_and_error(2, message, Some(error.info()))
    }

    fn failed_with_code(code: i64, message: impl Into<String>) -> Self {
        Self::failed_with_code_and_error(code, message, None)
    }

    fn failed_with_code_and_error(
        code: i64,
        message: impl Into<String>,
        error: Option<RocoErrorInfo>,
    ) -> Self {
        let message = message.into();
        Self {
            ok: false,
            code,
            error,
            result: MiniGameSubmitResult::failed(message.clone()),
            message,
        }
    }
}
