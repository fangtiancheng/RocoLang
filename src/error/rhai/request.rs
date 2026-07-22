use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<ScriptRequestError>("ScriptRequestError");
    engine.register_get("kind_code", |value: &mut ScriptRequestError| {
        value.kind_code()
    });
    engine.register_get("wait_context_code", |value: &mut ScriptRequestError| {
        value.wait_context_code()
    });
    engine.register_get(
        "system_failure_kind_code",
        |value: &mut ScriptRequestError| value.system_failure_kind_code(),
    );
    engine.register_get(
        "combat_intent_kind_code",
        |value: &mut ScriptRequestError| value.combat_intent_kind_code(),
    );
    engine.register_get(
        "combat_validation_kind_code",
        |value: &mut ScriptRequestError| value.combat_validation_kind_code(),
    );
    engine.register_get("spirit_index", |value: &mut ScriptRequestError| {
        value.spirit_index()
    });
    engine.register_get("value", |value: &mut ScriptRequestError| value.value());
    engine.register_get("operation_context", |value: &mut ScriptRequestError| {
        value.operation_context()
    });
}
