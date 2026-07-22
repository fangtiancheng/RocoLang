use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalDisplayItem, RocoOptionalI64, RocoRequestContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusRewardItem {
    pub item_index: i64,
    pub item_id: i64,
    pub count: i64,
    pub item_type: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub boss_left_hp: i64,
    pub boss_full_hp: i64,
    pub left_fight_count: i64,
    pub add_hit_level: i64,
    pub today_sum_hit: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}
