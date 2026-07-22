use serde::{Deserialize, Serialize};

use crate::{RocoError, RocoErrorInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub uin: i64,
    pub id: i64,
    pub nick_name: String,
    pub level: i64,
    pub is_vip: bool,
    pub vip_level: i64,
    pub vip_expiring_days: i64,
    pub vip_lulu: i64,
    pub trainer_level: i64,
    pub trainer_exp: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTimeInfo {
    pub stamp: i64,
    pub full_year: i64,
    pub month: i64,
    pub date: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub day: i64,
    pub day_of_year: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTimeResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: ServerTimeInfo,
}

impl ServerTimeResult {
    pub fn ok(result: ServerTimeInfo) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
            result: ServerTimeInfo::default(),
        }
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self {
            ok: false,
            code: 2,
            message,
            error: Some(error.info()),
            result: ServerTimeInfo::default(),
        }
    }
}
