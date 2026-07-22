use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ThreeStartersField, name, value);
    register_getters!(engine, ThreeStartersCounter, name, current, limit);
    register_getters!(
        engine,
        ThreeStartersRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        ThreeStartersBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        WaterSourceInfo,
        result_code,
        message,
        request_context,
        battle,
        schedule,
        time,
        increase
    );
    engine.register_get("fields", |value: &mut WaterSourceInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut WaterSourceInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut WaterSourceInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut WaterSourceInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("water", |value: &mut WaterSourceInfo| {
        to_array(&value.water)
    });
    register_getters!(
        engine,
        FiresWillInfo,
        result_code,
        message,
        request_context,
        schedule,
        num
    );
    engine.register_get("fields", |value: &mut FiresWillInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut FiresWillInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut FiresWillInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut FiresWillInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("fire", |value: &mut FiresWillInfo| to_array(&value.fire));
    register_getters!(
        engine,
        BatheSunInfo,
        result_code,
        message,
        request_context,
        battle,
        schedule,
        time,
        num,
        act,
        times,
        sun,
        add
    );
    engine.register_get("fields", |value: &mut BatheSunInfo| to_array(&value.fields));
    engine.register_get("counters", |value: &mut BatheSunInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut BatheSunInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut BatheSunInfo| {
        to_array(&value.bag_candidates)
    });
}
