use super::super::to_array;
use crate::types::{AdventureItem, AdventureRewards, AdventureStatus};
use rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, AdventureItem, item_id, count);
    register_getters!(
        engine,
        AdventureStatus,
        last_point,
        got_daily,
        auto_running,
        vip,
        guide_level,
        medal_bits,
        first
    );
    engine.register_get("props", |value: &mut AdventureStatus| value.props.clone());
    engine.register_get("items", |value: &mut AdventureRewards| {
        to_array(&value.items)
    });
}
