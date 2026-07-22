use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTearRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTearInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub buy: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub count_down: RocoOptionalI64,
    pub tear_state: RocoOptionalI64,
    pub rewards: Vec<DiamondTearRewardItem>,
}
