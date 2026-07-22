use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalDisplayItem, RocoOptionalI64, RocoRequestContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesReward {
    pub reward_index: i64,
    pub reward_type: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub boss_left_hp: i64,
    pub left_fight_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}
