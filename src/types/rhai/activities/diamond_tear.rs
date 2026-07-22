use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        DiamondTearRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        DiamondTearInfo,
        result_code,
        message,
        request_context,
        buy,
        level,
        count_down,
        tear_state
    );
    engine.register_get("rewards", |value: &mut DiamondTearInfo| {
        to_array(&value.rewards)
    });
}
