use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        TreasureRealmInfo,
        result_code,
        message,
        battle,
        battle_id,
        schedule,
        possible,
        time,
        got_box,
    );
    engine.register_get("item_counts", |value: &mut TreasureRealmInfo| {
        to_array(&value.item_counts)
    });
    engine.register_get("commits", |value: &mut TreasureRealmInfo| {
        to_array(&value.commits)
    });
}
