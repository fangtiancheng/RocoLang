use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        MultiEvolutionCandidate,
        candidate_index,
        spirit_id,
        catch_time,
        condition_code,
        condition_name
    );
    register_getters!(
        engine,
        MultiEvolutionRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        MultiEvolutionInfo,
        result_code,
        message,
        request_context,
        pet_id,
        result_side,
        item_id,
        count,
        available
    );
    engine.register_get("candidates", |value: &mut MultiEvolutionInfo| {
        to_array(&value.candidates)
    });
    engine.register_get("rewards", |value: &mut MultiEvolutionInfo| {
        to_array(&value.rewards)
    });
}
