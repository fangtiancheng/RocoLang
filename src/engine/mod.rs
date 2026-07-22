//! RocoEngine - Rhai engine wrapper and stdlib registration

use rhai::{Array, Dynamic, Engine, AST};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::debugger::{
    dynamic_preview, RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent,
    RocoDebugHooks, RocoDebugLocalVariable, RocoDebugStackFrame,
};
use crate::error::{Result, RocoScriptRuntimeErrorValue};
use crate::stdlib::RocoStdLib;
use crate::types::{
    IncubativeMachineActionResult, IncubativeMachineEggInfo, IncubativeMachineEggListResult,
    IncubativeMachineGetSpiritResult, IncubativeMachineIncubationInfo,
    IncubativeMachineIncubationResult, IncubativeMachineInfo, PetEggBeginResult,
    PetEggCancelResult, PetEggInfo, PetEggPreviewResult, PetEggSpeedUpResult, PetEggSpiritInfo,
    RemoteSceneData, RocoRequestContext, RocoRewardKind, TaskProgressResult,
};

include!(concat!(env!("OUT_DIR"), "/roco_type_list.rs"));

type PrintCallback = Arc<Mutex<dyn FnMut(&str) + Send>>;

#[derive(Default)]
struct DebugControlState {
    stop_requested: AtomicBool,
}

pub struct RocoEngine {
    engine: Engine,
    print_callback: Option<PrintCallback>,
}

mod debug;
mod registration;
mod script_error;

impl RocoEngine {
    pub fn new<T: RocoStdLib + 'static>(stdlib: Arc<Mutex<T>>) -> Self {
        let mut engine = Engine::new();
        engine.set_max_expr_depths(0, 0);
        Self::register_stdlib(&mut engine, stdlib);
        Self::register_builtin_helpers(&mut engine);
        crate::rocolib::register(&mut engine);

        Self {
            engine,
            print_callback: None,
        }
    }

    pub fn set_print_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(callback));
        self.print_callback = Some(callback.clone());

        let print_cb = callback.clone();
        self.engine.on_print(move |text| {
            if let Ok(mut cb) = print_cb.lock() {
                cb(text);
            }
        });

        let debug_cb = callback;
        self.engine.on_debug(move |text, source, pos| {
            if let Ok(mut cb) = debug_cb.lock() {
                let msg = if let Some(src) = source {
                    format!("[DEBUG {}:{}] {}", src, pos, text)
                } else {
                    format!("[DEBUG] {}", text)
                };
                cb(&msg);
            }
        });
    }

    pub fn eval(&mut self, script: &str) -> Result<Dynamic> {
        let ast = self
            .engine
            .compile(script)
            .map_err(|error| script_error::map_parse(error, None))?;
        self.engine.eval_ast(&ast).map_err(script_error::map_eval)
    }

    pub fn compile(&self, script: &str) -> Result<AST> {
        self.engine
            .compile(script)
            .map_err(|error| script_error::map_parse(error, None))
    }

    pub fn eval_ast(&mut self, ast: &AST) -> Result<Dynamic> {
        self.engine.eval_ast(ast).map_err(script_error::map_eval)
    }

    pub fn call_fn<T: Clone + 'static>(
        &mut self,
        ast: &AST,
        fn_name: &str,
        args: impl rhai::FuncArgs,
    ) -> Result<T> {
        let mut scope = rhai::Scope::new();
        self.engine
            .call_fn(&mut scope, ast, fn_name, args)
            .map_err(script_error::map_eval)
    }
}

#[cfg(test)]
mod tests {
    use super::RocoEngine;
    use crate::{RocoError, RocoScriptErrorSource};
    use rhai::Engine;

    #[test]
    fn eval_classifies_compile_failures_as_parse_errors() {
        let mut engine = RocoEngine {
            engine: Engine::new(),
            print_callback: None,
        };

        let error = engine.eval("let x = ;").expect_err("script should fail");

        assert!(matches!(
            error,
            RocoError::ScriptError(error)
                if matches!(error.source, RocoScriptErrorSource::Parse(_))
        ));
    }
}
