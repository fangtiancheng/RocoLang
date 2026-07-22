use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}
