use super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
