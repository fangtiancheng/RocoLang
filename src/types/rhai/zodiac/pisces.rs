use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, PiscesField, name, value);
    register_getters!(engine, PiscesCounter, name, current, limit);
    register_getters!(
        engine,
        PiscesBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_pisces_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("lights", |value: &mut $ty| to_array(&value.lights));
            engine.register_get("exchanges", |value: &mut $ty| to_array(&value.exchanges));
            engine.register_get("fights", |value: &mut $ty| to_array(&value.fights));
            engine.register_get("days", |value: &mut $ty| to_array(&value.days));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }

    register_pisces_info!(PiscesFirstInfo, "PiscesFirstInfo");
    register_pisces_info!(PiscesSecondInfo, "PiscesSecondInfo");
    register_pisces_info!(PiscesThirdInfo, "PiscesThirdInfo");
}
