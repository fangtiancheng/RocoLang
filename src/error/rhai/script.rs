use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<RocoScriptErrorSource>("RocoScriptErrorSource");
    engine.register_type_with_name::<RocoScriptParseErrorSource>("RocoScriptParseErrorSource");
    engine.register_type_with_name::<RocoScriptEvalErrorSource>("RocoScriptEvalErrorSource");
    engine.register_type_with_name::<RocoScriptRuntimeErrorValue>("RocoScriptRuntimeErrorValue");
    engine.register_type_with_name::<RocoJsonErrorCategory>("RocoJsonErrorCategory");
    engine.register_get("code", |value: &mut RocoScriptErrorSource| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut RocoScriptParseErrorSource| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut RocoScriptEvalErrorSource| {
        value.code().to_string()
    });
    engine.register_get(
        "runtime_value",
        |value: &mut RocoScriptEvalErrorSource| match value {
            RocoScriptEvalErrorSource::Runtime { value } => Some(value.clone()),
            _ => None,
        },
    );
    engine.register_get("code", |value: &mut RocoScriptRuntimeErrorValue| {
        value.code().to_string()
    });
    engine.register_get("message", |value: &mut RocoScriptRuntimeErrorValue| {
        value.message_text()
    });
    engine.register_get(
        "roco_type_name",
        |value: &mut RocoScriptRuntimeErrorValue| value.roco_type_name(),
    );
    engine.register_get(
        "json_error_category_code",
        |value: &mut RocoScriptRuntimeErrorValue| value.json_error_category_code(),
    );
    engine.register_get("code", |value: &mut RocoJsonErrorCategory| {
        value.code().to_string()
    });
}
