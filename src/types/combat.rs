use serde::{Deserialize, Serialize};

use crate::{RocoError, RocoErrorInfo};

use super::{ActionResult, BagItemInfo, SpiritSkillInfo};

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
