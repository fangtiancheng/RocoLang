use ::rhai::Engine;

use super::super::*;
pub(super) fn register(engine: &mut Engine) {
    engine.register_get("code", |value: &mut ScriptBridgeOperation| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut ScriptBridgeFailureReason| {
        value.code().to_string()
    });
    engine.register_get("message", |value: &mut ScriptBridgeFailureReason| {
        value.message().to_string()
    });
    register_getters!(engine, ScriptBridgeFailure, operation, reason);
    engine.register_get("message", |value: &mut ScriptBridgeFailure| value.message());
    engine.register_get("operation_code", |value: &mut ScriptBridgeFailure| {
        value.operation_code()
    });
    engine.register_get("reason_code", |value: &mut ScriptBridgeFailure| {
        value.reason_code()
    });
    engine.register_get("description", |value: &mut ScriptBridgeFailure| {
        value.description()
    });
    engine.register_get("code", |value: &mut ScriptSystemOperation| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut ScriptSystemFailureSource| {
        value.code().to_string()
    });
    register_getters!(engine, ScriptSystemFailure, operation, source);
    engine.register_get("operation_code", |value: &mut ScriptSystemFailure| {
        value.operation_code()
    });
    engine.register_get("source_code", |value: &mut ScriptSystemFailure| {
        value.source_code()
    });
    engine.register_get("message", |value: &mut ScriptSystemFailure| value.message());
    engine.register_get("description", |value: &mut ScriptSystemFailure| {
        value.description()
    });
}
