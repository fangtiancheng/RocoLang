use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSourceInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub battle: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub time: RocoOptionalI64,
    pub increase: RocoOptionalI64,
    pub water: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiresWillInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub schedule: RocoOptionalI64,
    pub num: RocoOptionalI64,
    pub fire: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatheSunInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub battle: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub time: RocoOptionalI64,
    pub num: RocoOptionalI64,
    pub act: RocoOptionalI64,
    pub times: RocoOptionalI64,
    pub sun: RocoOptionalI64,
    pub add: RocoOptionalI64,
}
