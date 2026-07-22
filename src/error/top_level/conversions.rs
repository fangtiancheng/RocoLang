use super::*;

impl From<RocoStdLibError> for RocoError {
    fn from(error: RocoStdLibError) -> Self {
        Self::StdLib(error)
    }
}

macro_rules! impl_stdlib_error_conversion {
    ($($source:ty => $variant:ident),+ $(,)?) => {
        $(
            impl From<$source> for RocoStdLibError {
                fn from(error: $source) -> Self {
                    Self::$variant(error)
                }
            }

            impl From<$source> for RocoError {
                fn from(error: $source) -> Self {
                    RocoStdLibError::from(error).into()
                }
            }
        )+
    };
}

impl_stdlib_error_conversion! {
    ScriptFunctionContextError => FunctionContext,
    ScriptQueryError => Query,
    ScriptCombatActionError => CombatAction,
    ScriptCombatRuntimeError => CombatRuntime,
    ScriptCombatWaitError => CombatWait,
    ScriptPendingResponseError => PendingResponse,
    ScriptLookupError => Lookup,
    ScriptSessionMemoryError => SessionMemory,
    ScriptStaticDataError => StaticData,
    ScriptSpiritOperationError => SpiritOperation,
    ScriptSystemError => System,
    ScriptActivityOperationError => ActivityOperation,
    ScriptBridgeError => Bridge,
    ScriptResponseError => Response,
    ScriptRequestError => Request,
    ScriptUnsupportedError => Unsupported,
}
