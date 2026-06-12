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
    Result, RocoError, RocoScriptError, RocoScriptErrorKind, RocoScriptLocation,
    RocoScriptPosition, RocoStdLibError, ScriptActivityOperationError, ScriptBridgeError,
    ScriptCombatActionError, ScriptCombatPhase, ScriptCombatRuntimeError, ScriptCombatWaitError,
    ScriptFunctionContextError, ScriptLookupEntity, ScriptLookupError, ScriptPendingResponseError,
    ScriptQueryError, ScriptResponseError, ScriptSessionMemoryError, ScriptSessionValueKind,
    ScriptSpiritOperationError, ScriptStaticDataError, ScriptSystemError,
};
pub use stdlib::{
    RocoActivityStdLib, RocoAdventureActivityStdLib, RocoAlchemyActivityStdLib,
    RocoAquariusActivityStdLib, RocoAriesActivityStdLib, RocoCancerActivityStdLib,
    RocoCombatStdLib, RocoEvolutionActivityStdLib, RocoGeminiActivityStdLib, RocoLeoActivityStdLib,
    RocoLibraActivityStdLib, RocoLookupStdLib, RocoMagicPioneerActivityStdLib,
    RocoManorActivityStdLib, RocoNewsActivityStdLib, RocoPiscesActivityStdLib, RocoRuntimeStdLib,
    RocoSagittariusActivityStdLib, RocoScorpioActivityStdLib, RocoSpiritStdLib, RocoStdLib,
    RocoSystemStdLib, RocoTaurusActivityStdLib, RocoThreeStartersActivityStdLib,
    RocoTowerActivityStdLib, RocoVirgoActivityStdLib, RocoZodiacActivityStdLib,
};
pub use types::*;
