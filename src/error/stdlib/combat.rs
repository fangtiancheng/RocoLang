use super::super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionError {
    NotInCombat,
    InvalidActiveSpiritIndex { position: u8 },
    CannotUseSkill,
    SkillUnavailableOrNoPp { skill_id: u32 },
    CannotUseItem,
    CombatItemUnavailable { item_id: u32 },
    CannotChangeSpirit,
    CannotEscape,
    SpiritIndexOutOfRange { position: u32 },
    InvalidTargetSpiritIndex { position: u8 },
    TargetSpiritFainted { position: u8 },
    TargetSpiritNotFound { position: u8 },
}

impl ScriptCombatActionError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotInCombat => "not_in_combat",
            Self::InvalidActiveSpiritIndex { .. } => "invalid_active_spirit_index",
            Self::CannotUseSkill => "cannot_use_skill",
            Self::SkillUnavailableOrNoPp { .. } => "skill_unavailable_or_no_pp",
            Self::CannotUseItem => "cannot_use_item",
            Self::CombatItemUnavailable { .. } => "combat_item_unavailable",
            Self::CannotChangeSpirit => "cannot_change_spirit",
            Self::CannotEscape => "cannot_escape",
            Self::SpiritIndexOutOfRange { .. } => "spirit_index_out_of_range",
            Self::InvalidTargetSpiritIndex { .. } => "invalid_target_spirit_index",
            Self::TargetSpiritFainted { .. } => "target_spirit_fainted",
            Self::TargetSpiritNotFound { .. } => "target_spirit_not_found",
        }
    }

    pub const fn position(&self) -> i64 {
        match self {
            Self::InvalidActiveSpiritIndex { position }
            | Self::InvalidTargetSpiritIndex { position }
            | Self::TargetSpiritFainted { position }
            | Self::TargetSpiritNotFound { position } => *position as i64,
            Self::SpiritIndexOutOfRange { position } => *position as i64,
            _ => -1,
        }
    }

    pub const fn skill_id(&self) -> i64 {
        match self {
            Self::SkillUnavailableOrNoPp { skill_id } => *skill_id as i64,
            _ => -1,
        }
    }

    pub const fn item_id(&self) -> i64 {
        match self {
            Self::CombatItemUnavailable { item_id } => *item_id as i64,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptCombatActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex { position } => {
                write!(f, "Invalid active spirit index: {position}")
            }
            Self::CannotUseSkill => f.write_str("cannot use skill in current combat state"),
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::CannotUseItem => f.write_str("cannot use item in current combat state"),
            Self::CombatItemUnavailable { item_id } => {
                write!(f, "combat item unavailable: item_id={item_id}")
            }
            Self::CannotChangeSpirit => f.write_str("cannot change spirit in current combat state"),
            Self::CannotEscape => f.write_str("cannot escape in current combat state"),
            Self::SpiritIndexOutOfRange { position } => {
                write!(f, "spirit_index out of range: {position}")
            }
            Self::InvalidTargetSpiritIndex { position } => {
                write!(f, "invalid target spirit index: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target index: {position}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatRuntimeError {
    AutoFinishPresentationFailed {
        kind: ScriptCombatCommandFailureKind,
    },
    MarkFrontendLoadedFailed {
        kind: ScriptCombatCommandFailureKind,
    },
    Backend {
        kind: ScriptBackendCombatRuntimeErrorKind,
    },
}

impl fmt::Display for ScriptCombatRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

impl ScriptCombatRuntimeError {
    pub fn message(&self) -> String {
        match self {
            Self::AutoFinishPresentationFailed { kind } => {
                format!(
                    "failed to auto-finish script combat presentation after {}",
                    kind.as_str()
                )
            }
            Self::MarkFrontendLoadedFailed { kind } => {
                format!(
                    "failed to mark script combat frontend loaded after {}",
                    kind.as_str()
                )
            }
            Self::Backend { kind } => {
                format!("combat runtime {}", kind.as_str())
            }
        }
    }

    pub fn command_failure_kind(&self) -> Option<ScriptCombatCommandFailureKind> {
        match self {
            Self::AutoFinishPresentationFailed { kind }
            | Self::MarkFrontendLoadedFailed { kind } => Some(*kind),
            Self::Backend { .. } => None,
        }
    }

    pub fn command_failure_kind_code(&self) -> String {
        self.command_failure_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }

    pub fn backend_kind(&self) -> Option<ScriptBackendCombatRuntimeErrorKind> {
        match self {
            Self::Backend { kind } => Some(*kind),
            Self::AutoFinishPresentationFailed { .. } | Self::MarkFrontendLoadedFailed { .. } => {
                None
            }
        }
    }

    pub fn backend_kind_code(&self) -> String {
        self.backend_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatCommandFailureKind {
    SessionNotStarted,
    OfflineUnsupported,
    CombatRuntime,
    CombatActionValidation,
    CombatEffects,
    ScriptSystem,
    Workflow,
    Wpe,
    LoginContext,
    LoginTarget,
    NetCommand,
    LineupSkill,
    InvalidUin,
    SystemClock,
    NetBridge,
    HttpBridge,
    SnapshotUnavailable,
    RuntimeStopped,
    ReplyDropped,
}

impl ScriptCombatCommandFailureKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::SessionNotStarted => "session_not_started",
            Self::OfflineUnsupported => "offline_unsupported",
            Self::CombatRuntime => "combat_runtime",
            Self::CombatActionValidation => "combat_action_validation",
            Self::CombatEffects => "combat_effects",
            Self::ScriptSystem => "script_system",
            Self::Workflow => "workflow",
            Self::Wpe => "wpe",
            Self::LoginContext => "login_context",
            Self::LoginTarget => "login_target",
            Self::NetCommand => "net_command",
            Self::LineupSkill => "lineup_skill",
            Self::InvalidUin => "invalid_uin",
            Self::SystemClock => "system_clock",
            Self::NetBridge => "net_bridge",
            Self::HttpBridge => "http_bridge",
            Self::SnapshotUnavailable => "snapshot_unavailable",
            Self::RuntimeStopped => "runtime_stopped",
            Self::ReplyDropped => "reply_dropped",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionNotStarted => "session not started",
            Self::OfflineUnsupported => "offline unsupported",
            Self::CombatRuntime => "combat runtime",
            Self::CombatActionValidation => "combat action validation",
            Self::CombatEffects => "combat effects",
            Self::ScriptSystem => "script system",
            Self::Workflow => "workflow",
            Self::Wpe => "WPE",
            Self::LoginContext => "login context",
            Self::LoginTarget => "login target",
            Self::NetCommand => "net command",
            Self::LineupSkill => "lineup skill",
            Self::InvalidUin => "invalid uin",
            Self::SystemClock => "system clock",
            Self::NetBridge => "net bridge",
            Self::HttpBridge => "http bridge",
            Self::SnapshotUnavailable => "snapshot unavailable",
            Self::RuntimeStopped => "runtime stopped",
            Self::ReplyDropped => "reply dropped",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBackendCombatRuntimeErrorKind {
    BattleFacts,
    SideResolution,
    InvalidReadyPhasePayload,
    MissingStartContext,
    InvalidPhase,
    MissingStartSummary,
    MissingSideRegistry,
    MissingHistoryRecorder,
    MissingBattleFacts,
    MissingObservedInitialStateForRoundHistory,
    MissingRoundPresentation,
    PresentationBuild,
    HistoryRecorder,
    HistoryProjection,
}

impl ScriptBackendCombatRuntimeErrorKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::BattleFacts => "battle_facts",
            Self::SideResolution => "side_resolution",
            Self::InvalidReadyPhasePayload => "invalid_ready_phase_payload",
            Self::MissingStartContext => "missing_start_context",
            Self::InvalidPhase => "invalid_phase",
            Self::MissingStartSummary => "missing_start_summary",
            Self::MissingSideRegistry => "missing_side_registry",
            Self::MissingHistoryRecorder => "missing_history_recorder",
            Self::MissingBattleFacts => "missing_battle_facts",
            Self::MissingObservedInitialStateForRoundHistory => {
                "missing_observed_initial_state_for_round_history"
            }
            Self::MissingRoundPresentation => "missing_round_presentation",
            Self::PresentationBuild => "presentation_build",
            Self::HistoryRecorder => "history_recorder",
            Self::HistoryProjection => "history_projection",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BattleFacts => "battle facts",
            Self::SideResolution => "side resolution",
            Self::InvalidReadyPhasePayload => "invalid ready phase payload",
            Self::MissingStartContext => "missing start context",
            Self::InvalidPhase => "invalid phase",
            Self::MissingStartSummary => "missing start summary",
            Self::MissingSideRegistry => "missing side registry",
            Self::MissingHistoryRecorder => "missing history recorder",
            Self::MissingBattleFacts => "missing battle facts",
            Self::MissingObservedInitialStateForRoundHistory => {
                "missing observed initial state for round history"
            }
            Self::MissingRoundPresentation => "missing active presentation",
            Self::PresentationBuild => "presentation build",
            Self::HistoryRecorder => "history recorder",
            Self::HistoryProjection => "history projection",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatWaitError {
    TimeoutWaitingCombatAction {
        phase: ScriptCombatPhase,
        elapsed_ms: u128,
    },
}

impl ScriptCombatWaitError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::TimeoutWaitingCombatAction { .. } => "timeout_waiting_combat_action",
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::TimeoutWaitingCombatAction { phase, .. } => phase.code().to_string(),
        }
    }

    pub const fn elapsed_ms(&self) -> i64 {
        match self {
            Self::TimeoutWaitingCombatAction { elapsed_ms, .. } => {
                if *elapsed_ms > i64::MAX as u128 {
                    i64::MAX
                } else {
                    *elapsed_ms as i64
                }
            }
        }
    }
}

impl fmt::Display for ScriptCombatWaitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeoutWaitingCombatAction { phase, elapsed_ms } => write!(
                f,
                "timeout waiting for combat action, phase {}, elapsed_ms={elapsed_ms}",
                phase.as_str()
            ),
        }
    }
}
