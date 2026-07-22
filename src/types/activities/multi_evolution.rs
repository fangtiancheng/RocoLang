use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub catch_time: i64,
    pub condition_code: i64,
    pub condition_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub candidates: Vec<MultiEvolutionCandidate>,
    pub rewards: Vec<MultiEvolutionRewardItem>,
    pub pet_id: RocoOptionalI64,
    pub result_side: RocoOptionalI64,
    pub item_id: RocoOptionalI64,
    pub count: i64,
    pub available: bool,
}
