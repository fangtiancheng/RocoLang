use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoBridgeErrorChannel {
    Http,
    Net,
}

impl RocoBridgeErrorChannel {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Net => "net",
        }
    }
}

impl fmt::Display for RocoBridgeErrorChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoHttpBridgeErrorKind {
    Validation,
    Timeout,
    Transport,
    Url,
    HttpStatus,
    Parse,
    Business,
    StaleSession,
    ReceiverDropped,
    RuntimeStopped,
    SessionNotStarted,
    DuplicatePendingRequest,
    PreloginSession,
    Dispatch,
    System,
}

impl RocoHttpBridgeErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Timeout => "timeout",
            Self::Transport => "transport",
            Self::Url => "url",
            Self::HttpStatus => "http_status",
            Self::Parse => "parse",
            Self::Business => "business",
            Self::StaleSession => "stale_session",
            Self::ReceiverDropped => "receiver_dropped",
            Self::RuntimeStopped => "runtime_stopped",
            Self::SessionNotStarted => "session_not_started",
            Self::DuplicatePendingRequest => "duplicate_pending_request",
            Self::PreloginSession => "prelogin_session",
            Self::Dispatch => "dispatch",
            Self::System => "system",
        }
    }
}

impl fmt::Display for RocoHttpBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetBridgeErrorKind {
    Session,
    Timeout,
    Transport,
    Protocol,
    Validation,
    Business,
    Dispatch,
}

impl RocoNetBridgeErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Timeout => "timeout",
            Self::Transport => "transport",
            Self::Protocol => "protocol",
            Self::Validation => "validation",
            Self::Business => "business",
            Self::Dispatch => "dispatch",
        }
    }
}

impl fmt::Display for RocoNetBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoBridgeErrorKind {
    Http(RocoHttpBridgeErrorKind),
    Net(RocoNetBridgeErrorKind),
}

impl RocoBridgeErrorKind {
    pub const fn channel(&self) -> RocoBridgeErrorChannel {
        match self {
            Self::Http(_) => RocoBridgeErrorChannel::Http,
            Self::Net(_) => RocoBridgeErrorChannel::Net,
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Self::Http(kind) => kind.code(),
            Self::Net(kind) => kind.code(),
        }
    }
}

impl fmt::Display for RocoBridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoBridgeErrorInfo {
    pub channel: RocoBridgeErrorChannel,
    pub kind: RocoBridgeErrorKind,
    pub operation_code: String,
    pub message: String,
}

impl RocoBridgeErrorInfo {
    pub fn http(
        kind: RocoHttpBridgeErrorKind,
        operation_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            channel: RocoBridgeErrorChannel::Http,
            kind: RocoBridgeErrorKind::Http(kind),
            operation_code: operation_code.into(),
            message: message.into(),
        }
    }

    pub fn net(
        kind: RocoNetBridgeErrorKind,
        operation_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            channel: RocoBridgeErrorChannel::Net,
            kind: RocoBridgeErrorKind::Net(kind),
            operation_code: operation_code.into(),
            message: message.into(),
        }
    }

    pub fn kind_code(&self) -> String {
        self.kind.code().to_string()
    }

    pub fn channel_code(&self) -> String {
        self.channel.code().to_string()
    }

    pub fn operation_code(&self) -> String {
        self.operation_code.clone()
    }

    pub fn description(&self) -> String {
        format!(
            "{} bridge error [{}]: {}",
            self.channel, self.operation_code, self.message
        )
    }
}
