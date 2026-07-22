use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        UnicornRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        UnicornBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        UnicornBossInfo,
        slot,
        npc_index,
        spirit_id,
        fight_id
    );
    register_getters!(
        engine,
        UnicornInfo,
        result_code,
        message,
        request_context,
        finish,
        start,
        total,
        book,
        purple_vine_count,
        energy,
        fruit_count,
        increase
    );
    engine.register_get("bosses", |value: &mut UnicornInfo| to_array(&value.bosses));
    engine.register_get("cultivation_times", |value: &mut UnicornInfo| {
        to_array(&value.cultivation_times)
    });
    engine.register_get("evolution_energy_costs", |value: &mut UnicornInfo| {
        to_array(&value.evolution_energy_costs)
    });
    engine.register_get("one_key_diamond_costs", |value: &mut UnicornInfo| {
        to_array(&value.one_key_diamond_costs)
    });
    engine.register_get("bag_candidates", |value: &mut UnicornInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("rewards", |value: &mut UnicornInfo| {
        to_array(&value.rewards)
    });
}
