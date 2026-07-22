use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeError {
    SendRequestFailed { failure: ScriptBridgeFailure },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptBridgeFailure {
    pub operation: ScriptBridgeOperation,
    pub reason: ScriptBridgeFailureReason,
}

impl ScriptBridgeFailure {
    pub fn send_request(reason: ScriptBridgeFailureReason) -> Self {
        Self {
            operation: ScriptBridgeOperation::SendRequest,
            reason,
        }
    }

    pub fn operation_code(&self) -> String {
        self.operation.code().to_string()
    }

    pub fn reason_code(&self) -> String {
        self.reason.code().to_string()
    }

    pub fn message(&self) -> String {
        self.reason.message().to_string()
    }

    pub fn description(&self) -> String {
        format!("{} failed: {}", self.operation, self.message())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeOperation {
    SendRequest,
}

impl ScriptBridgeOperation {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SendRequest => "send_request",
        }
    }
}

impl fmt::Display for ScriptBridgeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeFailureReason {
    RequestChannelClosed,
}

impl ScriptBridgeFailureReason {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::RequestChannelClosed => "request_channel_closed",
        }
    }

    pub const fn message(&self) -> &'static str {
        match self {
            Self::RequestChannelClosed => "request channel closed",
        }
    }
}

impl fmt::Display for ScriptBridgeFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl fmt::Display for ScriptBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendRequestFailed { failure } => {
                write!(f, "Failed to send request: {}", failure.message())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptResponseName {
    pub code: String,
}

impl ScriptResponseName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl fmt::Display for ScriptResponseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptResponseError {
    TypeMismatch {
        expected: ScriptResponseName,
        actual: ScriptResponseName,
    },
}

impl fmt::Display for ScriptResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Expected {expected} response, got {actual}")
            }
        }
    }
}

impl ScriptResponseError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::TypeMismatch { .. } => "type_mismatch",
        }
    }

    pub fn expected_response_code(&self) -> String {
        match self {
            Self::TypeMismatch { expected, .. } => expected.code.clone(),
        }
    }

    pub fn actual_response_code(&self) -> String {
        match self {
            Self::TypeMismatch { actual, .. } => actual.code.clone(),
        }
    }
}
