use serde::{Deserialize, Serialize};

use super::{RocoOptionalI64, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMachineRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMachineInfo {
    pub result_code: i64,
    pub message: String,
    pub can_play: bool,
    pub coin: i64,
    pub main_pet_id: RocoOptionalI64,
    pub storage_full: bool,
    pub pet_id: RocoOptionalI64,
    pub rewards: Vec<JumpMachineRewardItem>,
}
