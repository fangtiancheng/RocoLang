use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, CancerItemInfo, id, count, item_type);
    register_getters!(engine, CancerPetInfo, id, catch_time, level, need_money);
    register_getters!(
        engine,
        CancerSharpScorpionInfo,
        result_code,
        message,
        request_context,
        light_num,
        tail_num,
        boss_left_hp,
        boss_full_hp,
        left_fight_count,
        add_hit_level,
        today_sum_hit,
        exchange_count0,
        exchange_count1,
        display_item,
    );
    register_getters!(
        engine,
        CancerMendShapeInfo,
        result_code,
        message,
        request_context,
        left_times,
        step,
        complete,
    );
    register_getters!(
        engine,
        CancerMendShapeBagInfo,
        result_code,
        message,
        request_context,
    );
    engine.register_get("pets", |value: &mut CancerMendShapeBagInfo| {
        to_array(&value.pets)
    });
    register_getters!(
        engine,
        CancerUnsealMemoriesInfo,
        result_code,
        message,
        request_context,
        advance,
        level,
        power,
        event,
        pass,
        finish,
        schedule,
    );
    register_getters!(
        engine,
        CancerUnsealMemoriesBagInfo,
        result_code,
        message,
        request_context,
    );
    engine.register_get("pets", |value: &mut CancerUnsealMemoriesBagInfo| {
        to_array(&value.pets)
    });
}
