use super::*;

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
