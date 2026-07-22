use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoErrorKind {
    ScriptError,
    StdLib,
    InvalidParam,
    Network { kind: RocoNetworkErrorKind },
    Timeout,
    ServerRejected,
    Assertion,
    Other,
}

impl RocoErrorKind {
    pub const fn family_code(&self) -> &'static str {
        match self {
            Self::ScriptError => "script_error",
            Self::StdLib => "stdlib",
            Self::InvalidParam => "invalid_param",
            Self::Network { .. } => "network",
            Self::Timeout => "timeout",
            Self::ServerRejected => "server_rejected",
            Self::Assertion => "assertion",
            Self::Other => "other",
        }
    }

    pub fn kind_code(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RocoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network { kind } => write!(f, "network.{kind}"),
            _ => f.write_str(self.family_code()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoErrorInfo {
    pub kind: RocoErrorKind,
    pub code: String,
    pub message: String,
    pub detail: RocoErrorDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoErrorDetail {
    None,
    General(RocoGeneralError),
    InvalidParam(RocoInvalidParamInfo),
    UnsupportedFunction(ScriptFunctionName),
    FunctionContext(ScriptFunctionContextError),
    Query(ScriptQueryError),
    StaticData(ScriptStaticDataError),
    SystemFailure(ScriptSystemFailure),
    StdlibBridge(ScriptBridgeFailure),
    ActivityOperation(ScriptActivityOperationError),
    CombatAction(ScriptCombatActionError),
    CombatWait(ScriptCombatWaitError),
    CombatRuntime(ScriptCombatRuntimeError),
    PendingResponse(ScriptPendingResponseError),
    Response(ScriptResponseError),
    Request(ScriptRequestError),
    SessionMemory(ScriptSessionMemoryError),
    Lookup(ScriptLookupError),
    Bridge(RocoBridgeErrorInfo),
    NetResponseParse(RocoNetResponseParseFailure),
    ReturnCode(RocoReturnCodeRejection),
    HttpBusiness(RocoHttpBusinessRejection),
}

impl RocoErrorDetail {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::General(_) => "general",
            Self::InvalidParam(_) => "invalid_param",
            Self::UnsupportedFunction(_) => "unsupported_function",
            Self::FunctionContext(_) => "function_context",
            Self::Query(_) => "query",
            Self::StaticData(_) => "static_data",
            Self::SystemFailure(_) => "system_failure",
            Self::StdlibBridge(_) => "stdlib_bridge",
            Self::ActivityOperation(_) => "activity_operation",
            Self::CombatAction(_) => "combat_action",
            Self::CombatWait(_) => "combat_wait",
            Self::CombatRuntime(_) => "combat_runtime",
            Self::PendingResponse(_) => "pending_response",
            Self::Response(_) => "response",
            Self::Request(_) => "request",
            Self::SessionMemory(_) => "session_memory",
            Self::Lookup(_) => "lookup",
            Self::Bridge(_) => "bridge",
            Self::NetResponseParse(_) => "net_response_parse",
            Self::ReturnCode(_) => "return_code",
            Self::HttpBusiness(_) => "http_business",
        }
    }
}

impl RocoErrorInfo {
    pub fn kind_code(&self) -> String {
        self.kind.kind_code()
    }

    pub fn detail_kind_code(&self) -> String {
        self.detail.kind_code().to_string()
    }

    pub fn network_kind_code(&self) -> String {
        match &self.kind {
            RocoErrorKind::Network { kind } => kind.code().to_string(),
            _ => String::new(),
        }
    }
}

impl RocoNetworkError {
    pub fn kind(&self) -> RocoNetworkErrorKind {
        match self {
            Self::ChannelClosed => RocoNetworkErrorKind::ChannelClosed,
            Self::HttpRequestFailed { .. } => RocoNetworkErrorKind::HttpRequestFailed,
            Self::HttpBridgeRequestFailed { .. } => RocoNetworkErrorKind::HttpBridgeRequestFailed,
            Self::NetBridgeRequestFailed { .. } => RocoNetworkErrorKind::NetBridgeRequestFailed,
            Self::NetResponseParseFailed { .. } => RocoNetworkErrorKind::NetResponseParseFailed,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::ChannelClosed => "network.channel_closed",
            Self::HttpRequestFailed { .. } => "network.http_request_failed",
            Self::HttpBridgeRequestFailed { .. } => "network.http_bridge_request_failed",
            Self::NetBridgeRequestFailed { .. } => "network.net_bridge_request_failed",
            Self::NetResponseParseFailed { .. } => "network.net_response_parse_failed",
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn info(&self) -> RocoErrorInfo {
        let detail = match self {
            Self::NetResponseParseFailed { target, source } => {
                RocoErrorDetail::NetResponseParse(RocoNetResponseParseFailure {
                    target: *target,
                    source: source.clone(),
                })
            }
            Self::HttpBridgeRequestFailed { bridge } | Self::NetBridgeRequestFailed { bridge } => {
                RocoErrorDetail::Bridge(bridge.clone())
            }
            _ => RocoErrorDetail::None,
        };
        RocoErrorInfo {
            kind: RocoErrorKind::Network { kind: self.kind() },
            code: self.code().to_string(),
            message: self.message(),
            detail,
        }
    }

    pub fn bridge_info(&self) -> Option<RocoBridgeErrorInfo> {
        match self {
            Self::HttpBridgeRequestFailed { bridge } | Self::NetBridgeRequestFailed { bridge } => {
                Some(bridge.clone())
            }
            _ => None,
        }
    }

    pub fn net_response_parse_failure(&self) -> Option<RocoNetResponseParseFailure> {
        match self {
            Self::NetResponseParseFailed { target, source } => Some(RocoNetResponseParseFailure {
                target: *target,
                source: source.clone(),
            }),
            _ => None,
        }
    }
}

impl fmt::Display for RocoNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChannelClosed => f.write_str("Channel closed"),
            Self::HttpRequestFailed { message } => write!(f, "HTTP request failed: {message}"),
            Self::HttpBridgeRequestFailed { bridge } => {
                write!(
                    f,
                    "HTTP bridge request failed [{}]: {}",
                    bridge.operation_code, bridge.message
                )
            }
            Self::NetBridgeRequestFailed { bridge } => {
                write!(
                    f,
                    "Net bridge request failed [{}]: {}",
                    bridge.operation_code, bridge.message
                )
            }
            Self::NetResponseParseFailed { target, source } => {
                write!(
                    f,
                    "failed to parse network response {}: {source}",
                    target.label()
                )
            }
        }
    }
}

impl fmt::Display for RocoNetResponseParseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol {
                error_type, reason, ..
            } => {
                write!(f, "{} ({})", reason.message(), error_type.code())
            }
            Self::UnexpectedCommand { cmd_id } => {
                write!(f, "unexpected command cmd_id=0x{cmd_id:x}")
            }
        }
    }
}
