use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoGeneralError {
    LockPoisoned { target: RocoGeneralLockTarget },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoGeneralLockTarget {
    Stdlib,
}

impl RocoGeneralLockTarget {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Stdlib => "stdlib",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Stdlib => "stdlib",
        }
    }
}

impl RocoGeneralError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::LockPoisoned { .. } => "other.lock_poisoned",
        }
    }
}

impl fmt::Display for RocoGeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned { target } => write!(f, "{} lock poisoned", target.label()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RocoError {
    ScriptError(RocoScriptError),
    StdLib(RocoStdLibError),
    InvalidParam(RocoInvalidParamError),
    NetworkError(RocoNetworkError),
    TimeoutError(RocoTimeoutError),
    ServerRejected(RocoServerRejectedError),
    AssertionError(String),
    Other(RocoGeneralError),
}

impl fmt::Display for RocoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocoError::ScriptError(error) => {
                write!(f, "Script error")?;
                match &error.location {
                    RocoScriptLocation::Unknown => {}
                    RocoScriptLocation::Anonymous { position } => write!(f, " at {}", position)?,
                    RocoScriptLocation::Source { source, position } => {
                        write!(f, " in {}:{}", source, position)?
                    }
                }
                write!(f, ": [{}] {}", error.kind(), error.message())
            }
            RocoError::StdLib(error) => write!(f, "StdLib error: {}", error),
            RocoError::InvalidParam(error) => write!(f, "Invalid parameter: {}", error),
            RocoError::NetworkError(error) => write!(f, "Network error: {}", error),
            RocoError::TimeoutError(error) => write!(f, "Timeout error: {}", error),
            RocoError::ServerRejected(error) => write!(f, "Server rejected: {}", error),
            RocoError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
            RocoError::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl std::error::Error for RocoError {}

impl RocoError {
    pub fn kind(&self) -> RocoErrorKind {
        match self {
            Self::ScriptError(_) => RocoErrorKind::ScriptError,
            Self::StdLib(_) => RocoErrorKind::StdLib,
            Self::InvalidParam(_) => RocoErrorKind::InvalidParam,
            Self::NetworkError(error) => RocoErrorKind::Network { kind: error.kind() },
            Self::TimeoutError(_) => RocoErrorKind::Timeout,
            Self::ServerRejected(_) => RocoErrorKind::ServerRejected,
            Self::AssertionError(_) => RocoErrorKind::Assertion,
            Self::Other(_) => RocoErrorKind::Other,
        }
    }

    pub fn code(&self) -> String {
        match self {
            Self::ScriptError(error) => format!("script.{}", error.kind().as_str()),
            Self::StdLib(error) => error.code().to_string(),
            Self::InvalidParam(_) => "invalid_param".to_string(),
            Self::NetworkError(error) => error.code().to_string(),
            Self::TimeoutError(_) => "timeout.waiting_for_response".to_string(),
            Self::ServerRejected(error) => error.code().to_string(),
            Self::AssertionError(_) => "assertion".to_string(),
            Self::Other(error) => error.code().to_string(),
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn unsupported_function(&self) -> Option<ScriptFunctionName> {
        match self {
            Self::StdLib(RocoStdLibError::Unsupported(ScriptUnsupportedError::Function {
                name,
            })) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn return_code_rejection(&self) -> Option<RocoReturnCodeRejection> {
        match self {
            Self::ServerRejected(error) => error.return_code(),
            _ => None,
        }
    }

    pub fn http_business_rejection(&self) -> Option<RocoHttpBusinessRejection> {
        match self {
            Self::ServerRejected(error) => error.http_business(),
            _ => None,
        }
    }

    pub fn stdlib_bridge_failure(&self) -> Option<ScriptBridgeFailure> {
        match self {
            Self::StdLib(RocoStdLibError::Bridge(ScriptBridgeError::SendRequestFailed {
                failure,
            })) => Some(failure.clone()),
            _ => None,
        }
    }

    pub fn system_failure(&self) -> Option<ScriptSystemFailure> {
        match self {
            Self::StdLib(RocoStdLibError::System(error)) => error.failure(),
            _ => None,
        }
    }

    pub fn info(&self) -> RocoErrorInfo {
        let detail = match self {
            Self::InvalidParam(error) => RocoErrorDetail::InvalidParam(error.info()),
            Self::StdLib(RocoStdLibError::Unsupported(ScriptUnsupportedError::Function {
                name,
            })) => RocoErrorDetail::UnsupportedFunction(name.clone()),
            Self::StdLib(RocoStdLibError::FunctionContext(error)) => {
                RocoErrorDetail::FunctionContext(*error)
            }
            Self::StdLib(RocoStdLibError::Query(error)) => RocoErrorDetail::Query(*error),
            Self::StdLib(RocoStdLibError::StaticData(error)) => {
                RocoErrorDetail::StaticData(error.clone())
            }
            Self::StdLib(RocoStdLibError::System(error)) => error
                .failure()
                .map(RocoErrorDetail::SystemFailure)
                .unwrap_or(RocoErrorDetail::None),
            Self::StdLib(RocoStdLibError::Bridge(ScriptBridgeError::SendRequestFailed {
                failure,
            })) => RocoErrorDetail::StdlibBridge(failure.clone()),
            Self::StdLib(RocoStdLibError::ActivityOperation(error)) => {
                RocoErrorDetail::ActivityOperation(error.clone())
            }
            Self::StdLib(RocoStdLibError::CombatAction(error)) => {
                RocoErrorDetail::CombatAction(*error)
            }
            Self::StdLib(RocoStdLibError::CombatWait(error)) => {
                RocoErrorDetail::CombatWait(error.clone())
            }
            Self::StdLib(RocoStdLibError::CombatRuntime(error)) => {
                RocoErrorDetail::CombatRuntime(error.clone())
            }
            Self::StdLib(RocoStdLibError::PendingResponse(error)) => {
                RocoErrorDetail::PendingResponse(error.clone())
            }
            Self::StdLib(RocoStdLibError::Response(error)) => {
                RocoErrorDetail::Response(error.clone())
            }
            Self::StdLib(RocoStdLibError::Request(error)) => {
                RocoErrorDetail::Request(error.clone())
            }
            Self::StdLib(RocoStdLibError::SessionMemory(error)) => {
                RocoErrorDetail::SessionMemory(error.clone())
            }
            Self::StdLib(RocoStdLibError::Lookup(error)) => RocoErrorDetail::Lookup(error.clone()),
            Self::StdLib(RocoStdLibError::SpiritOperation(error)) => {
                RocoErrorDetail::SpiritOperation(error.clone())
            }
            Self::NetworkError(error) => error.info().detail,
            Self::ServerRejected(RocoServerRejectedError::ReturnCode { rejection }) => {
                RocoErrorDetail::ReturnCode(rejection.clone())
            }
            Self::ServerRejected(RocoServerRejectedError::HttpBusiness { rejection }) => {
                RocoErrorDetail::HttpBusiness(rejection.clone())
            }
            Self::Other(error) => RocoErrorDetail::General(error.clone()),
            _ => RocoErrorDetail::None,
        };
        RocoErrorInfo {
            kind: self.kind(),
            code: self.code(),
            message: self.message(),
            detail,
        }
    }
}

impl From<RocoStdLibError> for RocoError {
    fn from(error: RocoStdLibError) -> Self {
        RocoError::StdLib(error)
    }
}

impl From<ScriptFunctionContextError> for RocoStdLibError {
    fn from(error: ScriptFunctionContextError) -> Self {
        RocoStdLibError::FunctionContext(error)
    }
}

impl From<ScriptQueryError> for RocoStdLibError {
    fn from(error: ScriptQueryError) -> Self {
        RocoStdLibError::Query(error)
    }
}

impl From<ScriptCombatActionError> for RocoStdLibError {
    fn from(error: ScriptCombatActionError) -> Self {
        RocoStdLibError::CombatAction(error)
    }
}

impl From<ScriptCombatRuntimeError> for RocoStdLibError {
    fn from(error: ScriptCombatRuntimeError) -> Self {
        RocoStdLibError::CombatRuntime(error)
    }
}

impl From<ScriptCombatWaitError> for RocoStdLibError {
    fn from(error: ScriptCombatWaitError) -> Self {
        RocoStdLibError::CombatWait(error)
    }
}

impl From<ScriptPendingResponseError> for RocoStdLibError {
    fn from(error: ScriptPendingResponseError) -> Self {
        RocoStdLibError::PendingResponse(error)
    }
}

impl From<ScriptLookupError> for RocoStdLibError {
    fn from(error: ScriptLookupError) -> Self {
        RocoStdLibError::Lookup(error)
    }
}

impl From<ScriptSessionMemoryError> for RocoStdLibError {
    fn from(error: ScriptSessionMemoryError) -> Self {
        RocoStdLibError::SessionMemory(error)
    }
}

impl From<ScriptStaticDataError> for RocoStdLibError {
    fn from(error: ScriptStaticDataError) -> Self {
        RocoStdLibError::StaticData(error)
    }
}

impl From<ScriptSpiritOperationError> for RocoStdLibError {
    fn from(error: ScriptSpiritOperationError) -> Self {
        RocoStdLibError::SpiritOperation(error)
    }
}

impl From<ScriptSystemError> for RocoStdLibError {
    fn from(error: ScriptSystemError) -> Self {
        RocoStdLibError::System(error)
    }
}

impl From<ScriptActivityOperationError> for RocoStdLibError {
    fn from(error: ScriptActivityOperationError) -> Self {
        RocoStdLibError::ActivityOperation(error)
    }
}

impl From<ScriptBridgeError> for RocoStdLibError {
    fn from(error: ScriptBridgeError) -> Self {
        RocoStdLibError::Bridge(error)
    }
}

impl From<ScriptResponseError> for RocoStdLibError {
    fn from(error: ScriptResponseError) -> Self {
        RocoStdLibError::Response(error)
    }
}

impl From<ScriptRequestError> for RocoStdLibError {
    fn from(error: ScriptRequestError) -> Self {
        RocoStdLibError::Request(error)
    }
}

impl From<ScriptUnsupportedError> for RocoStdLibError {
    fn from(error: ScriptUnsupportedError) -> Self {
        RocoStdLibError::Unsupported(error)
    }
}

impl From<ScriptFunctionContextError> for RocoError {
    fn from(error: ScriptFunctionContextError) -> Self {
        RocoError::StdLib(RocoStdLibError::FunctionContext(error))
    }
}

impl From<ScriptQueryError> for RocoError {
    fn from(error: ScriptQueryError) -> Self {
        RocoError::StdLib(RocoStdLibError::Query(error))
    }
}

impl From<ScriptCombatActionError> for RocoError {
    fn from(error: ScriptCombatActionError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatAction(error))
    }
}

impl From<ScriptCombatRuntimeError> for RocoError {
    fn from(error: ScriptCombatRuntimeError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatRuntime(error))
    }
}

impl From<ScriptCombatWaitError> for RocoError {
    fn from(error: ScriptCombatWaitError) -> Self {
        RocoError::StdLib(RocoStdLibError::CombatWait(error))
    }
}

impl From<ScriptPendingResponseError> for RocoError {
    fn from(error: ScriptPendingResponseError) -> Self {
        RocoError::StdLib(RocoStdLibError::PendingResponse(error))
    }
}

impl From<ScriptLookupError> for RocoError {
    fn from(error: ScriptLookupError) -> Self {
        RocoError::StdLib(RocoStdLibError::Lookup(error))
    }
}

impl From<ScriptSessionMemoryError> for RocoError {
    fn from(error: ScriptSessionMemoryError) -> Self {
        RocoError::StdLib(RocoStdLibError::SessionMemory(error))
    }
}

impl From<ScriptStaticDataError> for RocoError {
    fn from(error: ScriptStaticDataError) -> Self {
        RocoError::StdLib(RocoStdLibError::StaticData(error))
    }
}

impl From<ScriptSpiritOperationError> for RocoError {
    fn from(error: ScriptSpiritOperationError) -> Self {
        RocoError::StdLib(RocoStdLibError::SpiritOperation(error))
    }
}

impl From<ScriptSystemError> for RocoError {
    fn from(error: ScriptSystemError) -> Self {
        RocoError::StdLib(RocoStdLibError::System(error))
    }
}

impl From<ScriptActivityOperationError> for RocoError {
    fn from(error: ScriptActivityOperationError) -> Self {
        RocoError::StdLib(RocoStdLibError::ActivityOperation(error))
    }
}

impl From<ScriptBridgeError> for RocoError {
    fn from(error: ScriptBridgeError) -> Self {
        RocoError::StdLib(RocoStdLibError::Bridge(error))
    }
}

impl From<ScriptResponseError> for RocoError {
    fn from(error: ScriptResponseError) -> Self {
        RocoError::StdLib(RocoStdLibError::Response(error))
    }
}

impl From<ScriptRequestError> for RocoError {
    fn from(error: ScriptRequestError) -> Self {
        RocoError::StdLib(RocoStdLibError::Request(error))
    }
}

impl From<ScriptUnsupportedError> for RocoError {
    fn from(error: ScriptUnsupportedError) -> Self {
        RocoError::StdLib(RocoStdLibError::Unsupported(error))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RocoErrorKind, RocoNetworkErrorKind, RocoReturnCodeKind, RocoReturnCodeRejection,
        ScriptCombatCommandFailureKind,
    };

    #[test]
    fn error_types_remain_compact() {
        assert!(size_of::<super::RocoScriptError>() <= 112);
        assert!(size_of::<super::RocoError>() <= 120);
        assert_eq!(
            size_of::<crate::types::RocoRequestContext>(),
            size_of::<String>()
        );
        assert!(size_of::<super::RocoHttpBusinessRejection>() <= 56);
    }

    #[test]
    fn error_kind_keeps_family_and_structured_kind_codes_separate() {
        let kind = RocoErrorKind::Network {
            kind: RocoNetworkErrorKind::HttpBridgeRequestFailed,
        };

        assert_eq!(kind.family_code(), "network");
        assert_eq!(kind.kind_code(), "network.http_bridge_request_failed");
    }

    #[test]
    fn combat_command_failure_kind_has_stable_code() {
        assert_eq!(
            ScriptCombatCommandFailureKind::LineupSkill.code(),
            "lineup_skill"
        );
    }

    #[test]
    fn contextual_command_failure_preserves_the_protocol_code() {
        let kind = RocoReturnCodeKind::CommandFailure { code: 4 };

        assert_eq!(kind.code(), 4);
        assert_eq!(kind.kind_code(), "command_failure");
        assert_eq!(kind.label(), "command rejected");

        let rejection = RocoReturnCodeRejection {
            kind,
            message: "not enough Rockbay".to_string(),
        };
        assert_eq!(
            rejection.description(),
            "command rejected (4): not enough Rockbay"
        );
    }
}
