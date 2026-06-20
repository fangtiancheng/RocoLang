//! RocoLang - 基于 Rhai 的洛克王国测试脚本语言
//!
//! 提供统一的脚本执行引擎和标准库接口定义

pub mod builtin_sources;
pub mod debugger;
pub mod engine;
pub mod error;
pub mod rocolib;
pub mod stdlib;
pub mod types;

// 重导出核心类型
pub use debugger::{
    RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent, RocoDebugHooks,
    RocoDebugLocalVariable, RocoDebugStackFrame,
};
pub use engine::RocoEngine;
pub use error::{
    Result, RocoError, RocoGeneralError, RocoInvalidParamError, RocoNetworkError, RocoParamRange,
    RocoScriptError, RocoScriptErrorKind, RocoScriptLocation, RocoScriptPosition,
    RocoServerRejectedError, RocoStdLibError, RocoTimeoutError, ScriptActivityOperationError,
    ScriptBackendCombatRuntimeErrorKind, ScriptBridgeError, ScriptCombatActionError,
    ScriptCombatActionValidationKind, ScriptCombatPhase, ScriptCombatRuntimeError,
    ScriptCombatWaitError, ScriptFunctionContextError, ScriptLookupEntity, ScriptLookupError,
    ScriptPendingResponseError, ScriptQueryError, ScriptRequestError,
    ScriptRequestSystemFailureKind, ScriptResponseError, ScriptSessionMemoryError,
    ScriptSessionValueKind, ScriptSpiritOperationError, ScriptStaticDataError, ScriptSystemError,
    ScriptUnsupportedError,
};
pub use stdlib::{
    documented_stdlib_function_keys, find_stdlib_function_doc, registered_stdlib_function_keys,
    registered_stdlib_function_registrations, stdlib_function_docs, RocoActivityStdLib,
    RocoAdventureActivityStdLib, RocoAlchemyActivityStdLib, RocoAquariusActivityStdLib,
    RocoAriesActivityStdLib, RocoCancerActivityStdLib, RocoCombatStdLib,
    RocoEvolutionActivityStdLib, RocoGeminiActivityStdLib, RocoLeoActivityStdLib,
    RocoLibraActivityStdLib, RocoLookupStdLib, RocoMagicPioneerActivityStdLib,
    RocoManorActivityStdLib, RocoNewsActivityStdLib, RocoPetTrainingActivityStdLib,
    RocoPiscesActivityStdLib, RocoRuntimeStdLib, RocoSagittariusActivityStdLib,
    RocoScorpioActivityStdLib, RocoSpiritBookStdLib, RocoSpiritStdLib, RocoStdLib,
    RocoSystemStdLib, RocoTaurusActivityStdLib, RocoThreeStartersActivityStdLib,
    RocoTowerActivityStdLib, RocoVirgoActivityStdLib, RocoZodiacActivityStdLib, StdlibFieldDoc,
    StdlibFunctionDoc, StdlibFunctionKey, StdlibFunctionRegistration, StdlibParamDoc,
    StdlibReturnDoc,
};
pub use types::*;
