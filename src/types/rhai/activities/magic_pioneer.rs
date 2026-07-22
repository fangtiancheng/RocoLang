use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, MagicPioneerField, name);
    engine.register_get("values", |value: &mut MagicPioneerField| {
        to_array(&value.values)
    });
    register_getters!(
        engine,
        MagicPioneerRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        MagicPioneerInfo,
        pet,
        cmd,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut MagicPioneerInfo| {
        to_array(&value.fields)
    });
    engine.register_get("rewards", |value: &mut MagicPioneerInfo| {
        to_array(&value.rewards)
    });
}
