use super::super::*;

mod combat_protocol;
mod system;
mod validation;
mod wait;

pub use combat_protocol::*;
pub use system::*;
pub use validation::*;
pub use wait::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptRequestError {
    NoRunningScriptForRequest,
    NoRunningScriptForCombatCommand,
    PauseStateUnknown,
    EquipItemPositionMustBeOneBased,
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
