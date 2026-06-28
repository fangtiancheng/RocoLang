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
    Request(ScriptRequestError),
    Unsupported(ScriptUnsupportedError),
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
            Self::Request(error) => write!(f, "{error}"),
            Self::Unsupported(error) => write!(f, "{error}"),
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
    AutoFinishPresentationFailed {
        message: String,
    },
    MarkFrontendLoadedFailed {
        message: String,
    },
    Backend {
        kind: ScriptBackendCombatRuntimeErrorKind,
        message: String,
    },
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
            Self::Backend { kind, message } => {
                write!(f, "combat runtime {}: {message}", kind.as_str())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptBackendCombatRuntimeErrorKind {
    BattleFacts,
    SideResolution,
    InvalidReadyPhasePayload,
    MissingStartContext,
    InvalidPhase,
    MissingStartSummary,
    MissingSideRegistry,
    MissingHistoryRecorder,
    MissingBattleFactsForHistorySnapshot,
    MissingObservedInitialStateForRoundHistory,
    MissingRoundBarrierForPresentation,
    HistoryRecorder,
    HistoryProjection,
    ChangeSpiritOwnerWithoutBattleFacts,
}

impl ScriptBackendCombatRuntimeErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BattleFacts => "battle facts",
            Self::SideResolution => "side resolution",
            Self::InvalidReadyPhasePayload => "invalid ready phase payload",
            Self::MissingStartContext => "missing start context",
            Self::InvalidPhase => "invalid phase",
            Self::MissingStartSummary => "missing start summary",
            Self::MissingSideRegistry => "missing side registry",
            Self::MissingHistoryRecorder => "missing history recorder",
            Self::MissingBattleFactsForHistorySnapshot => {
                "missing battle facts for history snapshot"
            }
            Self::MissingObservedInitialStateForRoundHistory => {
                "missing observed initial state for round history"
            }
            Self::MissingRoundBarrierForPresentation => "missing round barrier for presentation",
            Self::HistoryRecorder => "history recorder",
            Self::HistoryProjection => "history projection",
            Self::ChangeSpiritOwnerWithoutBattleFacts => "change spirit owner without battle facts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatWaitError {
    TimeoutWaitingCombatAction {
        phase: ScriptCombatPhase,
        elapsed_ms: u128,
    },
}

