//! Error types for RocoLang.

use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = std::result::Result<T, RocoError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptErrorKind {
    Parse,
    Runtime,
    FunctionCall,
    Module,
    Terminated,
    Eval,
    Other,
}

impl RocoScriptErrorKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Runtime => "runtime",
            Self::FunctionCall => "function_call",
            Self::Module => "module",
            Self::Terminated => "terminated",
            Self::Eval => "eval",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for RocoScriptErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptPosition {
    Line { line: usize },
    LineColumn { line: usize, column: usize },
}

impl RocoScriptPosition {
    pub const fn line(&self) -> usize {
        match self {
            Self::Line { line } | Self::LineColumn { line, .. } => *line,
        }
    }

    pub const fn column(&self) -> Option<usize> {
        match self {
            Self::Line { .. } => None,
            Self::LineColumn { column, .. } => Some(*column),
        }
    }
}

impl fmt::Display for RocoScriptPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line { line } => write!(f, "{line}"),
            Self::LineColumn { line, column } => write!(f, "{line}:{column}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoScriptLocation {
    Unknown,
    Anonymous {
        position: RocoScriptPosition,
    },
    Source {
        source: String,
        position: RocoScriptPosition,
    },
}

impl RocoScriptLocation {
    pub const fn position(&self) -> Option<&RocoScriptPosition> {
        match self {
            Self::Unknown => None,
            Self::Anonymous { position } | Self::Source { position, .. } => Some(position),
        }
    }

    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Source { source, .. } => Some(source),
            Self::Unknown | Self::Anonymous { .. } => None,
        }
    }

    pub fn parts(&self) -> (Option<&str>, Option<usize>, Option<usize>) {
        match self {
            Self::Unknown => (None, None, None),
            Self::Anonymous { position } => (None, Some(position.line()), position.column()),
            Self::Source { source, position } => (
                Some(source.as_str()),
                Some(position.line()),
                position.column(),
            ),
        }
    }
}

