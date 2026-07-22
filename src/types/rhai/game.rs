use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, MiniGameRewardItem, id, count, item_type);
    register_getters!(engine, MiniGameExtraField, key, value);
    register_getters!(
        engine,
        MiniGameSubmitResult,
        ok,
        code,
        message,
        game_id,
        score,
        game_type
    );
    engine.register_get("items", |value: &mut MiniGameSubmitResult| {
        to_array(&value.items)
    });
    engine.register_get("extra_fields", |value: &mut MiniGameSubmitResult| {
        to_array(&value.extra_fields)
    });
    register_getters!(engine, MiniGameSubmitTryResult, ok, code, message, error);
    register_error_info_getters!(engine, MiniGameSubmitTryResult);
    engine.register_get("result", |value: &mut MiniGameSubmitTryResult| {
        value.result.clone()
    });
}
