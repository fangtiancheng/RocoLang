use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoParamRange {
    Inclusive { min: i64, max: i64 },
    MinInclusive { min: i64 },
    TypeBounds { type_name: String },
}

impl fmt::Display for RocoParamRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive { min, max } => write!(f, "{min}..={max}"),
            Self::MinInclusive { min } => write!(f, ">={min}"),
            Self::TypeBounds { type_name } => write!(f, "valid {type_name} range"),
        }
    }
}

impl RocoParamRange {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::Inclusive { .. } => "inclusive",
            Self::MinInclusive { .. } => "min_inclusive",
            Self::TypeBounds { .. } => "type_bounds",
        }
    }

    pub fn min_value(&self) -> i64 {
        match self {
            Self::Inclusive { min, .. } | Self::MinInclusive { min } => *min,
            Self::TypeBounds { .. } => 0,
        }
    }

    pub fn max_value(&self) -> i64 {
        match self {
            Self::Inclusive { max, .. } => *max,
            _ => 0,
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::TypeBounds { type_name } => type_name.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamError {
    OutOfRange {
        name: String,
        value: i64,
        expected: RocoParamRange,
    },
    MustBePositive {
        name: String,
        value: i64,
    },
    MustBeNonZero {
        name: String,
        value: i64,
    },
    RhaiTypeMismatch {
        name: String,
        message: String,
    },
    Rejected {
        name: String,
        source_code: String,
        message: String,
    },
    ProtocolRejected {
        name: String,
        field: RocoProtocolFieldName,
        value: i64,
        expected: RocoParamRange,
    },
    InvalidTimestamp {
        value: i64,
        failure: ScriptSystemFailure,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamKind {
    OutOfRange,
    MustBePositive,
    MustBeNonZero,
    RhaiTypeMismatch,
    Rejected,
    ProtocolRejected,
    InvalidTimestamp,
}

impl RocoInvalidParamKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::OutOfRange => "out_of_range",
            Self::MustBePositive => "must_be_positive",
            Self::MustBeNonZero => "must_be_non_zero",
            Self::RhaiTypeMismatch => "rhai_type_mismatch",
            Self::Rejected => "rejected",
            Self::ProtocolRejected => "protocol_rejected",
            Self::InvalidTimestamp => "invalid_timestamp",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoInvalidParamInfo {
    pub kind: RocoInvalidParamKind,
    pub name: String,
    pub value: i64,
    pub detail: RocoInvalidParamDetail,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoInvalidParamDetail {
    None,
    Rejected {
        source_code: String,
    },
    ExpectedRange(RocoParamRange),
    ProtocolRejected {
        field: RocoProtocolFieldName,
        expected: RocoParamRange,
    },
    SystemFailure(ScriptSystemFailure),
}

impl RocoInvalidParamDetail {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Rejected { .. } => "rejected",
            Self::ExpectedRange(_) => "expected_range",
            Self::ProtocolRejected { .. } => "protocol_rejected",
            Self::SystemFailure(_) => "system_failure",
        }
    }

    pub fn expected_text(&self) -> String {
        match self {
            Self::ExpectedRange(expected) | Self::ProtocolRejected { expected, .. } => {
                expected.to_string()
            }
            _ => String::new(),
        }
    }

    pub fn protocol_field(&self) -> String {
        match self {
            Self::ProtocolRejected { field, .. } => field.code().to_string(),
            _ => String::new(),
        }
    }

    pub fn protocol_field_name(&self) -> Option<RocoProtocolFieldName> {
        match self {
            Self::ProtocolRejected { field, .. } => Some(field.clone()),
            _ => None,
        }
    }

    pub fn rejected_source_code(&self) -> String {
        match self {
            Self::Rejected { source_code } => source_code.clone(),
            _ => String::new(),
        }
    }

    pub fn system_operation_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn system_source_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.source_code(),
            _ => String::new(),
        }
    }

    pub fn system_message(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.message(),
            _ => String::new(),
        }
    }
}

impl RocoInvalidParamInfo {
    pub fn kind_code(&self) -> String {
        self.kind.code().to_string()
    }

