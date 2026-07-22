use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptRequestError {
    NoRunningScriptForRequest,
    NoRunningScriptForCombatCommand,
    PauseStateUnknown,
    EquipItemPositionMustBeOneBased,
    ClearLineupRequiresBatchSupport,
    PendingReplyRefreshFailed {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    WaitStateRejected {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    InvalidCombatIntent {
        kind: ScriptCombatIntentKind,
        spirit_index: u8,
        value: u32,
        error: ScriptCombatProtocolError,
    },
    InvalidCombatCommand {
        kind: ScriptCombatActionValidationKind,
    },
    UnsupportedCombatServerType {
        value: u8,
    },
    UnsupportedCombatType {
        value: u8,
    },
    UnsupportedOperation {
        context: RocoRequestContext,
        value: u8,
    },
}

impl ScriptRequestError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NoRunningScriptForRequest => "no_running_script_for_request",
            Self::NoRunningScriptForCombatCommand => "no_running_script_for_combat_command",
            Self::PauseStateUnknown => "pause_state_unknown",
            Self::EquipItemPositionMustBeOneBased => "equip_item_position_must_be_one_based",
            Self::ClearLineupRequiresBatchSupport => "clear_lineup_requires_batch_support",
            Self::PendingReplyRefreshFailed { .. } => "pending_reply_refresh_failed",
            Self::WaitStateRejected { .. } => "wait_state_rejected",
            Self::InvalidCombatIntent { .. } => "invalid_combat_intent",
            Self::InvalidCombatCommand { .. } => "invalid_combat_command",
            Self::UnsupportedCombatServerType { .. } => "unsupported_combat_server_type",
            Self::UnsupportedCombatType { .. } => "unsupported_combat_type",
            Self::UnsupportedOperation { .. } => "unsupported_operation",
        }
    }

    pub const fn wait_context_code(&self) -> &'static str {
        match self {
            Self::PendingReplyRefreshFailed { context, .. }
            | Self::WaitStateRejected { context, .. } => context.code(),
            _ => "",
        }
    }

    pub const fn system_failure_kind_code(&self) -> &'static str {
        match self {
            Self::PendingReplyRefreshFailed { kind, .. } | Self::WaitStateRejected { kind, .. } => {
                kind.code()
            }
            _ => "",
        }
    }

    pub const fn combat_intent_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatIntent { kind, .. } => kind.code(),
            _ => "",
        }
    }

    pub const fn combat_validation_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatCommand { kind } => kind.code(),
            _ => "",
        }
    }

    pub const fn spirit_index(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { spirit_index, .. } => *spirit_index as i64,
            _ => -1,
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { value, .. } => *value as i64,
            Self::UnsupportedCombatServerType { value } | Self::UnsupportedCombatType { value } => {
                *value as i64
            }
            Self::UnsupportedOperation { value, .. } => *value as i64,
            _ => -1,
        }
    }

    pub fn operation_context(&self) -> RocoRequestContext {
        match self {
            Self::UnsupportedOperation { context, .. } => context.clone(),
            _ => RocoRequestContext::default(),
        }
    }

    pub const fn combat_protocol_error_kind_code(&self) -> &'static str {
        match self {
            Self::InvalidCombatIntent { error, .. } => error.kind_code(),
            _ => "",
        }
    }

    pub const fn combat_protocol_error_value(&self) -> i64 {
        match self {
            Self::InvalidCombatIntent { error, .. } => error.value(),
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatIntentKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatIntentKind {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use_skill",
            Self::ChangeSpirit => "change_spirit",
            Self::UseItem => "use_item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatIntentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolError {
    InvalidSpiritPosition {
        value: u8,
    },
    TargetSpiritPositionOutOfRange {
        value: u32,
    },
    UnmappedAbnormalState {
        raw_id: u32,
    },
    PropertyStageOutOfRange {
        field: ScriptCombatProtocolPropertyStage,
        value: i16,
    },
    UnknownWeatherEffect {
        raw_weather: u8,
    },
    ParticipantDisplaySideUnresolved {
        role: ScriptCombatParticipantDisplayRole,
        id: u32,
        participant_type: u8,
        position: u8,
    },
    ChangeSpiritPositionOutOfRange {
        skill_id: u32,
    },
    InvalidChangeSpiritPosition {
        position: u8,
    },
}

impl ScriptCombatProtocolError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::InvalidSpiritPosition { .. } => "invalid_spirit_position",
            Self::TargetSpiritPositionOutOfRange { .. } => "target_spirit_position_out_of_range",
            Self::UnmappedAbnormalState { .. } => "unmapped_abnormal_state",
            Self::PropertyStageOutOfRange { .. } => "property_stage_out_of_range",
            Self::UnknownWeatherEffect { .. } => "unknown_weather_effect",
            Self::ParticipantDisplaySideUnresolved { .. } => "participant_display_side_unresolved",
            Self::ChangeSpiritPositionOutOfRange { .. } => "change_spirit_position_out_of_range",
            Self::InvalidChangeSpiritPosition { .. } => "invalid_change_spirit_position",
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidSpiritPosition { value } => *value as i64,
            Self::TargetSpiritPositionOutOfRange { value } => *value as i64,
            Self::UnmappedAbnormalState { raw_id } => *raw_id as i64,
            Self::PropertyStageOutOfRange { value, .. } => *value as i64,
            Self::UnknownWeatherEffect { raw_weather } => *raw_weather as i64,
            Self::ParticipantDisplaySideUnresolved { id, .. } => *id as i64,
            Self::ChangeSpiritPositionOutOfRange { skill_id } => *skill_id as i64,
            Self::InvalidChangeSpiritPosition { position } => *position as i64,
        }
    }

    pub const fn property_stage_code(&self) -> &'static str {
        match self {
            Self::PropertyStageOutOfRange { field, .. } => field.code(),
            _ => "",
        }
    }

    pub const fn participant_role_code(&self) -> &'static str {
        match self {
            Self::ParticipantDisplaySideUnresolved { role, .. } => role.code(),
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolPropertyStage {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    SpiritPower,
    DefensePierce,
    Critical,
}

impl ScriptCombatProtocolPropertyStage {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "pa",
            Self::PhysicalDefense => "pd",
            Self::MagicAttack => "ma",
            Self::MagicDefense => "md",
            Self::Speed => "ve",
            Self::SpiritPower => "sp",
            Self::DefensePierce => "dp",
            Self::Critical => "crit",
        }
    }
}

