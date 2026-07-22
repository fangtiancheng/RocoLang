use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, IceCrystalBattleInfo, battle_index, fight_id);
    register_getters!(
        engine,
        IceCrystalBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money
    );
    register_getters!(
        engine,
        IceCrystalRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        IceCrystalInfo,
        result_code,
        message,
        request_context,
        progress,
        battle_times,
        battle_index,
        get_times,
        add,
        current_battle
    );
    engine.register_get("item_counts", |value: &mut IceCrystalInfo| {
        to_array(&value.item_counts)
    });
    engine.register_get("crystal_counts", |value: &mut IceCrystalInfo| {
        to_array(&value.crystal_counts)
    });
    engine.register_get("item_costs", |value: &mut IceCrystalInfo| {
        to_array(&value.item_costs)
    });
    engine.register_get("one_key_diamond_costs", |value: &mut IceCrystalInfo| {
        to_array(&value.one_key_diamond_costs)
    });
    engine.register_get("bag_candidates", |value: &mut IceCrystalInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("rewards", |value: &mut IceCrystalInfo| {
        to_array(&value.rewards)
    });
}
