use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, LibraField, name, value);
    register_getters!(engine, LibraCounter, name, current, limit);
    register_getters!(
        engine,
        LibraBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<LibraFirstInfo>("LibraFirstInfo");
    register_getters!(
        engine,
        LibraFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<LibraSecondInfo>("LibraSecondInfo");
    register_getters!(
        engine,
        LibraSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<LibraThirdInfo>("LibraThirdInfo");
    register_getters!(
        engine,
        LibraThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraThirdInfo| {
        to_array(&value.bag_candidates)
    });
    register_getters!(
        engine,
        LibraThirdStatusInfo,
        result_code,
        message,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
        boss_left_hp,
        left_fight_count,
    );
    register_getters!(
        engine,
        LibraThirdExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
