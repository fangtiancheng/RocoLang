use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<ScriptSpiritOperationError>("ScriptSpiritOperationError");
    engine.register_get("kind_code", |value: &mut ScriptSpiritOperationError| {
        value.kind_code()
    });
    engine.register_get("spirit_id", |value: &mut ScriptSpiritOperationError| {
        value.spirit_id()
    });
    engine.register_get("catch_time", |value: &mut ScriptSpiritOperationError| {
        value.catch_time()
    });
}
