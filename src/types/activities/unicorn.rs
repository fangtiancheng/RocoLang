use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBossInfo {
    pub slot: i64,
    pub npc_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub fight_id: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub bosses: Vec<UnicornBossInfo>,
    pub finish: RocoOptionalI64,
    pub start: RocoOptionalI64,
    pub total: RocoOptionalI64,
    pub book: RocoOptionalI64,
    pub cultivation_times: Vec<i64>,
    pub evolution_energy_costs: Vec<i64>,
    pub one_key_diamond_costs: Vec<i64>,
    pub purple_vine_count: RocoOptionalI64,
    pub energy: RocoOptionalI64,
    pub fruit_count: RocoOptionalI64,
    pub increase: RocoOptionalI64,
    pub bag_candidates: Vec<UnicornBagCandidate>,
    pub rewards: Vec<UnicornRewardItem>,
}