impl fmt::Display for ScriptCombatWaitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeoutWaitingCombatAction { phase, elapsed_ms } => write!(
                f,
                "timeout waiting for combat action, phase {}, elapsed_ms={elapsed_ms}",
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
    SpiritBook,
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
            Self::SpiritBook => "spirit book",
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
    MysteryFusionMaterialCountExceedsLimit {
        count: usize,
        limit: usize,
    },
    SummonDrawCountInvalid {
        count: i64,
    },
    InvalidOption {
        activity: String,
        field: String,
        value: u32,
    },
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
            Self::InvalidOption {
                activity,
                field,
                value,
            } => write!(f, "{activity} invalid {field}: {value}"),
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
pub enum ScriptRequestError {
    NoRunningScriptForRequest,
    NoRunningScriptForCombatCommand,
    PauseStateUnknown,
    EquipItemPositionMustBeOneBased,
    ClearLineupRequiresBatchSupport,
    PendingReplyRefreshFailed {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    WaitStateRejected {
        context: ScriptWaitContext,
        kind: ScriptRequestSystemFailureKind,
    },
    InvalidCombatIntent {
        kind: ScriptCombatIntentKind,
        spirit_index: u8,
        value: u32,
        message: String,
    },
    InvalidCombatCommand {
        kind: ScriptCombatActionValidationKind,
    },
    InvalidCombatServerType {
        value: u8,
        message: String,
    },
    InvalidCombatType {
        value: u8,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatIntentKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatIntentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use_skill",
            Self::ChangeSpirit => "change_spirit",
            Self::UseItem => "use_item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatIntentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptWaitContext {
    WaitNextCombatAction,
    CombatActionSettlingAfterAck,
    UseItemAction,
    TalentRefresh,
    TalentRefreshResultAfterAck,
}

impl ScriptWaitContext {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaitNextCombatAction => "wait_next_combat_action",
            Self::CombatActionSettlingAfterAck => "combat_action_settling_after_ack",
            Self::UseItemAction => "use_item_action",
            Self::TalentRefresh => "talent_refresh",
            Self::TalentRefreshResultAfterAck => "talent_refresh_result_after_ack",
        }
    }
}

impl fmt::Display for ScriptWaitContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRunningScriptForRequest => {
                f.write_str("script request submitted while no script is running")
            }
            Self::NoRunningScriptForCombatCommand => {
                f.write_str("script combat command submitted while no script is running")
            }
            Self::PauseStateUnknown => f.write_str(
                "pause state is unknown; call game::try_set_pause or game::set_pause first",
            ),
            Self::EquipItemPositionMustBeOneBased => {
                f.write_str("equip_item position must be 1-based")
            }
            Self::ClearLineupRequiresBatchSupport => {
                f.write_str("clear_lineup requires confirmed batch request support")
            }
            Self::PendingReplyRefreshFailed { context, kind } => {
                write!(f, "{context} failed to refresh pending script reply: {kind}")
            }
            Self::WaitStateRejected { context, kind } => {
                write!(f, "{context} failed to set script wait state: {kind}")
            }
            Self::InvalidCombatIntent {
                kind,
                spirit_index,
                value,
                message,
            } => write!(
                f,
                "invalid combat intent kind={kind} spirit_index={spirit_index} value={value}: {message}"
            ),
            Self::InvalidCombatCommand { kind } => {
                write!(f, "invalid combat command: {kind}")
            }
            Self::InvalidCombatServerType { value, message } => {
                write!(f, "invalid combat server_type {value}: {message}")
            }
            Self::InvalidCombatType { value, message } => {
                write!(f, "invalid combat_type {value}: {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatActionValidationKind {
    CannotSubmitAction,
    BattleFactsUnavailable,
    InvalidActivePosition { position: u8 },
    SpiritPositionMismatch { active: u8, request: u8 },
    ActiveSpiritNotFound { position: u8 },
    ActiveSpiritFainted { position: u8 },
    ActionAvailabilityUnavailable,
    ActionUnavailable { action: String },
    SkillUnavailableOrNoPp { skill_id: u32 },
    TargetSpiritAlreadyActive { position: u8 },
    TargetSpiritNotFound { position: u8 },
    TargetSpiritFainted { position: u8 },
    InvalidItemId { item_id: u32 },
}

impl fmt::Display for ScriptCombatActionValidationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotSubmitAction => {
                f.write_str("cannot submit combat action in current combat runtime phase")
            }
            Self::BattleFactsUnavailable => f.write_str("combat battle facts unavailable"),
            Self::InvalidActivePosition { position } => {
                write!(f, "invalid active spirit position: {position}")
            }
            Self::SpiritPositionMismatch { active, request } => write!(
                f,
                "combat action spirit_position mismatch: active={active}, request={request}"
            ),
            Self::ActiveSpiritNotFound { position } => {
                write!(f, "active spirit not found: {position}")
            }
            Self::ActiveSpiritFainted { position } => {
                write!(f, "active spirit is fainted: {position}")
            }
            Self::ActionAvailabilityUnavailable => {
                f.write_str("combat action availability unavailable")
            }
            Self::ActionUnavailable { action } => {
                write!(f, "cannot {action} in current combat facts")
            }
            Self::SkillUnavailableOrNoPp { skill_id } => {
                write!(f, "skill unavailable or no PP: skill_id={skill_id}")
            }
            Self::TargetSpiritAlreadyActive { position } => {
                write!(f, "target spirit is already active: {position}")
            }
            Self::TargetSpiritNotFound { position } => {
                write!(f, "no spirit at target position: {position}")
            }
            Self::TargetSpiritFainted { position } => {
                write!(f, "target spirit is fainted: {position}")
            }
            Self::InvalidItemId { item_id } => write!(f, "invalid combat item id: {item_id}"),
        }
    }
}

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
        message: String,
    },
    SendDebugCommandFailed {
        message: String,
    },
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
            Self::SendResponseFailed { message } => {
                write!(f, "failed to send response: {message}")
            }
            Self::SendDebugCommandFailed { message } => {
                write!(f, "failed to send debug command: {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptUnsupportedError {
    Function { name: String },
}

impl fmt::Display for ScriptUnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function { name } => write!(f, "{name} unsupported by this runtime"),
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
        expected_action_kind: u8,
        expected_spirit_index: u8,
        actual_action_kind: u8,
        actual_spirit_index: u8,
    },
    SkillPoolIndexExceedsI64 {
        index: usize,
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
            Self::MissingScriptSessionForActionWait => {
                f.write_str("script action wait requested while no script is running")
            }
            Self::StorageSpiritDetailEmpty => f.write_str("storage spirit detail is empty"),
        }
    }
}

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
        message: String,
    },
    InvalidTimestamp {
        value: i64,
        message: String,
    },
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
            Self::Rejected { name, message } => write!(f, "{name} rejected: {message}"),
            Self::InvalidTimestamp { value, message } => {
                write!(f, "invalid timestamp {value}: {message}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetworkError {
    ChannelClosed,
    HttpRequestFailed {
        message: String,
    },
    HttpBridgeRequestFailed {
        code: String,
        message: String,
    },
    NetBridgeRequestFailed {
        code: String,
        message: String,
    },
    NetResponseParseFailed {
        target: String,
        source: RocoNetResponseParseSource,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetResponseParseSource {
    Protocol { message: String },
    UnexpectedCommand { cmd_id: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoErrorInfo {
    pub kind: String,
    pub code: String,
    pub message: String,
}

impl RocoNetworkError {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ChannelClosed => "channel_closed",
            Self::HttpRequestFailed { .. } => "http_request_failed",
            Self::HttpBridgeRequestFailed { .. } => "http_bridge_request_failed",
            Self::NetBridgeRequestFailed { .. } => "net_bridge_request_failed",
            Self::NetResponseParseFailed { .. } => "net_response_parse_failed",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::ChannelClosed => "network.channel_closed",
            Self::HttpRequestFailed { .. } => "network.http_request_failed",
            Self::HttpBridgeRequestFailed { code, .. } => code.as_str(),
            Self::NetBridgeRequestFailed { code, .. } => code.as_str(),
            Self::NetResponseParseFailed { .. } => "network.net_response_parse_failed",
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn info(&self) -> RocoErrorInfo {
        RocoErrorInfo {
            kind: self.kind().to_string(),
            code: self.code().to_string(),
            message: self.message(),
        }
    }
}

impl fmt::Display for RocoNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChannelClosed => f.write_str("Channel closed"),
            Self::HttpRequestFailed { message } => write!(f, "HTTP request failed: {message}"),
            Self::HttpBridgeRequestFailed { code, message } => {
                write!(f, "HTTP bridge request failed [{code}]: {message}")
            }
            Self::NetBridgeRequestFailed { code, message } => {
                write!(f, "Net bridge request failed [{code}]: {message}")
            }
            Self::NetResponseParseFailed { target, source } => {
                write!(f, "failed to parse network response {target}: {source}")
            }
        }
    }
}

impl fmt::Display for RocoNetResponseParseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol { message } => f.write_str(message),
            Self::UnexpectedCommand { cmd_id } => {
                write!(f, "unexpected command cmd_id=0x{cmd_id:x}")
            }
        }
    }
}

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
pub enum RocoServerRejectedError {
    ReturnCode { message: String },
    HttpResponse { message: String },
    Message { message: String },
}

