use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, SagittariusField, name, value);
    register_getters!(engine, SagittariusCounter, name, current, limit);
    register_getters!(engine, SagittariusScore, score_index, current, limit);
    register_getters!(
        engine,
        SagittariusStarPicture,
        picture_index,
        is_in,
        progress,
        finish
    );
    register_getters!(
        engine,
        SagittariusRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        SagittariusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<SagittariusFirstInfo>("SagittariusFirstInfo");
    register_getters!(
        engine,
        SagittariusFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusFirstInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusFirstInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusFirstInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<SagittariusSecondInfo>("SagittariusSecondInfo");
    register_getters!(
        engine,
        SagittariusSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusSecondInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusSecondInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusSecondInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<SagittariusThirdInfo>("SagittariusThirdInfo");
    register_getters!(
        engine,
        SagittariusThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusThirdInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusThirdInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusThirdInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusThirdInfo| {
        to_array(&value.bag_candidates)
    });
}
