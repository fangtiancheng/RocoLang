use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        CombatActions,
        can_submit_action,
        can_use_skill,
        can_capture,
        can_use_item,
        can_change_spirit,
        can_escape,
        can_use_any_skill,
        can_change_to_any_spirit
    );
    register_getters!(
        engine,
        CombatSpiritState,
        position,
        spirit_id,
        level,
        hp,
        max_hp
    );
    engine.register_get("skills", |value: &mut CombatSpiritState| {
        to_array(&value.skills)
    });
    register_getters!(engine, CombatSideState, uin, active_position);
    engine.register_get("spirits", |value: &mut CombatSideState| {
        to_array(&value.spirits)
    });
    register_getters!(engine, CombatState, round, weather, weather_round);
    engine.register_get("my_side", |value: &mut CombatState| value.my_side.clone());
    engine.register_get("rival_side", |value: &mut CombatState| {
        value.rival_side.clone()
    });
    register_getters!(engine, CombatActionSnapshot, is_finished);
    engine.register_get("state", |value: &mut CombatActionSnapshot| {
        value.state.clone()
    });
    engine.register_get("actions", |value: &mut CombatActionSnapshot| {
        value.actions.clone()
    });
    register_getters!(
        engine,
        CombatActionResult,
        ok,
        code,
        message,
        error,
        ack_received,
        combat_finished,
        next_action_ready
    );
    register_error_info_getters!(engine, CombatActionResult);
    register_getters!(engine, BattleInfo, battle_id, my_uin, rival_uin, started);
    register_getters!(engine, RoundResult, round, my_hp, rival_hp, finished);
    register_getters!(
        engine,
        BattleResult,
        winner,
        total_rounds,
        finish_code,
        trainer_exp,
        next_level_trainer_exp,
        honour_point,
        exp_add_bits
    );
    engine.register_get("obtained_items", |value: &mut BattleResult| {
        to_array(&value.obtained_items)
    });
    engine.register_get("spirit_results", |value: &mut BattleResult| {
        to_array(&value.spirit_results)
    });
    engine.register_get("captured_spirits", |value: &mut BattleResult| {
        to_array(&value.captured_spirits)
    });
    register_getters!(engine, BattleResultQueryResult, ok, code, message, error);
    register_error_info_getters!(engine, BattleResultQueryResult);
    engine.register_get("result", |value: &mut BattleResultQueryResult| {
        value.result.clone()
    });
    register_getters!(
        engine,
        BattleSpiritResult,
        position,
        exp,
        level_delta,
        level,
        next_exp,
        effort,
        evolve_spirit_id
    );
    engine.register_get("new_skill_ids", |value: &mut BattleSpiritResult| {
        to_array(&value.new_skill_ids)
    });
    register_getters!(engine, BattleCapturedSpirit, spirit_id, level, disposition);
    engine.register_get("property_list", |value: &mut BattleCapturedSpirit| {
        to_array(&value.property_list)
    });
    engine.register_get("flair_list", |value: &mut BattleCapturedSpirit| {
        to_array(&value.flair_list)
    });
}
