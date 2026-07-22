use super::super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptFunctionContextError {
    RequiresActiveCombat,
    NoCombatAvailable,
    CannotWaitForCombat { phase: ScriptCombatPhase },
    RequiresOutOfCombat,
}

impl ScriptFunctionContextError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::RequiresActiveCombat => "requires_active_combat",
            Self::NoCombatAvailable => "no_combat_available",
            Self::CannotWaitForCombat { .. } => "cannot_wait_for_combat",
            Self::RequiresOutOfCombat => "requires_out_of_combat",
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::CannotWaitForCombat { phase } => phase.code().to_string(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for ScriptFunctionContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresActiveCombat => f.write_str("This function can only be used in combat"),
            Self::NoCombatAvailable => f.write_str("No combat is available to wait for"),
            Self::CannotWaitForCombat { phase } => {
                write!(f, "Cannot wait for combat in phase {}", phase.as_str())
            }
            Self::RequiresOutOfCombat => f.write_str("This function cannot be used during combat"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatPhase {
    Idle,
    WaitingStartReply,
    WaitingFrontendReady,
    WaitingServerOpeningRelease,
    PlayingOpening,
    WaitingPlayerAction,
    WaitingRoundResult,
    PlayingRoundResult,
    WaitingRoundRelease,
    WaitingMyExtraSwitch,
    WaitingOpponentExtraSwitch,
    Finished,
    Aborted,
    Unknown,
}

impl ScriptCombatPhase {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::WaitingStartReply => "waiting_start_reply",
            Self::WaitingFrontendReady => "waiting_frontend_ready",
            Self::WaitingServerOpeningRelease => "waiting_server_opening_release",
            Self::PlayingOpening => "playing_opening",
            Self::WaitingPlayerAction => "waiting_player_action",
            Self::WaitingRoundResult => "waiting_round_result",
            Self::PlayingRoundResult => "playing_round_result",
            Self::WaitingRoundRelease => "waiting_round_release",
            Self::WaitingMyExtraSwitch => "waiting_my_extra_switch",
            Self::WaitingOpponentExtraSwitch => "waiting_opponent_extra_switch",
            Self::Finished => "finished",
            Self::Aborted => "aborted",
            Self::Unknown => "unknown",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::WaitingStartReply => "WaitingStartReply",
            Self::WaitingFrontendReady => "WaitingFrontendReady",
            Self::WaitingServerOpeningRelease => "WaitingServerOpeningRelease",
            Self::PlayingOpening => "PlayingOpening",
            Self::WaitingPlayerAction => "WaitingPlayerAction",
            Self::WaitingRoundResult => "WaitingRoundResult",
            Self::PlayingRoundResult => "PlayingRoundResult",
            Self::WaitingRoundRelease => "WaitingRoundRelease",
            Self::WaitingMyExtraSwitch => "WaitingMyExtraSwitch",
            Self::WaitingOpponentExtraSwitch => "WaitingOpponentExtraSwitch",
            Self::Finished => "Finished",
            Self::Aborted => "Aborted",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptQueryError {
    NotInAnyScene,
    SceneSpiritSnapshotUnavailable,
    CurrentRoleSnapshotUnavailable,
    BattleResultBeforeCombatStarts,
    BattleResultBeforeCombatFinishes,
    NotInCombat,
    InvalidActiveSpiritIndex,
    InvalidActiveRivalSpiritIndex,
    NoActiveSpirit,
    NoActiveRivalSpirit,
    NoSkillAtIndex { index: u32 },
}

impl ScriptQueryError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotInAnyScene => "not_in_any_scene",
            Self::SceneSpiritSnapshotUnavailable => "scene_spirit_snapshot_unavailable",
            Self::CurrentRoleSnapshotUnavailable => "current_role_snapshot_unavailable",
            Self::BattleResultBeforeCombatStarts => "battle_result_before_combat_starts",
            Self::BattleResultBeforeCombatFinishes => "battle_result_before_combat_finishes",
            Self::NotInCombat => "not_in_combat",
            Self::InvalidActiveSpiritIndex => "invalid_active_spirit_index",
            Self::InvalidActiveRivalSpiritIndex => "invalid_active_rival_spirit_index",
            Self::NoActiveSpirit => "no_active_spirit",
            Self::NoActiveRivalSpirit => "no_active_rival_spirit",
            Self::NoSkillAtIndex { .. } => "no_skill_at_index",
        }
    }

    pub const fn skill_index(&self) -> i64 {
        match self {
            Self::NoSkillAtIndex { index } => *index as i64,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInAnyScene => f.write_str("Not in any scene"),
            Self::SceneSpiritSnapshotUnavailable => {
                f.write_str("scene spirit snapshot unavailable")
            }
            Self::CurrentRoleSnapshotUnavailable => {
                f.write_str("current role snapshot unavailable")
            }
            Self::BattleResultBeforeCombatStarts => {
                f.write_str("battle result is unavailable before combat starts")
            }
            Self::BattleResultBeforeCombatFinishes => {
                f.write_str("battle result is unavailable before combat finishes")
            }
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex => f.write_str("Invalid active spirit index"),
            Self::InvalidActiveRivalSpiritIndex => f.write_str("Invalid active rival spirit index"),
            Self::NoActiveSpirit => f.write_str("No active spirit"),
            Self::NoActiveRivalSpirit => f.write_str("No active rival spirit"),
            Self::NoSkillAtIndex { index } => write!(f, "No skill at index {index}"),
        }
    }
}
