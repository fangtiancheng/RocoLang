use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioReward {
    pub reward_index: i64,
    pub reward_type: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}
