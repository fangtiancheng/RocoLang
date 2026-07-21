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
    Result, RocoBridgeErrorChannel, RocoBridgeErrorInfo, RocoBridgeErrorKind, RocoError,
    RocoErrorDetail, RocoErrorInfo, RocoErrorKind, RocoGeneralError, RocoGeneralLockTarget,
    RocoHttpBridgeErrorKind, RocoHttpBusinessRejection, RocoInvalidParamDetail,
    RocoInvalidParamError, RocoInvalidParamInfo, RocoInvalidParamKind, RocoNetBridgeErrorKind,
    RocoNetResponseParseFailure, RocoNetResponseParseSource, RocoNetResponseParseTarget,
    RocoNetworkError, RocoNetworkErrorKind, RocoParamRange, RocoProtocolFieldName,
    RocoProtocolParseContext, RocoProtocolParseErrorType, RocoProtocolParseFailureKind,
    RocoProtocolParseLayer, RocoProtocolParseReason, RocoReturnCodeKind, RocoReturnCodeRejection,
    RocoScriptError, RocoScriptErrorKind, RocoScriptErrorSource, RocoScriptEvalErrorSource,
    RocoScriptLocation, RocoScriptParseErrorSource, RocoScriptPosition, RocoServerRejectedError,
    RocoSpiritStorageProtoContext, RocoSpiritStorageProtoField, RocoStdLibError, RocoTimeoutError,
    ScriptActiveConfigUnavailableSource, ScriptActivityName, ScriptActivityOperationError,
    ScriptActivityOptionField, ScriptBackendCombatRuntimeErrorKind, ScriptBridgeError,
    ScriptBridgeFailure, ScriptBridgeFailureReason, ScriptBridgeOperation,
    ScriptCombatActionAvailabilityKind, ScriptCombatActionError, ScriptCombatActionValidationKind,
    ScriptCombatCommandFailureKind, ScriptCombatIntentKind, ScriptCombatParticipantDisplayRole,
    ScriptCombatPhase, ScriptCombatProtocolError, ScriptCombatProtocolPropertyStage,
    ScriptCombatRuntimeError, ScriptCombatWaitError, ScriptFunctionContextError,
    ScriptFunctionName, ScriptHttpResponseName, ScriptIntegerType, ScriptLookupEntity,
    ScriptLookupError, ScriptModuleName, ScriptPendingResponseError, ScriptQueryError,
    ScriptRequestError, ScriptRequestSystemFailureKind, ScriptResponseError, ScriptResponseName,
    ScriptSessionMemoryError, ScriptSessionValueKind, ScriptSpiritOperationError,
    ScriptStaticDataError, ScriptSystemError, ScriptSystemFailure, ScriptSystemFailureSource,
    ScriptSystemOperation, ScriptUnsupportedError, ScriptWaitContext,
};
pub use stdlib::{
    documented_stdlib_function_keys, find_stdlib_function_doc, registered_stdlib_function_keys,
    registered_stdlib_function_registrations, stdlib_function_docs, RocoActivityStdLib,
    RocoAdventureActivityStdLib, RocoAlchemyActivityStdLib, RocoAquariusActivityStdLib,
    RocoAriesActivityStdLib, RocoCancerActivityStdLib, RocoCombatStdLib,
    RocoEvolutionActivityStdLib, RocoGeminiActivityStdLib, RocoIncubativeMachineStdLib,
    RocoLeoActivityStdLib, RocoLibraActivityStdLib, RocoLookupStdLib,
    RocoMagicPioneerActivityStdLib, RocoManorActivityStdLib, RocoNewsActivityStdLib,
    RocoPetEggStdLib, RocoPetTrainingActivityStdLib, RocoPiscesActivityStdLib, RocoRuntimeStdLib,
    RocoSagittariusActivityStdLib, RocoScorpioActivityStdLib, RocoSpiritBookStdLib,
    RocoSpiritStdLib, RocoStdLib, RocoSystemStdLib, RocoTaskStdLib, RocoTaurusActivityStdLib,
    RocoThreeStartersActivityStdLib, RocoTowerActivityStdLib, RocoVirgoActivityStdLib,
    RocoZodiacActivityStdLib, StdlibFieldDoc, StdlibFunctionDoc, StdlibFunctionKey,
    StdlibFunctionRegistration, StdlibParamDoc, StdlibReturnDoc,
};
pub use types::*;
