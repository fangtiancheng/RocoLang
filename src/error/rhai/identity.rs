use ::rhai::Engine;

use super::super::*;
pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<ScriptCombatRuntimeError>("ScriptCombatRuntimeError");
    engine.register_type_with_name::<RocoReturnCodeKind>("RocoReturnCodeKind");
    engine.register_type_with_name::<RocoReturnCodeRejection>("RocoReturnCodeRejection");
    engine.register_type_with_name::<RocoHttpBusinessRejection>("RocoHttpBusinessRejection");
    engine.register_type_with_name::<ScriptModuleName>("ScriptModuleName");
    engine.register_type_with_name::<ScriptFunctionName>("ScriptFunctionName");
    engine.register_type_with_name::<ScriptHttpResponseName>("ScriptHttpResponseName");
    register_getters!(engine, ScriptHttpResponseName, code);
    engine.register_type_with_name::<ScriptResponseName>("ScriptResponseName");
    register_getters!(engine, ScriptResponseName, code);
    register_getters!(engine, RocoHttpBusinessRejection, message);
    engine.register_get("result_code", |value: &mut RocoHttpBusinessRejection| {
        value.result_code()
    });
    engine.register_get(
        "request_context",
        |value: &mut RocoHttpBusinessRejection| value.request_context.clone(),
    );
    engine.register_get(
        "request_context_raw",
        |value: &mut RocoHttpBusinessRejection| value.request_context(),
    );
    engine.register_get("description", |value: &mut RocoHttpBusinessRejection| {
        value.description()
    });
    register_getters!(engine, RocoReturnCodeRejection, kind, message);
    engine.register_get("code", |value: &mut RocoReturnCodeRejection| {
        i64::from(value.code())
    });
    engine.register_get("kind_code", |value: &mut RocoReturnCodeRejection| {
        value.kind_code()
    });
    engine.register_get("description", |value: &mut RocoReturnCodeRejection| {
        value.description()
    });
    engine.register_get("code", |value: &mut RocoReturnCodeKind| {
        i64::from(value.code())
    });
    engine.register_get("kind_code", |value: &mut RocoReturnCodeKind| {
        value.kind_code()
    });
    engine.register_get("code", |value: &mut ScriptModuleName| {
        value.code().to_string()
    });
    register_getters!(engine, ScriptFunctionName, module, function);
    engine.register_get("module_code", |value: &mut ScriptFunctionName| {
        value.module_code()
    });
    engine.register_get("qualified_name", |value: &mut ScriptFunctionName| {
        value.qualified_name()
    });
}
