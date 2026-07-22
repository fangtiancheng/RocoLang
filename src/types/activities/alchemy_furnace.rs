use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyCultivationInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub daytimes: RocoOptionalI64,
    pub finish: RocoOptionalI64,
    pub progress: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyEvoInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub branch_type: RocoOptionalI64,
    pub done: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagingFireInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub vip: RocoOptionalI64,
    pub daytimes: RocoOptionalI64,
    pub required_stone_indexes: Vec<i64>,
    pub progress: Vec<i64>,
    pub finish: RocoOptionalI64,
    pub fusion: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}