impl fmt::Display for ScriptCombatProtocolPropertyStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatParticipantDisplayRole {
    Actor,
    Target,
}

impl ScriptCombatParticipantDisplayRole {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::Target => "target",
        }
    }

    pub const fn identity_str(self) -> &'static str {
        match self {
            Self::Actor => "offense",
            Self::Target => "defense",
        }
    }
}

impl fmt::Display for ScriptCombatParticipantDisplayRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpiritPosition { value } => {
                write!(f, "spirit position must be in 1..=6, got {value}")
            }
            Self::TargetSpiritPositionOutOfRange { value } => {
                write!(f, "target spirit position out of range: {value}")
            }
            Self::UnmappedAbnormalState { raw_id } => {
                write!(f, "unmapped combat abnormal state id: {raw_id}")
            }
            Self::PropertyStageOutOfRange { field, value } => {
                write!(f, "combat property stage {field} out of range: {value}")
            }
            Self::UnknownWeatherEffect { raw_weather } => {
                write!(f, "unknown combat weather effect: {raw_weather}")
            }
            Self::ParticipantDisplaySideUnresolved {
                role,
                id,
                participant_type,
                position,
            } => write!(
                f,
                "combat participant display {role} side unresolved {}_id={id} {}_type={participant_type} {}_index={position}",
                role.identity_str(),
                role.identity_str(),
                role.identity_str()
            ),
            Self::ChangeSpiritPositionOutOfRange { skill_id } => {
                write!(f, "combat change spirit position out of range: {skill_id}")
            }
            Self::InvalidChangeSpiritPosition { position } => {
                write!(f, "combat change spirit position must be in 1..=6, got {position}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptWaitContext {
    WaitNextCombatAction,
    CombatActionSettlingAfterAck,
    UseItemAction,
    TalentRefresh,
    TalentRefreshResultAfterAck,
}

impl ScriptWaitContext {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaitNextCombatAction => "wait_next_combat_action",
            Self::CombatActionSettlingAfterAck => "combat_action_settling_after_ack",
            Self::UseItemAction => "use_item_action",
            Self::TalentRefresh => "talent_refresh",
            Self::TalentRefreshResultAfterAck => "talent_refresh_result_after_ack",
        }
    }
}

