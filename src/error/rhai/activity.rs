use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<ScriptActivityName>("ScriptActivityName");
    engine.register_type_with_name::<ScriptActivityOptionField>("ScriptActivityOptionField");
    engine.register_type_with_name::<ScriptActivityOperationError>("ScriptActivityOperationError");
    engine.register_get("code", |value: &mut ScriptActivityName| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut ScriptActivityOptionField| {
        value.code().to_string()
    });
    engine.register_get("kind_code", |value: &mut ScriptActivityOperationError| {
        value.kind_code()
    });
    engine.register_get(
        "activity_code",
        |value: &mut ScriptActivityOperationError| value.activity_code(),
    );
    engine.register_get("field_code", |value: &mut ScriptActivityOperationError| {
        value.field_code()
    });
    engine.register_get("count", |value: &mut ScriptActivityOperationError| {
        value.count()
    });
    engine.register_get("limit", |value: &mut ScriptActivityOperationError| {
        value.limit()
    });
    engine.register_get("value", |value: &mut ScriptActivityOperationError| {
        value.value()
    });
}
