use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_get("code", |value: &mut ScriptCombatCommandFailureKind| {
        value.code().to_string()
    });
    engine.register_get("message", |value: &mut ScriptCombatRuntimeError| {
        value.message()
    });
    engine.register_get(
        "command_failure_kind",
        |value: &mut ScriptCombatRuntimeError| value.command_failure_kind(),
    );
    engine.register_get(
        "command_failure_kind_code",
        |value: &mut ScriptCombatRuntimeError| value.command_failure_kind_code(),
    );
    engine.register_get("backend_kind", |value: &mut ScriptCombatRuntimeError| {
        value.backend_kind()
    });
    engine.register_get(
        "backend_kind_code",
        |value: &mut ScriptCombatRuntimeError| value.backend_kind_code(),
    );
}
