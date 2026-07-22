use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        MountainSeaBossInfo,
        index,
        boss_type,
        fight_id,
        name,
        status,
    );
    register_getters!(
        engine,
        MountainSeaSoulInfo,
        soul_type,
        boss_type,
        name,
        count
    );
    register_getters!(
        engine,
        MountainSeaInfo,
        result_code,
        message,
        fight_id,
        seal_count,
        success,
    );
    engine.register_get("attrs", |value: &mut MountainSeaInfo| {
        to_array(&value.attrs)
    });
    engine.register_get("bosses", |value: &mut MountainSeaInfo| {
        to_array(&value.bosses)
    });
    engine.register_get("souls", |value: &mut MountainSeaInfo| {
        to_array(&value.souls)
    });
}
