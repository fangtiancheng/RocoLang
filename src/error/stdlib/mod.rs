use super::*;

mod activity;
mod bridge;
mod combat;
mod context;
mod lookup;
mod names;
mod pending;
mod request;
mod session;
mod spirit;
mod system;

pub use activity::*;
pub use bridge::*;
pub use combat::*;
pub use context::*;
pub use lookup::*;
pub use names::*;
pub use pending::*;
pub use request::*;
pub use session::*;
pub use spirit::*;
pub use system::*;

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

impl RocoStdLibError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::FunctionContext(_) => "stdlib.function_context",
            Self::Query(_) => "stdlib.query",
            Self::CombatAction(_) => "stdlib.combat_action",
            Self::CombatRuntime(_) => "stdlib.combat_runtime",
            Self::CombatWait(_) => "stdlib.combat_wait",
            Self::PendingResponse(_) => "stdlib.pending_response",
            Self::Lookup(_) => "stdlib.lookup",
            Self::SessionMemory(_) => "stdlib.session_memory",
            Self::StaticData(_) => "stdlib.static_data",
            Self::SpiritOperation(_) => "stdlib.spirit_operation",
            Self::System(_) => "stdlib.system",
            Self::ActivityOperation(_) => "stdlib.activity_operation",
            Self::Bridge(_) => "stdlib.bridge",
            Self::Response(_) => "stdlib.response",
            Self::Request(_) => "stdlib.request",
            Self::Unsupported(error) => error.code(),
        }
    }
}