impl fmt::Display for ScriptWaitContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRunningScriptForRequest => {
                f.write_str("script request submitted while no script is running")
            }
            Self::NoRunningScriptForCombatCommand => {
                f.write_str("script combat command submitted while no script is running")
            }
            Self::PauseStateUnknown => f.write_str(
                "pause state is unknown; call game::try_set_pause or game::set_pause first",
            ),
            Self::EquipItemPositionMustBeOneBased => {
                f.write_str("equip_item position must be 1-based")
            }
            Self::ClearLineupRequiresBatchSupport => {
                f.write_str("clear_lineup requires confirmed batch request support")
            }
            Self::PendingReplyRefreshFailed { context, kind } => {
                write!(f, "{context} failed to refresh pending script reply: {kind}")
            }
            Self::WaitStateRejected { context, kind } => {
                write!(f, "{context} failed to set script wait state: {kind}")
            }
            Self::InvalidCombatIntent {
                kind,
                spirit_index,
                value,
                error,
            } => write!(
                f,
                "invalid combat intent kind={kind} spirit_index={spirit_index} value={value}: {error}"
            ),
            Self::InvalidCombatCommand { kind } => {
                write!(f, "invalid combat command: {kind}")
            }
            Self::UnsupportedCombatServerType { value } => {
                write!(f, "unsupported combat server_type: {value}")
            }
            Self::UnsupportedCombatType { value } => {
                write!(f, "unsupported combat_type: {value}")
            }
            Self::UnsupportedOperation { context, value } => {
                write!(f, "unsupported {} operation: {value}", context.raw)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionValidationKind {
    CannotSubmitAction,
    BattleFactsUnavailable,
    InvalidActivePosition {
        position: u8,
    },
    SpiritPositionMismatch {
        active: u8,
        request: u8,
    },
    ActiveSpiritNotFound {
        position: u8,
    },
    ActiveSpiritFainted {
        position: u8,
    },
    ActionAvailabilityUnavailable,
    ActionUnavailable {
        action: ScriptCombatActionAvailabilityKind,
    },
    SkillUnavailableOrNoPp {
        skill_id: u32,
    },
    TargetSpiritAlreadyActive {
        position: u8,
    },
    TargetSpiritNotFound {
        position: u8,
    },
    TargetSpiritFainted {
        position: u8,
    },
    InvalidItemId {
        item_id: u32,
    },
}

impl ScriptCombatActionValidationKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CannotSubmitAction => "cannot_submit_action",
            Self::BattleFactsUnavailable => "battle_facts_unavailable",
            Self::InvalidActivePosition { .. } => "invalid_active_position",
            Self::SpiritPositionMismatch { .. } => "spirit_position_mismatch",
            Self::ActiveSpiritNotFound { .. } => "active_spirit_not_found",
            Self::ActiveSpiritFainted { .. } => "active_spirit_fainted",
            Self::ActionAvailabilityUnavailable => "action_availability_unavailable",
            Self::ActionUnavailable { .. } => "action_unavailable",
            Self::SkillUnavailableOrNoPp { .. } => "skill_unavailable_or_no_pp",
            Self::TargetSpiritAlreadyActive { .. } => "target_spirit_already_active",
            Self::TargetSpiritNotFound { .. } => "target_spirit_not_found",
            Self::TargetSpiritFainted { .. } => "target_spirit_fainted",
            Self::InvalidItemId { .. } => "invalid_item_id",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionAvailabilityKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatActionAvailabilityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use skill",
            Self::ChangeSpirit => "change spirit",
            Self::UseItem => "use item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatActionAvailabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatActionValidationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotSubmitAction => {
                f.write_str("cannot submit combat action in current combat runtime phase")
            }
            Self::BattleFactsUnavailable => f.write_str("combat battle facts unavailable"),
            Self::InvalidActivePosition { position } => {
                write!(f, "invalid active spirit position: {position}")
            }
            Self::SpiritPositionMismatch { active, request } => write!(
                f,
                "combat action spirit_position mismatch: active={active}, request={request}"
            ),
            Self::ActiveSpiritNotFound { position } => {
                write!(f, "active spirit not found: {position}")
            }
            Self::ActiveSpiritFainted { position } => {
                write!(f, "active spirit is fainted: {position}")
            }
            Self::ActionAvailabilityUnavailable => {
                f.write_str("combat action availability unavailable")
            }
            Self::ActionUnavailable { action } => {
                write!(f, "cannot {action} in current combat facts")
            }
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::TargetSpiritAlreadyActive { position } => {
                write!(f, "target spirit is already active: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target position: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::InvalidItemId { item_id } => write!(f, "invalid combat item id: {item_id}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptRequestSystemFailureKind {
    DraftUpdateWhileRunning,
    ScriptAlreadyRunning,
    NoRunningScript,
    NoDebugScriptRunning,
    InactiveSession {
        session_id: u64,
        current_session_id: u64,
    },
    MissingPendingReply {
        serial_num: u32,
    },
    SendResponseFailed {
        payload_returned: bool,
    },
    SendDebugCommandFailed {
        payload_returned: bool,
    },
}

impl ScriptRequestSystemFailureKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::DraftUpdateWhileRunning => "draft_update_while_running",
            Self::ScriptAlreadyRunning => "script_already_running",
            Self::NoRunningScript => "no_running_script",
            Self::NoDebugScriptRunning => "no_debug_script_running",
            Self::InactiveSession { .. } => "inactive_session",
            Self::MissingPendingReply { .. } => "missing_pending_reply",
            Self::SendResponseFailed { .. } => "send_response_failed",
            Self::SendDebugCommandFailed { .. } => "send_debug_command_failed",
        }
    }
}

impl fmt::Display for ScriptRequestSystemFailureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DraftUpdateWhileRunning => {
                f.write_str("cannot update script draft while script is running")
            }
            Self::ScriptAlreadyRunning => {
                f.write_str("script is running; stop it before running a new task")
            }
            Self::NoRunningScript => f.write_str("no script running"),
            Self::NoDebugScriptRunning => f.write_str("no debug script running"),
            Self::InactiveSession {
                session_id,
                current_session_id,
            } => write!(
                f,
                "inactive script session session_id={session_id} current_session_id={current_session_id}"
            ),
            Self::MissingPendingReply { serial_num } => {
                write!(f, "no pending script reply for serial_num={serial_num}")
            }
            Self::SendResponseFailed { payload_returned } => {
                write!(
                    f,
                    "failed to send runtime response (payload_returned={payload_returned})"
                )
            }
            Self::SendDebugCommandFailed { payload_returned } => {
                write!(
                    f,
                    "failed to send debug command (payload_returned={payload_returned})"
                )
            }
        }
    }
}
