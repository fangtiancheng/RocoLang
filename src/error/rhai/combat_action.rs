use ::rhai::Engine;

use super::super::*;

pub(super) fn register(engine: &mut Engine) {
    engine.register_type_with_name::<ScriptCombatActionError>("ScriptCombatActionError");
    engine.register_get("kind_code", |value: &mut ScriptCombatActionError| {
        value.kind_code()
    });
    engine.register_get("position", |value: &mut ScriptCombatActionError| {
        value.position()
    });
    engine.register_get("skill_id", |value: &mut ScriptCombatActionError| {
        value.skill_id()
    });
    engine.register_get("item_id", |value: &mut ScriptCombatActionError| {
        value.item_id()
    });
    engine.register_type_with_name::<ScriptCombatWaitError>("ScriptCombatWaitError");
    engine.register_get("kind_code", |value: &mut ScriptCombatWaitError| {
        value.kind_code()
    });
    engine.register_get("combat_phase_code", |value: &mut ScriptCombatWaitError| {
        value.combat_phase_code()
    });
    engine.register_get("elapsed_ms", |value: &mut ScriptCombatWaitError| {
        value.elapsed_ms()
    });
}
