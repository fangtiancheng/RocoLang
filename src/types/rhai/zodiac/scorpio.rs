use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ScorpioField, name, value);
    register_getters!(engine, ScorpioCounter, name, current, limit);
    register_getters!(
        engine,
        ScorpioReward,
        reward_index,
        reward_type,
        reward_count
    );
    register_getters!(
        engine,
        ScorpioBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<ScorpioFirstInfo>("ScorpioFirstInfo");
    register_getters!(
        engine,
        ScorpioFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioFirstInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioFirstInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<ScorpioSecondInfo>("ScorpioSecondInfo");
    register_getters!(
        engine,
        ScorpioSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioSecondInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioSecondInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<ScorpioThirdInfo>("ScorpioThirdInfo");
    register_getters!(
        engine,
        ScorpioThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioThirdInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioThirdInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioThirdInfo| {
        to_array(&value.bag_candidates)
    });
}