impl fmt::Display for RocoServerRejectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnCode { message } => write!(f, "return code rejected: {message}"),
            Self::HttpResponse { message } => write!(f, "HTTP response rejected: {message}"),
            Self::Message { message } => f.write_str(message),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoGeneralError {
    LockPoisoned { message: String },
    Message { message: String },
}

impl fmt::Display for RocoGeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned { message } => write!(f, "lock poisoned: {message}"),
            Self::Message { message } => f.write_str(message),
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
                write!(f, ": [{}] {}", error.kind, error.message)
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
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ScriptError(_) => "script_error",
            Self::StdLib(_) => "stdlib",
            Self::InvalidParam(_) => "invalid_param",
            Self::NetworkError(error) => error.kind(),
            Self::TimeoutError(_) => "timeout",
            Self::ServerRejected(_) => "server_rejected",
            Self::AssertionError(_) => "assertion",
            Self::Other(_) => "other",
        }
    }

    pub fn code(&self) -> String {
        match self {
            Self::ScriptError(error) => format!("script.{}", error.kind.as_str()),
            Self::StdLib(_) => "stdlib".to_string(),
            Self::InvalidParam(_) => "invalid_param".to_string(),
            Self::NetworkError(error) => error.code().to_string(),
            Self::TimeoutError(_) => "timeout.waiting_for_response".to_string(),
            Self::ServerRejected(_) => "server_rejected".to_string(),
            Self::AssertionError(_) => "assertion".to_string(),
            Self::Other(_) => "other".to_string(),
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn info(&self) -> RocoErrorInfo {
        RocoErrorInfo {
            kind: self.kind().to_string(),
            code: self.code(),
            message: self.message(),
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
