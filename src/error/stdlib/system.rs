use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemError {
    CurrentTimeBeforeUnixEpoch { failure: ScriptSystemFailure },
    CurrentTimestampExceedsI64,
    BuildTimeOffsetFailed { failure: ScriptSystemFailure },
    ParseTimeFormatFailed { failure: ScriptSystemFailure },
    FormatTimestampFailed { failure: ScriptSystemFailure },
    SendLogEventFailed { failure: ScriptSystemFailure },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptSystemFailure {
    pub operation: ScriptSystemOperation,
    pub source: ScriptSystemFailureSource,
}

impl ScriptSystemFailure {
    pub const fn new(operation: ScriptSystemOperation, source: ScriptSystemFailureSource) -> Self {
        Self { operation, source }
    }

    pub fn external(operation: ScriptSystemOperation, message: impl Into<String>) -> Self {
        Self::new(
            operation,
            ScriptSystemFailureSource::External {
                message: message.into(),
            },
        )
    }

    pub fn operation_code(&self) -> String {
        self.operation.code().to_string()
    }

    pub fn source_code(&self) -> String {
        self.source.code().to_string()
    }

    pub fn message(&self) -> String {
        self.source.message()
    }

    pub fn description(&self) -> String {
        format!("{} failed: {}", self.operation, self.message())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemFailureSource {
    SystemTimeBeforeUnixEpoch {
        seconds: u64,
        nanos: u32,
    },
    TimeComponentRange {
        component: String,
        conditional_range: bool,
        message: String,
    },
    TimeFormatDescription {
        message: String,
    },
    TimeFormat {
        message: String,
    },
    SendLogEvent {
        message: String,
    },
    External {
        message: String,
    },
}

impl ScriptSystemFailureSource {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SystemTimeBeforeUnixEpoch { .. } => "system_time_before_unix_epoch",
            Self::TimeComponentRange { .. } => "time_component_range",
            Self::TimeFormatDescription { .. } => "time_format_description",
            Self::TimeFormat { .. } => "time_format",
            Self::SendLogEvent { .. } => "send_log_event",
            Self::External { .. } => "external",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::SystemTimeBeforeUnixEpoch { seconds, nanos } => {
                format!("system time is {seconds}s {nanos}ns before unix epoch")
            }
            Self::TimeComponentRange {
                component,
                conditional_range,
                message,
            } => format!("{component} component out of range: {message}; conditional_range={conditional_range}"),
            Self::TimeFormatDescription { message }
            | Self::TimeFormat { message }
            | Self::SendLogEvent { message }
            | Self::External { message } => message.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemOperation {
    CurrentTimeBeforeUnixEpoch,
    BuildTimeOffset,
    ParseTimeFormat,
    FormatTimestamp,
    SendLogEvent,
}

impl ScriptSystemOperation {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CurrentTimeBeforeUnixEpoch => "current_time_before_unix_epoch",
            Self::BuildTimeOffset => "build_time_offset",
            Self::ParseTimeFormat => "parse_time_format",
            Self::FormatTimestamp => "format_timestamp",
            Self::SendLogEvent => "send_log_event",
        }
    }
}

impl fmt::Display for ScriptSystemOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl ScriptSystemError {
    pub fn failure(&self) -> Option<ScriptSystemFailure> {
        match self {
            Self::CurrentTimeBeforeUnixEpoch { failure }
            | Self::BuildTimeOffsetFailed { failure }
            | Self::ParseTimeFormatFailed { failure }
            | Self::FormatTimestampFailed { failure }
            | Self::SendLogEventFailed { failure } => Some(failure.clone()),
            Self::CurrentTimestampExceedsI64 => None,
        }
    }
}

impl fmt::Display for ScriptSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentTimeBeforeUnixEpoch { failure } => {
                write!(f, "current time before unix epoch: {}", failure.message())
            }
            Self::CurrentTimestampExceedsI64 => f.write_str("current timestamp exceeds i64 range"),
            Self::BuildTimeOffsetFailed { failure } => {
                write!(f, "build UTC+8 offset: {}", failure.message())
            }
            Self::ParseTimeFormatFailed { failure } => {
                write!(f, "parse time format: {}", failure.message())
            }
            Self::FormatTimestampFailed { failure } => {
                write!(f, "format timestamp: {}", failure.message())
            }
            Self::SendLogEventFailed { failure } => {
                write!(f, "Failed to send log event: {}", failure.message())
            }
        }
    }
}
