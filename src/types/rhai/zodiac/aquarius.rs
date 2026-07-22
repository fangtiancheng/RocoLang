use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, AquariusField, name, value);
    register_getters!(engine, AquariusCounter, name, current, limit);
    register_getters!(
        engine,
        AquariusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        AquariusRewardItem,
        item_index,
        item_id,
        count,
        item_type
    );

    macro_rules! register_aquarius_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("item_counts", |value: &mut $ty| {
                to_array(&value.item_counts)
            });
            engine.register_get("states", |value: &mut $ty| to_array(&value.states));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
            engine.register_get("reward_items", |value: &mut $ty| {
                to_array(&value.reward_items)
            });
        };
    }
    register_aquarius_info!(AquariusFirstInfo, "AquariusFirstInfo");
    register_aquarius_info!(AquariusSecondInfo, "AquariusSecondInfo");
    register_aquarius_info!(AquariusThirdInfo, "AquariusThirdInfo");
    register_getters!(
        engine,
        AquariusSecondStatusInfo,
        result_code,
        message,
        light_num,
        tail_num,
        boss_left_hp,
        boss_full_hp,
        left_fight_count,
        add_hit_level,
        today_sum_hit,
        exchange_count0,
        exchange_count1,
    );
    register_getters!(
        engine,
        AquariusSecondExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
