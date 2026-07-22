use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}
