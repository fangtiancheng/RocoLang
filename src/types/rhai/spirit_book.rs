use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, SpiritBookEntry, id, starred, unknown, newed);
    register_getters!(engine, SpiritBookGroup, template_id);
    engine.register_get("spirits", |value: &mut SpiritBookGroup| {
        to_array(&value.spirits)
    });
    register_getters!(
        engine,
        SpiritBookSummary,
        id,
        name,
        is_new,
        has_cover,
        background,
        page_idx,
        spirit_count,
    );
    register_getters!(
        engine,
        SpiritBookInfo,
        id,
        name,
        is_new,
        has_cover,
        background,
        page_idx,
    );
    engine.register_get("groups", |value: &mut SpiritBookInfo| {
        to_array(&value.groups)
    });
    register_getters!(engine, SpiritBookStates, uin, count);
    engine.register_get("states", |value: &mut SpiritBookStates| {
        to_array(&value.states)
    });
    engine.register_fn(
        "is_owned",
        |value: &mut SpiritBookStates, spirit_id: i64| value.spirit_owned(spirit_id),
    );
    register_getters!(engine, SpiritBookSpiritState, spirit_id, state, owned);
}