impl fmt::Display for RocoScriptLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => f.write_str("unknown"),
            Self::Anonymous { position } => write!(f, "{position}"),
            Self::Source { source, position } => write!(f, "{source}:{position}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoScriptError {
    pub kind: RocoScriptErrorKind,
    pub message: String,
    pub location: RocoScriptLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoStdLibError {
    FunctionContext(ScriptFunctionContextError),
    Query(ScriptQueryError),
    CombatAction(ScriptCombatActionError),
    CombatRuntime(ScriptCombatRuntimeError),
    CombatWait(ScriptCombatWaitError),
    PendingResponse(ScriptPendingResponseError),
    Lookup(ScriptLookupError),
    SessionMemory(ScriptSessionMemoryError),
    StaticData(ScriptStaticDataError),
    SpiritOperation(ScriptSpiritOperationError),
    System(ScriptSystemError),
    ActivityOperation(ScriptActivityOperationError),
    Bridge(ScriptBridgeError),
    Response(ScriptResponseError),
    Message(String),
}

impl fmt::Display for RocoStdLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionContext(error) => write!(f, "{error}"),
            Self::Query(error) => write!(f, "{error}"),
            Self::CombatAction(error) => write!(f, "{error}"),
            Self::CombatRuntime(error) => write!(f, "{error}"),
            Self::CombatWait(error) => write!(f, "{error}"),
            Self::PendingResponse(error) => write!(f, "{error}"),
            Self::Lookup(error) => write!(f, "{error}"),
            Self::SessionMemory(error) => write!(f, "{error}"),
            Self::StaticData(error) => write!(f, "{error}"),
            Self::SpiritOperation(error) => write!(f, "{error}"),
            Self::System(error) => write!(f, "{error}"),
            Self::ActivityOperation(error) => write!(f, "{error}"),
            Self::Bridge(error) => write!(f, "{error}"),
            Self::Response(error) => write!(f, "{error}"),
            Self::Message(message) => f.write_str(message),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptFunctionContextError {
    RequiresActiveCombat,
    NoCombatAvailable,
    CannotWaitForCombat { phase: ScriptCombatPhase },
    RequiresOutOfCombat,
}

impl fmt::Display for ScriptFunctionContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresActiveCombat => f.write_str("This function can only be used in combat"),
            Self::NoCombatAvailable => f.write_str("No combat is available to wait for"),
            Self::CannotWaitForCombat { phase } => {
                write!(f, "Cannot wait for combat in phase {}", phase.as_str())
            }
            Self::RequiresOutOfCombat => f.write_str("This function cannot be used during combat"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatPhase {
    Idle,
    WaitingStartReply,
    WaitingLocalReady,
    WaitingPeerReady,
    PlayingOpening,
    WaitingPlayerAction,
    WaitingRoundResult,
    PlayingRoundResult,
    WaitingRoundRelease,
    WaitingMyExtraSwitch,
    WaitingOpponentExtraSwitch,
    Finished,
    Aborted,
    Unknown,
}

impl ScriptCombatPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::WaitingStartReply => "WaitingStartReply",
            Self::WaitingLocalReady => "WaitingLocalReady",
            Self::WaitingPeerReady => "WaitingPeerReady",
            Self::PlayingOpening => "PlayingOpening",
            Self::WaitingPlayerAction => "WaitingPlayerAction",
            Self::WaitingRoundResult => "WaitingRoundResult",
            Self::PlayingRoundResult => "PlayingRoundResult",
            Self::WaitingRoundRelease => "WaitingRoundRelease",
            Self::WaitingMyExtraSwitch => "WaitingMyExtraSwitch",
            Self::WaitingOpponentExtraSwitch => "WaitingOpponentExtraSwitch",
            Self::Finished => "Finished",
            Self::Aborted => "Aborted",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptQueryError {
    NotInAnyScene,
    SceneSpiritSnapshotUnavailable,
    CurrentRoleSnapshotUnavailable,
    BattleResultBeforeCombatStarts,
    BattleResultBeforeCombatFinishes,
    NotInCombat,
    InvalidActiveSpiritIndex,
    InvalidActiveRivalSpiritIndex,
    NoActiveSpirit,
    NoActiveRivalSpirit,
    NoSkillAtIndex { index: u32 },
}

impl fmt::Display for ScriptQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInAnyScene => f.write_str("Not in any scene"),
            Self::SceneSpiritSnapshotUnavailable => {
                f.write_str("scene spirit snapshot unavailable")
            }
            Self::CurrentRoleSnapshotUnavailable => {
                f.write_str("current role snapshot unavailable")
            }
            Self::BattleResultBeforeCombatStarts => {
                f.write_str("battle result is unavailable before combat starts")
            }
            Self::BattleResultBeforeCombatFinishes => {
                f.write_str("battle result is unavailable before combat finishes")
            }
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex => f.write_str("Invalid active spirit index"),
            Self::InvalidActiveRivalSpiritIndex => f.write_str("Invalid active rival spirit index"),
            Self::NoActiveSpirit => f.write_str("No active spirit"),
            Self::NoActiveRivalSpirit => f.write_str("No active rival spirit"),
            Self::NoSkillAtIndex { index } => write!(f, "No skill at index {index}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionError {
    NotInCombat,
    InvalidActiveSpiritIndex { position: u8 },
    CannotUseSkill,
    SkillUnavailableOrNoPp { skill_id: u32 },
    CannotUseItem,
    CombatItemUnavailable { item_id: u32 },
    CannotChangeSpirit,
    CannotEscape,
    SpiritIndexOutOfRange { position: u32 },
    InvalidTargetSpiritIndex { position: u8 },
    TargetSpiritFainted { position: u8 },
    TargetSpiritNotFound { position: u8 },
}

impl fmt::Display for ScriptCombatActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInCombat => f.write_str("Not in combat"),
            Self::InvalidActiveSpiritIndex { position } => {
                write!(f, "Invalid active spirit index: {position}")
            }
            Self::CannotUseSkill => f.write_str("cannot use skill in current combat state"),
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::CannotUseItem => f.write_str("cannot use item in current combat state"),
            Self::CombatItemUnavailable { item_id } => {
                write!(f, "combat item unavailable: item_id={item_id}")
            }
            Self::CannotChangeSpirit => f.write_str("cannot change spirit in current combat state"),
            Self::CannotEscape => f.write_str("cannot escape in current combat state"),
            Self::SpiritIndexOutOfRange { position } => {
                write!(f, "spirit_index out of range: {position}")
            }
            Self::InvalidTargetSpiritIndex { position } => {
                write!(f, "invalid target spirit index: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target index: {position}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatRuntimeError {
    AutoFinishPresentationFailed { message: String },
    MarkFrontendLoadedFailed { message: String },
}

impl fmt::Display for ScriptCombatRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AutoFinishPresentationFailed { message } => {
                write!(
                    f,
                    "failed to auto-finish script combat presentation: {message}"
                )
            }
            Self::MarkFrontendLoadedFailed { message } => {
                write!(f, "failed to mark script combat frontend loaded: {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatWaitError {
    TimeoutWaitingRoundSettled {
        phase: ScriptCombatPhase,
        elapsed_ms: u128,
    },
}

impl fmt::Display for ScriptCombatWaitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeoutWaitingRoundSettled { phase, elapsed_ms } => write!(
                f,
                "timeout waiting for combat round to settle, phase {}, elapsed_ms={elapsed_ms}",
                phase.as_str()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionMemoryError {
    TypeMismatch {
        key: String,
        expected: ScriptSessionValueKind,
        actual: ScriptSessionValueKind,
    },
}

impl fmt::Display for ScriptSessionMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch {
                key,
                expected,
                actual,
            } => write!(
                f,
                "session key '{key}' has different type: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSessionValueKind {
    Integer,
    String,
    Bool,
}

impl ScriptSessionValueKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Integer => "integer",
            Self::String => "string",
            Self::Bool => "bool",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupError {
    NotFound {
        entity: ScriptLookupEntity,
        key: String,
    },
}

impl fmt::Display for ScriptLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { entity, key } => {
                write!(f, "{} not found: {key}", entity.label())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupEntity {
    ItemInfo,
    StriveItemInfo,
    GuardianPetPropertyInfo,
    TitleInfo,
    MagicInfo,
    PluginInfo,
    TalentInfo,
    SkillInfo,
    SpiritInfo,
}

impl ScriptLookupEntity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ItemInfo => "item info",
            Self::StriveItemInfo => "strive item info",
            Self::GuardianPetPropertyInfo => "guardian pet property info",
            Self::TitleInfo => "title info",
            Self::MagicInfo => "magic info",
            Self::PluginInfo => "plugin info",
            Self::TalentInfo => "talent info",
            Self::SkillInfo => "skill info",
            Self::SpiritInfo => "spirit info",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptStaticDataError {
    BagSpiritNotFound { position: u8 },
    StaticGameDataNotInitialized,
    ActiveConfigNotAvailable { message: String },
    NotImplemented { function_name: String },
}

impl fmt::Display for ScriptStaticDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BagSpiritNotFound { position } => {
                write!(f, "bag spirit not found at position {position}")
            }
            Self::StaticGameDataNotInitialized => f.write_str("static game data not initialized"),
            Self::ActiveConfigNotAvailable { message } => {
                write!(f, "active config not available: {message}")
            }
            Self::NotImplemented { function_name } => {
                write!(f, "{function_name} not yet implemented")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSpiritOperationError {
    RecoverAllRequiresVip,
    StorageSpiritNotFound { spirit_id: u32, catch_time: u32 },
    MultipleStorageSpiritsMatch { spirit_id: u32 },
    StorageSpiritCatchTimeOutOfRange { catch_time: i64 },
}

impl fmt::Display for ScriptSpiritOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecoverAllRequiresVip => f.write_str("recover_all_spirits requires VIP"),
            Self::StorageSpiritNotFound {
                spirit_id,
                catch_time,
            } => write!(
                f,
                "storage spirit not found: spirit_id={spirit_id} catch_time={catch_time}"
            ),
            Self::MultipleStorageSpiritsMatch { spirit_id } => write!(
                f,
                "multiple storage spirits match spirit_id={spirit_id}; pass catch_time to disambiguate"
            ),
            Self::StorageSpiritCatchTimeOutOfRange { catch_time } => {
                write!(f, "storage spirit catch_time out of u32 range: {catch_time}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSystemError {
    CurrentTimeBeforeUnixEpoch { message: String },
    CurrentTimestampExceedsI64,
    BuildTimeOffsetFailed { message: String },
    ParseTimeFormatFailed { message: String },
    FormatTimestampFailed { message: String },
    SendLogEventFailed { message: String },
}

impl fmt::Display for ScriptSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentTimeBeforeUnixEpoch { message } => {
                write!(f, "current time before unix epoch: {message}")
            }
            Self::CurrentTimestampExceedsI64 => f.write_str("current timestamp exceeds i64 range"),
            Self::BuildTimeOffsetFailed { message } => write!(f, "build UTC+8 offset: {message}"),
            Self::ParseTimeFormatFailed { message } => write!(f, "parse time format: {message}"),
            Self::FormatTimestampFailed { message } => write!(f, "format timestamp: {message}"),
            Self::SendLogEventFailed { message } => {
                write!(f, "Failed to send log event: {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityOperationError {
    MysteryFusionMaterialCountExceedsLimit { count: usize, limit: usize },
    SummonDrawCountInvalid { count: i64 },
}

impl fmt::Display for ScriptActivityOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { count, limit } => write!(
                f,
                "mystery_fusion::fuse material count {count} exceeds AS limit {limit}"
            ),
            Self::SummonDrawCountInvalid { count } => {
                write!(f, "summon::draw draw_count must be 1 or 10, got {count}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBridgeError {
    SendRequestFailed { message: String },
}

impl fmt::Display for ScriptBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendRequestFailed { message } => write!(f, "Failed to send request: {message}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptResponseError {
    TypeMismatch { expected: String, actual: String },
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptPendingResponseError {
    ExternalGameExtraFieldParseFailed {
        key: String,
        value: String,
        message: String,
    },
    UnexpectedHttpResponse {
        pending: String,
        expected: String,
        actual: String,
    },
    CombatLoadedUnexpectedPhase {
        phase: ScriptCombatPhase,
    },
    CombatActionAckMismatch {
        expected_req_type: u8,
        expected_spirit_index: u8,
        actual_req_type: u8,
        actual_spirit_index: u8,
    },
    MissingScriptSessionForActionWait,
    StorageSpiritDetailEmpty,
}

impl fmt::Display for ScriptPendingResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExternalGameExtraFieldParseFailed {
                key,
                value,
                message,
            } => write!(
                f,
                "parse external_game extra field {key}={value:?} failed: {message}"
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
                expected_req_type,
                expected_spirit_index,
                actual_req_type,
                actual_spirit_index,
            } => write!(
                f,
                "combat action ack mismatch: expected req_type={expected_req_type} spirit_index={expected_spirit_index}, got req_type={actual_req_type} spirit_index={actual_spirit_index}"
            ),
            Self::MissingScriptSessionForActionWait => {
                f.write_str("script action wait requested while no script is running")
            }
            Self::StorageSpiritDetailEmpty => f.write_str("storage spirit detail is empty"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RocoError {
    ScriptError(RocoScriptError),
    StdLib(RocoStdLibError),
    StdLibError(String),
    InvalidParam(String),
    NetworkError(String),
    TimeoutError(String),
    ServerRejected(String),
    AssertionError(String),
    Other(String),
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
                write!(f, ": [{}] {}", error.kind, error.message)
            }
            RocoError::StdLib(error) => write!(f, "StdLib error: {}", error),
            RocoError::StdLibError(msg) => write!(f, "StdLib error: {}", msg),
            RocoError::InvalidParam(msg) => write!(f, "Invalid parameter: {}", msg),
            RocoError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RocoError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            RocoError::ServerRejected(msg) => write!(f, "Server rejected: {}", msg),
            RocoError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
            RocoError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RocoError {}

impl From<String> for RocoError {
    fn from(s: String) -> Self {
        RocoError::Other(s)
    }
}

impl From<&str> for RocoError {
    fn from(s: &str) -> Self {
        RocoError::Other(s.to_string())
    }
}

impl From<RocoStdLibError> for RocoError {
    fn from(error: RocoStdLibError) -> Self {
        RocoError::StdLib(error)
    }
}

impl From<String> for RocoStdLibError {
    fn from(message: String) -> Self {
        RocoStdLibError::Message(message)
    }
}

impl From<&str> for RocoStdLibError {
    fn from(message: &str) -> Self {
        RocoStdLibError::Message(message.to_string())
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
