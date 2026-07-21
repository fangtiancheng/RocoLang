use serde::{Deserialize, Serialize};

use super::{to_array, Engine, RocoOptionalI64, RocoRewardKind};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        JumpMachineRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        JumpMachineInfo,
        result_code,
        message,
        can_play,
        coin,
        main_pet_id,
        storage_full,
        pet_id
    );
    engine.register_get("rewards", |value: &mut JumpMachineInfo| {
        to_array(&value.rewards)
    });
}
