use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, TaurusField, name, value);
    register_getters!(engine, TaurusCounter, name, current, limit);
    register_getters!(
        engine,
        TaurusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_taurus_info {
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
        };
    }
    register_taurus_info!(TaurusFirstInfo, "TaurusFirstInfo");
    register_taurus_info!(TaurusSecondInfo, "TaurusSecondInfo");
    register_taurus_info!(TaurusThirdInfo, "TaurusThirdInfo");
}