    pub fn detail_kind_code(&self) -> String {
        self.detail.kind_code().to_string()
    }

    pub fn system_operation_code(&self) -> String {
        self.detail.system_operation_code()
    }

    pub fn system_source_code(&self) -> String {
        self.detail.system_source_code()
    }

    pub fn expected_text(&self) -> String {
        self.detail.expected_text()
    }

    pub fn protocol_field(&self) -> String {
        self.detail.protocol_field()
    }

    pub fn protocol_field_name(&self) -> Option<RocoProtocolFieldName> {
        self.detail.protocol_field_name()
    }

    pub fn rejected_source_code(&self) -> String {
        self.detail.rejected_source_code()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoProtocolFieldName {
    code: String,
}

impl RocoProtocolFieldName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for RocoProtocolFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl RocoInvalidParamError {
    pub fn type_bounds(name: impl Into<String>, value: i64, type_name: impl Into<String>) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::TypeBounds {
                type_name: type_name.into(),
            },
        }
    }

    pub fn inclusive(name: impl Into<String>, value: i64, min: i64, max: i64) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::Inclusive { min, max },
        }
    }

    pub fn min_inclusive(name: impl Into<String>, value: i64, min: i64) -> Self {
        Self::OutOfRange {
            name: name.into(),
            value,
            expected: RocoParamRange::MinInclusive { min },
        }
    }

    pub fn info(&self) -> RocoInvalidParamInfo {
        match self {
            Self::OutOfRange {
                name,
                value,
                expected,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::OutOfRange,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(expected.clone()),
                message: expected.to_string(),
            },
            Self::MustBePositive { name, value } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::MustBePositive,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(RocoParamRange::MinInclusive {
                    min: 1,
                }),
                message: String::new(),
            },
            Self::MustBeNonZero { name, value } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::MustBeNonZero,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ExpectedRange(RocoParamRange::MinInclusive {
                    min: 1,
                }),
                message: String::new(),
            },
            Self::RhaiTypeMismatch { name, message } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::RhaiTypeMismatch,
                name: name.clone(),
                value: 0,
                detail: RocoInvalidParamDetail::None,
                message: message.clone(),
            },
            Self::Rejected {
                name,
                source_code,
                message,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::Rejected,
                name: name.clone(),
                value: 0,
                detail: RocoInvalidParamDetail::Rejected {
                    source_code: source_code.clone(),
                },
                message: message.clone(),
            },
            Self::ProtocolRejected {
                name,
                field,
                value,
                expected,
            } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::ProtocolRejected,
                name: name.clone(),
                value: *value,
                detail: RocoInvalidParamDetail::ProtocolRejected {
                    field: field.clone(),
                    expected: expected.clone(),
                },
                message: String::new(),
            },
            Self::InvalidTimestamp { value, failure } => RocoInvalidParamInfo {
                kind: RocoInvalidParamKind::InvalidTimestamp,
                name: "timestamp".to_string(),
                value: *value,
                detail: RocoInvalidParamDetail::SystemFailure(failure.clone()),
                message: failure.message(),
            },
        }
    }
}

impl fmt::Display for RocoInvalidParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                name,
                value,
                expected,
            } => write!(f, "{name} out of range: {value}, expected {expected}"),
            Self::MustBePositive { name, value } => {
                write!(f, "{name} must be positive: {value}")
            }
            Self::MustBeNonZero { name, value } => {
                write!(f, "{name} must be non-zero: {value}")
            }
            Self::RhaiTypeMismatch { name, message } => {
                write!(f, "{name} has invalid Rhai type: {message}")
            }
            Self::Rejected {
                name,
                source_code,
                message,
            } => write!(f, "{name} rejected by {source_code}: {message}"),
            Self::ProtocolRejected {
                name,
                field,
                value,
                expected,
            } => {
                write!(
                    f,
                    "{name} rejected by protocol field {field}: {value}, expected {expected}"
                )
            }
            Self::InvalidTimestamp { value, failure } => {
                write!(f, "invalid timestamp {value}: {}", failure.message())
            }
        }
    }
}
