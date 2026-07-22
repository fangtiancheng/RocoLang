use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, GeminiField, name, value);
    register_getters!(engine, GeminiCounter, name, current, limit);
    register_getters!(
        engine,
        GeminiRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        GeminiBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_gemini_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("scores", |value: &mut $ty| to_array(&value.scores));
            engine.register_get("sun_scores", |value: &mut $ty| to_array(&value.sun_scores));
            engine.register_get("moon_scores", |value: &mut $ty| {
                to_array(&value.moon_scores)
            });
            engine.register_get("rewards", |value: &mut $ty| to_array(&value.rewards));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }
    register_gemini_info!(GeminiFirstInfo, "GeminiFirstInfo");
    register_gemini_info!(GeminiSecondInfo, "GeminiSecondInfo");
    register_gemini_info!(GeminiThirdInfo, "GeminiThirdInfo");
}
