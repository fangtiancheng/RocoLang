use serde::{Deserialize, Serialize};

use crate::{RocoError, RocoErrorInfo};

use super::{to_array, ActionResult, BagItemInfo, Engine, SpiritSkillInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActions {
    pub can_submit_action: bool,
    pub can_use_skill: bool,
    pub can_capture: bool,
    pub can_use_item: bool,
    pub can_change_spirit: bool,
    pub can_escape: bool,
    pub can_use_any_skill: bool,
    pub can_change_to_any_spirit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSpiritState {
    pub position: i64,
    pub spirit_id: i64,
    pub level: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSideState {
    pub uin: i64,
    pub active_position: i64,
    pub spirits: Vec<CombatSpiritState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    pub round: i64,
    pub weather: i64,
    pub weather_round: i64,
    pub my_side: CombatSideState,
    pub rival_side: CombatSideState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActionSnapshot {
    pub is_finished: bool,
    pub state: CombatState,
    pub actions: CombatActions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActionResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub ack_received: bool,
    pub combat_finished: bool,
    pub next_action_ready: bool,
}

impl CombatActionResult {
    pub fn ok(combat_finished: bool, next_action_ready: bool) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            ack_received: true,
            combat_finished,
            next_action_ready,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn unavailable_error(error: impl Into<RocoError>) -> Self {
        let error = error.into();
        Self {
            ok: false,
            code: 1,
            message: error.message(),
            error: Some(error.info()),
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn failed_error(error: impl Into<RocoError>) -> Self {
        let error = error.into();
        Self {
            ok: false,
            code: 2,
            message: error.message(),
            error: Some(error.info()),
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn from_action_result(action: ActionResult) -> Self {
        Self {
            ok: false,
            code: action.code,
            message: action.message,
            error: action.error,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInfo {
    pub battle_id: String,
    pub my_uin: i64,
    pub rival_uin: i64,
    pub started: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round: i64,
    pub my_hp: i64,
    pub rival_hp: i64,
    pub finished: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BattleResult {
    pub winner: Option<i64>,
    pub total_rounds: i64,
    pub finish_code: i64,
    pub trainer_exp: i64,
    pub next_level_trainer_exp: i64,
    pub honour_point: i64,
    pub exp_add_bits: i64,
    pub obtained_items: Vec<BagItemInfo>,
    pub spirit_results: Vec<BattleSpiritResult>,
    pub captured_spirits: Vec<BattleCapturedSpirit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResultQueryResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: Option<BattleResult>,
}

impl BattleResultQueryResult {
    pub fn ok(result: BattleResult) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result: Some(result),
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
            result: None,
        }
    }

    pub fn unavailable_with_error(error: RocoError) -> Self {
        Self {
            ok: false,
            code: 1,
            message: error.message(),
            error: Some(error.info()),
            result: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSpiritResult {
    pub position: i64,
    pub exp: i64,
    pub level_delta: i64,
    pub level: i64,
    pub next_exp: i64,
    pub effort: i64,
    pub new_skill_ids: Vec<i64>,
    pub evolve_spirit_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleCapturedSpirit {
    pub spirit_id: i64,
    pub level: i64,
    pub disposition: i64,
    pub property_list: Vec<i64>,
    pub flair_list: Vec<i64>,
}

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
