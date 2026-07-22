use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, VirgoField, name, value);
    register_getters!(engine, VirgoCounter, name, current, limit);
    register_getters!(
        engine,
        VirgoPetInfo,
        candidate_index,
        spirit_id,
        has_bag_index,
        bag_index,
        catch_time,
        has_level,
        level,
        has_need_money,
        need_money,
    );

    engine.register_type_with_name::<VirgoServeGodInfo>("VirgoServeGodInfo");
    register_getters!(
        engine,
        VirgoServeGodInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut VirgoServeGodInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut VirgoServeGodInfo| {
        to_array(&value.counters)
    });
    engine.register_get("states", |value: &mut VirgoServeGodInfo| {
        to_array(&value.states)
    });
    engine.register_get("pets", |value: &mut VirgoServeGodInfo| {
        to_array(&value.pets)
    });
    engine.register_type_with_name::<VirgoFindHalidomInfo>("VirgoFindHalidomInfo");
    register_getters!(
        engine,
        VirgoFindHalidomInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut VirgoFindHalidomInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut VirgoFindHalidomInfo| {
        to_array(&value.counters)
    });
    engine.register_get("states", |value: &mut VirgoFindHalidomInfo| {
        to_array(&value.states)
    });
    engine.register_get("pets", |value: &mut VirgoFindHalidomInfo| {
        to_array(&value.pets)
    });
    engine.register_type_with_name::<VirgoBellFoxInfo>("VirgoBellFoxInfo");
    register_getters!(
        engine,
        VirgoBellFoxInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut VirgoBellFoxInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut VirgoBellFoxInfo| {
        to_array(&value.counters)
    });
    engine.register_get("states", |value: &mut VirgoBellFoxInfo| {
        to_array(&value.states)
    });
    engine.register_get("pets", |value: &mut VirgoBellFoxInfo| to_array(&value.pets));
    register_getters!(
        engine,
        VirgoBellFoxStatusInfo,
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
        VirgoBellFoxExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
