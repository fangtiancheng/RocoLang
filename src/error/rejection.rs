use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoTimeoutError {
    WaitingForResponse { serial_num: u32, timeout_ms: i64 },
}

impl RocoTimeoutError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RocoTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaitingForResponse {
                serial_num,
                timeout_ms,
            } => write!(
                f,
                "Timeout waiting for response (serial_num={serial_num}, timeout={timeout_ms}ms)"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoReturnCodeKind {
    GenericFailure,
    SafeCodeRequired,
    RoleUnavailable,
    CommandFailure { code: i32 },
    Unrecognized { code: i32 },
}

impl RocoReturnCodeKind {
    pub fn code(&self) -> i32 {
        match self {
            Self::GenericFailure => -1,
            Self::SafeCodeRequired => -4,
            Self::RoleUnavailable => -7,
            Self::CommandFailure { code } => *code,
            Self::Unrecognized { code } => *code,
        }
    }

    pub fn kind_code(&self) -> &'static str {
        match self {
            Self::GenericFailure => "generic_failure",
            Self::SafeCodeRequired => "safe_code_required",
            Self::RoleUnavailable => "role_unavailable",
            Self::CommandFailure { .. } => "command_failure",
            Self::Unrecognized { .. } => "unrecognized",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::GenericFailure => "generic failure",
            Self::SafeCodeRequired => "safe code required",
            Self::RoleUnavailable => "role unavailable",
            Self::CommandFailure { .. } => "command rejected",
            Self::Unrecognized { .. } => "unrecognized return code",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoReturnCodeRejection {
    pub kind: RocoReturnCodeKind,
    pub message: String,
}

impl RocoReturnCodeRejection {
    pub fn code(&self) -> i32 {
        self.kind.code()
    }

    pub fn kind_code(&self) -> &'static str {
        self.kind.kind_code()
    }

    pub fn description(&self) -> String {
        if self.message.is_empty() {
            format!("{} ({})", self.kind.label(), self.code())
        } else {
            format!("{} ({}): {}", self.kind.label(), self.code(), self.message)
        }
    }
}

impl fmt::Display for RocoReturnCodeRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoHttpBusinessRejection {
    pub result_code: i32,
    pub message: String,
    pub request_context: Option<RocoRequestContext>,
}

impl RocoHttpBusinessRejection {
    pub fn result_code(&self) -> i64 {
        i64::from(self.result_code)
    }

    pub fn request_context(&self) -> String {
        self.request_context
            .as_ref()
            .map(|context| context.raw.clone())
            .unwrap_or_default()
    }

    pub fn description(&self) -> String {
        let context = self.request_context();
        if context.is_empty() {
            format!(
                "HTTP business result_code={}: {}",
                self.result_code, self.message
            )
        } else {
            format!(
                "HTTP business {} result_code={}: {}",
                context, self.result_code, self.message
            )
        }
    }
}

impl fmt::Display for RocoHttpBusinessRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoServerRejectedError {
    ReturnCode {
        rejection: RocoReturnCodeRejection,
    },
    HttpBusiness {
        rejection: RocoHttpBusinessRejection,
    },
    HttpResponse {
        message: String,
    },
}

impl RocoServerRejectedError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ReturnCode { .. } => "server_rejected.return_code",
            Self::HttpBusiness { .. } => "server_rejected.http_business",
            Self::HttpResponse { .. } => "server_rejected.http_response",
        }
    }

    pub fn return_code(&self) -> Option<RocoReturnCodeRejection> {
        match self {
            Self::ReturnCode { rejection } => Some(rejection.clone()),
            _ => None,
        }
    }

    pub fn http_business(&self) -> Option<RocoHttpBusinessRejection> {
        match self {
            Self::HttpBusiness { rejection } => Some(rejection.clone()),
            _ => None,
        }
    }
}

impl fmt::Display for RocoServerRejectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnCode { rejection } => write!(f, "return code rejected: {rejection}"),
            Self::HttpBusiness { rejection } => {
                write!(f, "HTTP business rejected: {rejection}")
            }
            Self::HttpResponse { message } => write!(f, "HTTP response rejected: {message}"),
        }
    }
}
