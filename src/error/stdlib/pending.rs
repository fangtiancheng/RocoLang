use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptPendingResponseError {
    ExternalGameExtraFieldInvalidInteger {
        key: String,
        value: String,
        expected: ScriptIntegerType,
    },
    UnexpectedHttpResponse {
        pending: ScriptHttpResponseName,
        expected: ScriptHttpResponseName,
        actual: ScriptHttpResponseName,
    },
    CombatLoadedUnexpectedPhase {
        phase: ScriptCombatPhase,
    },
    CombatActionAckMismatch {
        expected_action_kind: u8,
        expected_spirit_index: u8,
        actual_action_kind: u8,
        actual_spirit_index: u8,
    },
    SkillPoolIndexExceedsI64 {
        index: usize,
    },
    MissingNetResponsePayload {
        target: RocoNetResponseParseTarget,
    },
    MissingScriptSessionForActionWait,
    StorageSpiritDetailEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptIntegerType {
    I64,
}

impl fmt::Display for ScriptIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I64 => f.write_str("i64"),
        }
    }
}

impl fmt::Display for ScriptPendingResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExternalGameExtraFieldInvalidInteger {
                key,
                value,
                expected,
            } => write!(
                f,
                "external_game extra field {key}={value:?} is not a valid {expected}"
            ),
            Self::UnexpectedHttpResponse {
                pending,
                expected,
                actual,
            } => write!(
                f,
                "unexpected script http response for {pending}: expected {expected}, got {actual}"
            ),
            Self::CombatLoadedUnexpectedPhase { phase } => write!(
                f,
                "combat did not become ready after loaded ack, current phase {}",
                phase.as_str()
            ),
            Self::CombatActionAckMismatch {
                expected_action_kind,
                expected_spirit_index,
                actual_action_kind,
                actual_spirit_index,
            } => write!(
                f,
                "combat action ack mismatch: expected action_kind={expected_action_kind} spirit_index={expected_spirit_index}, got action_kind={actual_action_kind} spirit_index={actual_spirit_index}"
            ),
            Self::SkillPoolIndexExceedsI64 { index } => {
                write!(f, "skill pool index exceeds i64 range: {index}")
            }
            Self::MissingNetResponsePayload { target } => {
                write!(f, "network response {} is missing payload", target.label())
            }
            Self::MissingScriptSessionForActionWait => {
                f.write_str("script action wait requested while no script is running")
            }
            Self::StorageSpiritDetailEmpty => f.write_str("storage spirit detail is empty"),
        }
    }
}

impl ScriptPendingResponseError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::ExternalGameExtraFieldInvalidInteger { .. } => {
                "external_game_extra_field_invalid_integer"
            }
            Self::UnexpectedHttpResponse { .. } => "unexpected_http_response",
            Self::CombatLoadedUnexpectedPhase { .. } => "combat_loaded_unexpected_phase",
            Self::CombatActionAckMismatch { .. } => "combat_action_ack_mismatch",
            Self::SkillPoolIndexExceedsI64 { .. } => "skill_pool_index_exceeds_i64",
            Self::MissingNetResponsePayload { .. } => "missing_net_response_payload",
            Self::MissingScriptSessionForActionWait => "missing_script_session_for_action_wait",
            Self::StorageSpiritDetailEmpty => "storage_spirit_detail_empty",
        }
    }

    pub fn pending_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { pending, .. } => pending.code.clone(),
            _ => String::new(),
        }
    }

    pub fn expected_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { expected, .. } => expected.code.clone(),
            _ => String::new(),
        }
    }

    pub fn actual_http_response_code(&self) -> String {
        match self {
            Self::UnexpectedHttpResponse { actual, .. } => actual.code.clone(),
            _ => String::new(),
        }
    }

    pub fn combat_phase_code(&self) -> String {
        match self {
            Self::CombatLoadedUnexpectedPhase { phase } => phase.as_str().to_string(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target_code(&self) -> String {
        match self {
            Self::MissingNetResponsePayload { target } => target.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn expected_action_kind(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                expected_action_kind,
                ..
            } => i64::from(*expected_action_kind),
            _ => 0,
        }
    }

    pub fn expected_spirit_index(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                expected_spirit_index,
                ..
            } => i64::from(*expected_spirit_index),
            _ => 0,
        }
    }

    pub fn actual_action_kind(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                actual_action_kind, ..
            } => i64::from(*actual_action_kind),
            _ => 0,
        }
    }

    pub fn actual_spirit_index(&self) -> i64 {
        match self {
            Self::CombatActionAckMismatch {
                actual_spirit_index,
                ..
            } => i64::from(*actual_spirit_index),
            _ => 0,
        }
    }
}
