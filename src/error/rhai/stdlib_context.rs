use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_get("code", |value: &mut ScriptCombatPhase| {
        value.code().to_string()
    });
    engine.register_get("kind_code", |value: &mut ScriptFunctionContextError| {
        value.kind_code()
    });
    engine.register_get(
        "combat_phase_code",
        |value: &mut ScriptFunctionContextError| value.combat_phase_code(),
    );
    engine.register_get("kind_code", |value: &mut ScriptQueryError| {
        value.kind_code()
    });
    engine.register_get("skill_index", |value: &mut ScriptQueryError| {
        value.skill_index()
    });
    engine.register_type_with_name::<ScriptStaticDataError>("ScriptStaticDataError");
    engine.register_get("kind_code", |value: &mut ScriptStaticDataError| {
        value.kind_code()
    });
    engine.register_get("position", |value: &mut ScriptStaticDataError| {
        value.position()
    });
    engine.register_get("function_name", |value: &mut ScriptStaticDataError| {
        value.function_name()
    });
    engine.register_get("message", |value: &mut ScriptStaticDataError| {
        value.message()
    });
}
