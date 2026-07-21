//! RocoEngine - Rhai engine wrapper and stdlib registration

use rhai::{Array, Dynamic, Engine, AST};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::debugger::{
    dynamic_preview, RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent,
    RocoDebugHooks, RocoDebugLocalVariable, RocoDebugStackFrame,
};
use crate::error::{
    Result, RocoBridgeErrorChannel, RocoBridgeErrorInfo, RocoBridgeErrorKind, RocoError,
    RocoErrorDetail, RocoErrorInfo, RocoErrorKind, RocoGeneralLockTarget, RocoHttpBridgeErrorKind,
    RocoHttpBusinessRejection, RocoInvalidParamDetail, RocoInvalidParamInfo, RocoInvalidParamKind,
    RocoJsonErrorCategory, RocoNetBridgeErrorKind, RocoNetResponseParseFailure,
    RocoNetResponseParseSource, RocoNetResponseParseTarget, RocoNetworkErrorKind, RocoParamRange,
    RocoProtocolFieldName, RocoProtocolParseContext, RocoProtocolParseErrorType,
    RocoProtocolParseFailureKind, RocoProtocolParseLayer, RocoProtocolParseReason,
    RocoReturnCodeKind, RocoReturnCodeRejection, RocoScriptError, RocoScriptErrorSource,
    RocoScriptEvalErrorSource, RocoScriptLocation, RocoScriptParseErrorSource, RocoScriptPosition,
    RocoScriptRuntimeErrorValue, RocoSpiritStorageProtoContext, RocoSpiritStorageProtoField,
    ScriptActivityName, ScriptActivityOperationError, ScriptActivityOptionField,
    ScriptBackendCombatRuntimeErrorKind, ScriptBridgeFailure, ScriptBridgeFailureReason,
    ScriptBridgeOperation, ScriptCombatActionError, ScriptCombatCommandFailureKind,
    ScriptCombatPhase, ScriptCombatRuntimeError, ScriptCombatWaitError, ScriptFunctionContextError,
    ScriptFunctionName, ScriptHttpResponseName, ScriptModuleName, ScriptQueryError,
    ScriptRequestError, ScriptResponseName, ScriptSpiritOperationError, ScriptStaticDataError,
    ScriptSystemFailure, ScriptSystemFailureSource, ScriptSystemOperation,
};
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

impl RocoEngine {
    fn apply_debug_breakpoints(
        debugger: &mut rhai::debugger::Debugger,
        default_source: Option<&str>,
        breakpoints: &[RocoDebugBreakpoint],
    ) {
        debugger.break_points_mut().clear();
        for breakpoint in breakpoints {
            if !breakpoint.enabled || breakpoint.line == 0 {
                continue;
            }

            let line = breakpoint.line.min(u16::MAX as usize) as u16;
            let column = breakpoint.column.unwrap_or(0).min(u16::MAX as usize) as u16;
            let source = breakpoint
                .source
                .clone()
                .or_else(|| default_source.map(ToString::to_string))
                .map(Into::into);
            debugger
                .break_points_mut()
                .push(rhai::debugger::BreakPoint::AtPosition {
                    source,
                    pos: rhai::Position::new(line, column),
                    enabled: true,
                });
        }
    }

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

    fn register_stdlib<T: RocoStdLib + 'static>(engine: &mut Engine, stdlib: Arc<Mutex<T>>) {
        Self::register_static_info_types(engine);
        crate::stdlib::register_modules(engine, stdlib);
    }
    pub fn eval(&mut self, script: &str) -> Result<Dynamic> {
        let ast = self
            .engine
            .compile(script)
            .map_err(|error| Self::map_parse_error(error, None))?;
        self.engine.eval_ast(&ast).map_err(Self::map_eval_error)
    }

    pub fn compile(&self, script: &str) -> Result<AST> {
        self.engine
            .compile(script)
            .map_err(|error| Self::map_parse_error(error, None))
    }

    pub fn eval_ast(&mut self, ast: &AST) -> Result<Dynamic> {
        self.engine.eval_ast(ast).map_err(Self::map_eval_error)
    }

    fn script_position(position: rhai::Position) -> Option<RocoScriptPosition> {
        position.line().map(|line| match position.position() {
            Some(column) => RocoScriptPosition::LineColumn { line, column },
            None => RocoScriptPosition::Line { line },
        })
    }

    fn script_location(source: Option<String>, position: rhai::Position) -> RocoScriptLocation {
        let Some(position) = Self::script_position(position) else {
            return RocoScriptLocation::Unknown;
        };
        match source {
            Some(source) if !source.is_empty() => RocoScriptLocation::Source { source, position },
            _ => RocoScriptLocation::Anonymous { position },
        }
    }

    fn script_parse_error_source(error: &rhai::ParseErrorType) -> RocoScriptParseErrorSource {
        match error {
            rhai::ParseErrorType::UnexpectedEOF => RocoScriptParseErrorSource::UnexpectedEof,
            rhai::ParseErrorType::BadInput(_) => RocoScriptParseErrorSource::BadInput,
            rhai::ParseErrorType::UnknownOperator(operator) => {
                RocoScriptParseErrorSource::UnknownOperator {
                    operator: operator.clone(),
                }
            }
            rhai::ParseErrorType::MissingToken(token, description) => {
                RocoScriptParseErrorSource::MissingToken {
                    token: token.clone(),
                    description: description.clone(),
                }
            }
            rhai::ParseErrorType::MissingSymbol(description) => {
                RocoScriptParseErrorSource::MissingSymbol {
                    description: description.clone(),
                }
            }
            #[allow(deprecated)]
            rhai::ParseErrorType::MalformedCallExpr(description) => {
                RocoScriptParseErrorSource::DeprecatedMalformedCallExpr {
                    description: description.clone(),
                }
            }
            rhai::ParseErrorType::MalformedIndexExpr(description) => {
                RocoScriptParseErrorSource::MalformedIndexExpr {
                    description: description.clone(),
                }
            }
            #[allow(deprecated)]
            rhai::ParseErrorType::MalformedInExpr(description) => {
                RocoScriptParseErrorSource::DeprecatedMalformedInExpr {
                    description: description.clone(),
                }
            }
            rhai::ParseErrorType::MalformedCapture(description) => {
                RocoScriptParseErrorSource::MalformedCapture {
                    description: description.clone(),
                }
            }
            rhai::ParseErrorType::DuplicatedProperty(property) => {
                RocoScriptParseErrorSource::DuplicatedProperty {
                    property: property.clone(),
                }
            }
            #[allow(deprecated)]
            rhai::ParseErrorType::DuplicatedSwitchCase => {
                RocoScriptParseErrorSource::DeprecatedDuplicatedSwitchCase
            }
            rhai::ParseErrorType::DuplicatedVariable(variable) => {
                RocoScriptParseErrorSource::DuplicatedVariable {
                    variable: variable.clone(),
                }
            }
            rhai::ParseErrorType::WrongSwitchIntegerCase => {
                RocoScriptParseErrorSource::WrongSwitchIntegerCase
            }
            rhai::ParseErrorType::WrongSwitchDefaultCase => {
                RocoScriptParseErrorSource::WrongSwitchDefaultCase
            }
            rhai::ParseErrorType::WrongSwitchCaseCondition => {
                RocoScriptParseErrorSource::WrongSwitchCaseCondition
            }
            rhai::ParseErrorType::PropertyExpected => RocoScriptParseErrorSource::PropertyExpected,
            rhai::ParseErrorType::VariableExpected => RocoScriptParseErrorSource::VariableExpected,
            rhai::ParseErrorType::ForbiddenVariable(variable) => {
                RocoScriptParseErrorSource::ForbiddenVariable {
                    variable: variable.clone(),
                }
            }
            rhai::ParseErrorType::Reserved(symbol) => RocoScriptParseErrorSource::Reserved {
                symbol: symbol.clone(),
            },
            rhai::ParseErrorType::MismatchedType(expected, actual) => {
                RocoScriptParseErrorSource::MismatchedType {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }
            }
            rhai::ParseErrorType::ExprExpected(expression) => {
                RocoScriptParseErrorSource::ExprExpected {
                    expression: expression.clone(),
                }
            }
            rhai::ParseErrorType::WrongDocComment => RocoScriptParseErrorSource::WrongDocComment,
            rhai::ParseErrorType::WrongFnDefinition => {
                RocoScriptParseErrorSource::WrongFunctionDefinition
            }
            rhai::ParseErrorType::FnDuplicatedDefinition(function, params) => {
                RocoScriptParseErrorSource::FunctionDuplicatedDefinition {
                    function: function.clone(),
                    params: *params,
                }
            }
            rhai::ParseErrorType::FnMissingName => RocoScriptParseErrorSource::FunctionMissingName,
            rhai::ParseErrorType::FnMissingParams(function) => {
                RocoScriptParseErrorSource::FunctionMissingParams {
                    function: function.clone(),
                }
            }
            rhai::ParseErrorType::FnDuplicatedParam(function, parameter) => {
                RocoScriptParseErrorSource::FunctionDuplicatedParam {
                    function: function.clone(),
                    parameter: parameter.clone(),
                }
            }
            rhai::ParseErrorType::FnMissingBody(function) => {
                RocoScriptParseErrorSource::FunctionMissingBody {
                    function: function.clone(),
                }
            }
            rhai::ParseErrorType::WrongExport => RocoScriptParseErrorSource::WrongExport,
            rhai::ParseErrorType::AssignmentToConstant(variable) => {
                RocoScriptParseErrorSource::AssignmentToConstant {
                    variable: variable.clone(),
                }
            }
            rhai::ParseErrorType::AssignmentToInvalidLHS(description) => {
                RocoScriptParseErrorSource::AssignmentToInvalidLhs {
                    description: description.clone(),
                }
            }
            rhai::ParseErrorType::VariableExists(variable) => {
                RocoScriptParseErrorSource::VariableExists {
                    variable: variable.clone(),
                }
            }
            rhai::ParseErrorType::VariableUndefined(variable) => {
                RocoScriptParseErrorSource::VariableUndefined {
                    variable: variable.clone(),
                }
            }
            rhai::ParseErrorType::ModuleUndefined(module) => {
                RocoScriptParseErrorSource::ModuleUndefined {
                    module: module.clone(),
                }
            }
            rhai::ParseErrorType::ExprTooDeep => RocoScriptParseErrorSource::ExprTooDeep,
            rhai::ParseErrorType::TooManyFunctions => RocoScriptParseErrorSource::TooManyFunctions,
            rhai::ParseErrorType::LiteralTooLarge(type_name, max_size) => {
                RocoScriptParseErrorSource::LiteralTooLarge {
                    type_name: type_name.clone(),
                    max_size: *max_size,
                }
            }
            rhai::ParseErrorType::LoopBreak => RocoScriptParseErrorSource::LoopBreak,
            _ => RocoScriptParseErrorSource::UnclassifiedParse,
        }
    }

    fn script_eval_error_source(error: &rhai::EvalAltResult) -> RocoScriptEvalErrorSource {
        match error {
            rhai::EvalAltResult::ErrorSystem(message, _) => RocoScriptEvalErrorSource::System {
                message: message.clone(),
            },
            rhai::EvalAltResult::ErrorParsing(source, _) => RocoScriptEvalErrorSource::Parsing {
                source: Self::script_parse_error_source(source),
            },
            rhai::EvalAltResult::ErrorVariableExists(variable, _) => {
                RocoScriptEvalErrorSource::VariableExists {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorForbiddenVariable(variable, _) => {
                RocoScriptEvalErrorSource::ForbiddenVariable {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorVariableNotFound(variable, _) => {
                RocoScriptEvalErrorSource::VariableNotFound {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorPropertyNotFound(property, _) => {
                RocoScriptEvalErrorSource::PropertyNotFound {
                    property: property.clone(),
                }
            }
            rhai::EvalAltResult::ErrorIndexNotFound(_, _) => {
                RocoScriptEvalErrorSource::IndexNotFound
            }
            rhai::EvalAltResult::ErrorFunctionNotFound(signature, _) => {
                RocoScriptEvalErrorSource::FunctionNotFound {
                    signature: signature.clone(),
                }
            }
            rhai::EvalAltResult::ErrorModuleNotFound(module, _) => {
                RocoScriptEvalErrorSource::ModuleNotFound {
                    module: module.clone(),
                }
            }
            rhai::EvalAltResult::ErrorInFunctionCall(function, source, cause, _) => {
                RocoScriptEvalErrorSource::FunctionCall {
                    function: function.clone(),
                    source: source.clone(),
                    cause: Box::new(Self::script_eval_error_source(cause)),
                }
            }
            rhai::EvalAltResult::ErrorInModule(module, cause, _) => {
                RocoScriptEvalErrorSource::Module {
                    module: module.clone(),
                    cause: Box::new(Self::script_eval_error_source(cause)),
                }
            }
            rhai::EvalAltResult::ErrorUnboundThis(_) => RocoScriptEvalErrorSource::UnboundThis,
            rhai::EvalAltResult::ErrorMismatchDataType(expected, actual, _) => {
                RocoScriptEvalErrorSource::MismatchDataType {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }
            }
            rhai::EvalAltResult::ErrorMismatchOutputType(expected, actual, _) => {
                RocoScriptEvalErrorSource::MismatchOutputType {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }
            }
            rhai::EvalAltResult::ErrorIndexingType(type_name, _) => {
                RocoScriptEvalErrorSource::IndexingType {
                    type_name: type_name.clone(),
                }
            }
            rhai::EvalAltResult::ErrorArrayBounds(len, index, _) => {
                RocoScriptEvalErrorSource::ArrayBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorStringBounds(len, index, _) => {
                RocoScriptEvalErrorSource::StringBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorBitFieldBounds(len, index, _) => {
                RocoScriptEvalErrorSource::BitFieldBounds {
                    len: *len,
                    index: *index,
                }
            }
            rhai::EvalAltResult::ErrorFor(_) => RocoScriptEvalErrorSource::ForLoopNonIterable,
            rhai::EvalAltResult::ErrorDataRace(variable, _) => {
                RocoScriptEvalErrorSource::DataRace {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorNonPureMethodCallOnConstant(method, _) => {
                RocoScriptEvalErrorSource::NonPureMethodCallOnConstant {
                    method: method.clone(),
                }
            }
            rhai::EvalAltResult::ErrorAssignmentToConstant(variable, _) => {
                RocoScriptEvalErrorSource::AssignmentToConstant {
                    variable: variable.clone(),
                }
            }
            rhai::EvalAltResult::ErrorDotExpr(expression, _) => {
                RocoScriptEvalErrorSource::DotExpr {
                    expression: expression.clone(),
                }
            }
            rhai::EvalAltResult::ErrorArithmetic(message, _) => {
                RocoScriptEvalErrorSource::Arithmetic {
                    message: message.clone(),
                }
            }
            rhai::EvalAltResult::ErrorTooManyOperations(_) => {
                RocoScriptEvalErrorSource::TooManyOperations
            }
            rhai::EvalAltResult::ErrorTooManyVariables(_) => {
                RocoScriptEvalErrorSource::TooManyVariables
            }
            rhai::EvalAltResult::ErrorTooManyModules(_) => {
                RocoScriptEvalErrorSource::TooManyModules
            }
            rhai::EvalAltResult::ErrorStackOverflow(_) => RocoScriptEvalErrorSource::StackOverflow,
            rhai::EvalAltResult::ErrorDataTooLarge(type_name, _) => {
                RocoScriptEvalErrorSource::DataTooLarge {
                    type_name: type_name.clone(),
                }
            }
            rhai::EvalAltResult::ErrorTerminated(_, _) => RocoScriptEvalErrorSource::Terminated,
            rhai::EvalAltResult::ErrorCustomSyntax(message, symbols, _) => {
                RocoScriptEvalErrorSource::CustomSyntax {
                    message: message.clone(),
                    symbols: symbols.clone(),
                }
            }
            rhai::EvalAltResult::ErrorRuntime(value, _) => RocoScriptEvalErrorSource::Runtime {
                value: Self::script_runtime_error_value(value),
            },
            rhai::EvalAltResult::LoopBreak(is_break, _, _) => {
                RocoScriptEvalErrorSource::LoopBreak {
                    is_break: *is_break,
                }
            }
            rhai::EvalAltResult::Return(_, _) => RocoScriptEvalErrorSource::Return,
            rhai::EvalAltResult::Exit(_, _) => RocoScriptEvalErrorSource::Exit,
            _ => RocoScriptEvalErrorSource::UnclassifiedEval,
        }
    }

    fn script_runtime_error_value(value: &Dynamic) -> RocoScriptRuntimeErrorValue {
        value
            .clone()
            .try_cast::<RocoScriptRuntimeErrorValue>()
            .unwrap_or_else(|| RocoScriptRuntimeErrorValue::message(value.to_string()))
    }

    fn map_eval_error(error: Box<rhai::EvalAltResult>) -> RocoError {
        let position = error.position();
        let location_source = match error.as_ref() {
            rhai::EvalAltResult::ErrorInFunctionCall(_, source, ..) if !source.is_empty() => {
                Some(source.clone())
            }
            _ => None,
        };
        let error_source = RocoScriptErrorSource::Eval(Self::script_eval_error_source(&error));
        RocoError::ScriptError(RocoScriptError {
            location: Self::script_location(location_source, position),
            source: error_source,
        })
    }

    #[allow(deprecated)]
    pub fn eval_debug(
        &mut self,
        script: &str,
        config: RocoDebugConfig,
        hooks: RocoDebugHooks,
    ) -> Result<Dynamic> {
        let control_state = Arc::new(DebugControlState::default());
        let hooks = Arc::new(Mutex::new(hooks));
        let init_config = config.clone();
        let event_hooks = hooks.clone();
        let progress_control = control_state.clone();
        let debugger_control = control_state.clone();

        self.engine.on_progress(move |_| {
            if progress_control.stop_requested.load(Ordering::Relaxed) {
                Some("debug session stopped".into())
            } else {
                None
            }
        });

        self.engine.register_debugger(
            move |_, mut debugger| {
                Self::apply_debug_breakpoints(
                    &mut debugger,
                    init_config.source.as_deref(),
                    &init_config.breakpoints,
                );
                debugger
            },
            move |mut context, event, _node, source, pos| {
                let mut hooks = event_hooks.lock().map_err(|err| {
                    Box::<rhai::EvalAltResult>::from(rhai::EvalAltResult::ErrorRuntime(
                        format!("Debugger hook lock error: {err}").into(),
                        pos,
                    ))
                })?;

                match event {
                    rhai::debugger::DebuggerEvent::Start => {
                        (hooks.on_event)(RocoDebugEvent::Started);
                        Ok(rhai::debugger::DebuggerCommand::Continue)
                    }
                    rhai::debugger::DebuggerEvent::End => {
                        (hooks.on_event)(RocoDebugEvent::Ended);
                        Ok(rhai::debugger::DebuggerCommand::Continue)
                    }
                    event => {
                        let reason = match event {
                            rhai::debugger::DebuggerEvent::Step => "step".to_string(),
                            rhai::debugger::DebuggerEvent::BreakPoint(index) => {
                                format!("breakpoint:{index}")
                            }
                            rhai::debugger::DebuggerEvent::FunctionExitWithValue(_) => {
                                "function_exit".to_string()
                            }
                            rhai::debugger::DebuggerEvent::FunctionExitWithError(_) => {
                                "function_exit_error".to_string()
                            }
                            _ => "debugger_event".to_string(),
                        };
                        let stack = context
                            .global_runtime_state()
                            .debugger()
                            .call_stack()
                            .iter()
                            .map(|frame| RocoDebugStackFrame {
                                function_name: frame.fn_name.to_string(),
                                location: Self::script_location(
                                    frame.source.as_ref().map(ToString::to_string),
                                    frame.pos,
                                ),
                                args_preview: frame.args.iter().map(dynamic_preview).collect(),
                            })
                            .collect();
                        let visible_scope = context.scope().clone_visible();
                        let locals = visible_scope
                            .iter_raw()
                            .map(|(name, is_constant, value)| RocoDebugLocalVariable {
                                name: name.to_string(),
                                type_name: context.engine().map_type_name(value.type_name()).into(),
                                value_preview: dynamic_preview(value),
                                is_constant,
                            })
                            .collect();

                        (hooks.on_event)(RocoDebugEvent::Paused {
                            reason,
                            location: Self::script_location(source.map(ToString::to_string), pos),
                            stack,
                            locals,
                        });

                        loop {
                            let command = (hooks.wait_command)();
                            match command {
                                RocoDebugCommand::SetBreakpoints(breakpoints) => {
                                    Self::apply_debug_breakpoints(
                                        context.global_runtime_state_mut().debugger_mut(),
                                        source,
                                        &breakpoints,
                                    );
                                }
                                RocoDebugCommand::Continue => {
                                    (hooks.on_event)(RocoDebugEvent::Continued);
                                    return Ok(rhai::debugger::DebuggerCommand::Continue);
                                }
                                RocoDebugCommand::StepInto => {
                                    (hooks.on_event)(RocoDebugEvent::Continued);
                                    return Ok(rhai::debugger::DebuggerCommand::StepInto);
                                }
                                RocoDebugCommand::StepOver => {
                                    (hooks.on_event)(RocoDebugEvent::Continued);
                                    return Ok(rhai::debugger::DebuggerCommand::StepOver);
                                }
                                RocoDebugCommand::Next => {
                                    (hooks.on_event)(RocoDebugEvent::Continued);
                                    return Ok(rhai::debugger::DebuggerCommand::Next);
                                }
                                RocoDebugCommand::StepOut => {
                                    (hooks.on_event)(RocoDebugEvent::Continued);
                                    return Ok(rhai::debugger::DebuggerCommand::FunctionExit);
                                }
                                RocoDebugCommand::Stop => {
                                    debugger_control
                                        .stop_requested
                                        .store(true, Ordering::Relaxed);
                                    return Err(Box::<rhai::EvalAltResult>::from(
                                        rhai::EvalAltResult::ErrorTerminated(
                                            "debug session stopped".into(),
                                            pos,
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                }
            },
        );

        let mut ast = self
            .engine
            .compile(script)
            .map_err(|error| Self::map_parse_error(error, config.source.clone()))?;
        if let Some(source) = config.source {
            ast.set_source(source);
        }

        self.engine.eval_ast(&ast).map_err(Self::map_eval_error)
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
            .map_err(Self::map_eval_error)
    }

    fn map_parse_error(error: rhai::ParseError, source: Option<String>) -> RocoError {
        let position = error.position();
        let error_source =
            RocoScriptErrorSource::Parse(Self::script_parse_error_source(error.err_type()));
        RocoError::ScriptError(RocoScriptError {
            location: Self::script_location(source, position),
            source: error_source,
        })
    }

    fn register_builtin_helpers(engine: &mut Engine) {
        engine.register_fn("len", |array: &mut Array| array.len() as i64);
    }

    fn to_array<T: Clone + Send + Sync + 'static>(items: &[T]) -> Array {
        items.iter().cloned().map(Dynamic::from).collect()
    }

    fn register_static_info_types(engine: &mut Engine) {
        macro_rules! register_getters {
            ($type:ty, $($field:ident),+ $(,)?) => {
                $(
                    engine.register_get(stringify!($field), |value: &mut $type| {
                        value.$field.clone()
                    });
                )+
            };
        }
        macro_rules! register_roco_type_basics {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                engine.register_fn(
                    "to_string",
                    |value: &mut $type| -> std::result::Result<String, Box<rhai::EvalAltResult>> {
                        serde_json::to_string_pretty(value).map_err(|error| {
                            Box::<rhai::EvalAltResult>::from(rhai::EvalAltResult::ErrorRuntime(
                                Dynamic::from(
                                    RocoScriptRuntimeErrorValue::roco_type_json_serialize::<$type>(
                                        &error,
                                    ),
                                ),
                                rhai::Position::NONE,
                            ))
                        })
                    },
                );
            };
        }
        for_each_roco_type!(register_roco_type_basics);
        crate::types::register_rhai_getters(engine);
        engine.register_get("raw", |value: &mut RocoRequestContext| value.raw.clone());
        engine.register_get("domain", |value: &mut RocoRequestContext| {
            value.domain().to_string()
        });
        engine.register_get("action", |value: &mut RocoRequestContext| {
            value.action().to_string()
        });
        engine.register_get("code", |value: &mut RocoRewardKind| {
            value.code().to_string()
        });
        register_getters!(
            IncubativeMachineEggInfo,
            egg_id,
            incubate_time,
            property,
            catch_time,
            egg_uin,
            egg_name,
            roco_name
        );
        register_getters!(
            IncubativeMachineIncubationInfo,
            egg_type,
            spirit_id,
            egg_id,
            percentile,
            remainder_time,
            stage,
            egg_name,
            property
        );
        register_getters!(
            IncubativeMachineEggListResult,
            result_code,
            message,
            egg_type,
            total_pages,
            current_page
        );
        engine.register_get("eggs", |value: &mut IncubativeMachineEggListResult| {
            Self::to_array(&value.eggs)
        });
        register_getters!(
            IncubativeMachineInfo,
            result_code,
            message,
            guide,
            incubator_type,
            total_pages,
            current_page,
            incubation
        );
        engine.register_get("eggs", |value: &mut IncubativeMachineInfo| {
            Self::to_array(&value.eggs)
        });
        register_getters!(
            IncubativeMachineIncubationResult,
            result_code,
            message,
            incubation
        );
        register_getters!(IncubativeMachineActionResult, result_code, message);
        register_getters!(
            IncubativeMachineGetSpiritResult,
            result_code,
            message,
            spirit_id,
            spirit_level
        );
        register_getters!(
            PetEggSpiritInfo,
            spirit_id,
            level,
            exp_to_next_level,
            personality,
            hp,
            max_hp,
            caught_time,
            caught_location,
            storage_time
        );
        engine.register_get("skills", |value: &mut PetEggSpiritInfo| {
            Self::to_array(&value.skills)
        });
        register_getters!(
            PetEggInfo,
            result_code,
            message,
            current_egg_count,
            max_egg_count,
            vip_count,
            male,
            female,
            egg
        );
        register_getters!(
            PetEggSpeedUpResult,
            result_code,
            message,
            current_egg_count,
            max_egg_count,
            vip_count
        );
        register_getters!(PetEggBeginResult, result_code, message, max_egg_count);
        register_getters!(PetEggCancelResult, result_code, message, detail_code);
        register_getters!(PetEggPreviewResult, result_code, message, egg);
        register_getters!(RemoteSceneData, scene_id, values);
        register_getters!(TaskProgressResult, result_code, message);
        engine.register_type_with_name::<RocoScriptErrorSource>("RocoScriptErrorSource");
        engine.register_type_with_name::<RocoScriptParseErrorSource>("RocoScriptParseErrorSource");
        engine.register_type_with_name::<RocoScriptEvalErrorSource>("RocoScriptEvalErrorSource");
        engine
            .register_type_with_name::<RocoScriptRuntimeErrorValue>("RocoScriptRuntimeErrorValue");
        engine.register_type_with_name::<RocoJsonErrorCategory>("RocoJsonErrorCategory");
        engine.register_get("code", |value: &mut RocoScriptErrorSource| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut RocoScriptParseErrorSource| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut RocoScriptEvalErrorSource| {
            value.code().to_string()
        });
        engine.register_get(
            "runtime_value",
            |value: &mut RocoScriptEvalErrorSource| match value {
                RocoScriptEvalErrorSource::Runtime { value } => Some(value.clone()),
                _ => None,
            },
        );
        engine.register_get("code", |value: &mut RocoScriptRuntimeErrorValue| {
            value.code().to_string()
        });
        engine.register_get("message", |value: &mut RocoScriptRuntimeErrorValue| {
            value.message_text()
        });
        engine.register_get(
            "roco_type_name",
            |value: &mut RocoScriptRuntimeErrorValue| value.roco_type_name(),
        );
        engine.register_get(
            "json_error_category_code",
            |value: &mut RocoScriptRuntimeErrorValue| value.json_error_category_code(),
        );
        engine.register_get("code", |value: &mut RocoJsonErrorCategory| {
            value.code().to_string()
        });
        engine.register_type_with_name::<RocoErrorInfo>("RocoErrorInfo");
        engine.register_type_with_name::<RocoErrorDetail>("RocoErrorDetail");
        engine.register_type_with_name::<RocoErrorKind>("RocoErrorKind");
        engine.register_type_with_name::<RocoGeneralLockTarget>("RocoGeneralLockTarget");
        engine.register_type_with_name::<RocoInvalidParamInfo>("RocoInvalidParamInfo");
        engine.register_type_with_name::<RocoInvalidParamDetail>("RocoInvalidParamDetail");
        engine.register_type_with_name::<RocoInvalidParamKind>("RocoInvalidParamKind");
        engine.register_type_with_name::<RocoProtocolFieldName>("RocoProtocolFieldName");
        engine.register_type_with_name::<RocoParamRange>("RocoParamRange");
        engine.register_type_with_name::<RocoNetworkErrorKind>("RocoNetworkErrorKind");
        engine.register_type_with_name::<RocoBridgeErrorChannel>("RocoBridgeErrorChannel");
        engine.register_type_with_name::<RocoBridgeErrorInfo>("RocoBridgeErrorInfo");
        engine
            .register_type_with_name::<RocoNetResponseParseFailure>("RocoNetResponseParseFailure");
        engine.register_type_with_name::<RocoNetResponseParseSource>("RocoNetResponseParseSource");
        engine.register_type_with_name::<RocoNetResponseParseTarget>("RocoNetResponseParseTarget");
        engine.register_type_with_name::<RocoProtocolParseReason>("RocoProtocolParseReason");
        engine.register_type_with_name::<RocoProtocolParseContext>("RocoProtocolParseContext");
        engine.register_type_with_name::<RocoSpiritStorageProtoContext>(
            "RocoSpiritStorageProtoContext",
        );
        engine
            .register_type_with_name::<RocoSpiritStorageProtoField>("RocoSpiritStorageProtoField");
        engine.register_type_with_name::<RocoProtocolParseFailureKind>(
            "RocoProtocolParseFailureKind",
        );
        engine.register_type_with_name::<RocoProtocolParseErrorType>("RocoProtocolParseErrorType");
        engine.register_type_with_name::<RocoProtocolParseLayer>("RocoProtocolParseLayer");
        engine.register_type_with_name::<ScriptBridgeOperation>("ScriptBridgeOperation");
        engine.register_type_with_name::<ScriptBridgeFailure>("ScriptBridgeFailure");
        engine.register_type_with_name::<ScriptSystemOperation>("ScriptSystemOperation");
        engine.register_type_with_name::<ScriptSystemFailure>("ScriptSystemFailure");
        engine.register_type_with_name::<ScriptSystemFailureSource>("ScriptSystemFailureSource");
        engine.register_type_with_name::<ScriptCombatCommandFailureKind>(
            "ScriptCombatCommandFailureKind",
        );
        engine.register_type_with_name::<ScriptBackendCombatRuntimeErrorKind>(
            "ScriptBackendCombatRuntimeErrorKind",
        );
        engine.register_type_with_name::<ScriptCombatPhase>("ScriptCombatPhase");
        engine.register_type_with_name::<ScriptFunctionContextError>("ScriptFunctionContextError");
        engine.register_type_with_name::<ScriptQueryError>("ScriptQueryError");
        engine.register_get("code", |value: &mut ScriptCombatPhase| {
            value.code().to_string()
        });
        engine.register_get("kind_code", |value: &mut ScriptFunctionContextError| {
            value.kind_code()
        });
        engine.register_get(
            "combat_phase_code",
            |value: &mut ScriptFunctionContextError| value.combat_phase_code(),
        );
        engine.register_get("kind_code", |value: &mut ScriptQueryError| {
            value.kind_code()
        });
        engine.register_get("skill_index", |value: &mut ScriptQueryError| {
            value.skill_index()
        });
        engine.register_type_with_name::<ScriptStaticDataError>("ScriptStaticDataError");
        engine.register_get("kind_code", |value: &mut ScriptStaticDataError| {
            value.kind_code()
        });
        engine.register_get("position", |value: &mut ScriptStaticDataError| {
            value.position()
        });
        engine.register_get("function_name", |value: &mut ScriptStaticDataError| {
            value.function_name()
        });
        engine.register_get("message", |value: &mut ScriptStaticDataError| {
            value.message()
        });
        engine.register_type_with_name::<ScriptActivityName>("ScriptActivityName");
        engine.register_type_with_name::<ScriptActivityOptionField>("ScriptActivityOptionField");
        engine.register_type_with_name::<ScriptActivityOperationError>(
            "ScriptActivityOperationError",
        );
        engine.register_get("code", |value: &mut ScriptActivityName| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut ScriptActivityOptionField| {
            value.code().to_string()
        });
        engine.register_get("kind_code", |value: &mut ScriptActivityOperationError| {
            value.kind_code()
        });
        engine.register_get(
            "activity_code",
            |value: &mut ScriptActivityOperationError| value.activity_code(),
        );
        engine.register_get("field_code", |value: &mut ScriptActivityOperationError| {
            value.field_code()
        });
        engine.register_get("count", |value: &mut ScriptActivityOperationError| {
            value.count()
        });
        engine.register_get("limit", |value: &mut ScriptActivityOperationError| {
            value.limit()
        });
        engine.register_get("value", |value: &mut ScriptActivityOperationError| {
            value.value()
        });
        engine.register_type_with_name::<ScriptCombatActionError>("ScriptCombatActionError");
        engine.register_get("kind_code", |value: &mut ScriptCombatActionError| {
            value.kind_code()
        });
        engine.register_get("position", |value: &mut ScriptCombatActionError| {
            value.position()
        });
        engine.register_get("skill_id", |value: &mut ScriptCombatActionError| {
            value.skill_id()
        });
        engine.register_get("item_id", |value: &mut ScriptCombatActionError| {
            value.item_id()
        });
        engine.register_type_with_name::<ScriptCombatWaitError>("ScriptCombatWaitError");
        engine.register_get("kind_code", |value: &mut ScriptCombatWaitError| {
            value.kind_code()
        });
        engine.register_get("combat_phase_code", |value: &mut ScriptCombatWaitError| {
            value.combat_phase_code()
        });
        engine.register_get("elapsed_ms", |value: &mut ScriptCombatWaitError| {
            value.elapsed_ms()
        });
        engine.register_type_with_name::<ScriptRequestError>("ScriptRequestError");
        engine.register_get("kind_code", |value: &mut ScriptRequestError| {
            value.kind_code()
        });
        engine.register_get("wait_context_code", |value: &mut ScriptRequestError| {
            value.wait_context_code()
        });
        engine.register_get(
            "system_failure_kind_code",
            |value: &mut ScriptRequestError| value.system_failure_kind_code(),
        );
        engine.register_get(
            "combat_intent_kind_code",
            |value: &mut ScriptRequestError| value.combat_intent_kind_code(),
        );
        engine.register_get(
            "combat_validation_kind_code",
            |value: &mut ScriptRequestError| value.combat_validation_kind_code(),
        );
        engine.register_get("spirit_index", |value: &mut ScriptRequestError| {
            value.spirit_index()
        });
        engine.register_get("value", |value: &mut ScriptRequestError| value.value());
        engine.register_get("operation_context", |value: &mut ScriptRequestError| {
            value.operation_context()
        });
        engine.register_type_with_name::<ScriptSpiritOperationError>("ScriptSpiritOperationError");
        engine.register_get("kind_code", |value: &mut ScriptSpiritOperationError| {
            value.kind_code()
        });
        engine.register_get("spirit_id", |value: &mut ScriptSpiritOperationError| {
            value.spirit_id()
        });
        engine.register_get("catch_time", |value: &mut ScriptSpiritOperationError| {
            value.catch_time()
        });
        engine.register_type_with_name::<ScriptCombatRuntimeError>("ScriptCombatRuntimeError");
        engine.register_type_with_name::<RocoReturnCodeKind>("RocoReturnCodeKind");
        engine.register_type_with_name::<RocoReturnCodeRejection>("RocoReturnCodeRejection");
        engine.register_type_with_name::<RocoHttpBusinessRejection>("RocoHttpBusinessRejection");
        engine.register_type_with_name::<ScriptModuleName>("ScriptModuleName");
        engine.register_type_with_name::<ScriptFunctionName>("ScriptFunctionName");
        engine.register_type_with_name::<ScriptHttpResponseName>("ScriptHttpResponseName");
        register_getters!(ScriptHttpResponseName, code);
        engine.register_type_with_name::<ScriptResponseName>("ScriptResponseName");
        register_getters!(ScriptResponseName, code);
        register_getters!(RocoHttpBusinessRejection, message);
        engine.register_get("result_code", |value: &mut RocoHttpBusinessRejection| {
            value.result_code()
        });
        engine.register_get(
            "request_context",
            |value: &mut RocoHttpBusinessRejection| value.request_context.clone(),
        );
        engine.register_get(
            "request_context_raw",
            |value: &mut RocoHttpBusinessRejection| value.request_context(),
        );
        engine.register_get("description", |value: &mut RocoHttpBusinessRejection| {
            value.description()
        });
        register_getters!(RocoReturnCodeRejection, kind, message);
        engine.register_get("code", |value: &mut RocoReturnCodeRejection| {
            i64::from(value.code())
        });
        engine.register_get("kind_code", |value: &mut RocoReturnCodeRejection| {
            value.kind_code()
        });
        engine.register_get("description", |value: &mut RocoReturnCodeRejection| {
            value.description()
        });
        engine.register_get("code", |value: &mut RocoReturnCodeKind| {
            i64::from(value.code())
        });
        engine.register_get("kind_code", |value: &mut RocoReturnCodeKind| {
            value.kind_code()
        });
        engine.register_get("code", |value: &mut ScriptModuleName| {
            value.code().to_string()
        });
        register_getters!(ScriptFunctionName, module, function);
        engine.register_get("module_code", |value: &mut ScriptFunctionName| {
            value.module_code()
        });
        engine.register_get("qualified_name", |value: &mut ScriptFunctionName| {
            value.qualified_name()
        });
        register_getters!(RocoErrorInfo, kind, code, message, detail);
        engine.register_get("kind_code", |value: &mut RocoErrorInfo| {
            value.kind.kind_code()
        });
        engine.register_get("detail_kind_code", |value: &mut RocoErrorInfo| {
            value.detail_kind_code()
        });
        engine.register_get("kind_code", |value: &mut RocoErrorDetail| value.kind_code());
        engine.register_get("general_lock_target", |value: &mut RocoErrorDetail| {
            value.general_lock_target()
        });
        engine.register_get("general_lock_target_code", |value: &mut RocoErrorDetail| {
            value.general_lock_target_code()
        });
        engine.register_get("invalid_param_kind_code", |value: &mut RocoErrorDetail| {
            value.invalid_param_kind_code()
        });
        engine.register_get("invalid_param_name", |value: &mut RocoErrorDetail| {
            value.invalid_param_name()
        });
        engine.register_get("invalid_param_value", |value: &mut RocoErrorDetail| {
            value.invalid_param_value()
        });
        engine.register_get("invalid_param_message", |value: &mut RocoErrorDetail| {
            value.invalid_param_message()
        });
        engine.register_get("invalid_param_expected", |value: &mut RocoErrorDetail| {
            value.invalid_param_expected()
        });
        engine.register_get(
            "invalid_param_protocol_field",
            |value: &mut RocoErrorDetail| value.invalid_param_protocol_field(),
        );
        engine.register_get(
            "invalid_param_rejected_source_code",
            |value: &mut RocoErrorDetail| value.invalid_param_rejected_source_code(),
        );
        engine.register_get("bridge_channel_code", |value: &mut RocoErrorDetail| {
            value.bridge_channel_code()
        });
        engine.register_get("bridge_operation_code", |value: &mut RocoErrorDetail| {
            value.bridge_operation_code()
        });
        engine.register_get("bridge_message", |value: &mut RocoErrorDetail| {
            value.bridge_message()
        });
        engine.register_get(
            "net_response_parse_target",
            |value: &mut RocoErrorDetail| value.net_response_parse_target(),
        );
        engine.register_get(
            "net_response_parse_source_kind_code",
            |value: &mut RocoErrorDetail| value.net_response_parse_source_kind_code(),
        );
        engine.register_get(
            "net_response_parse_protocol_message",
            |value: &mut RocoErrorDetail| value.net_response_parse_protocol_message(),
        );
        engine.register_get(
            "net_response_parse_protocol_error_type",
            |value: &mut RocoErrorDetail| value.net_response_parse_protocol_error_type(),
        );
        engine.register_get(
            "net_response_parse_unexpected_cmd_id",
            |value: &mut RocoErrorDetail| value.net_response_parse_unexpected_cmd_id(),
        );
        engine.register_get("unsupported_module", |value: &mut RocoErrorDetail| {
            value.unsupported_module()
        });
        engine.register_get(
            "unsupported_function_name",
            |value: &mut RocoErrorDetail| value.unsupported_function_name(),
        );
        engine.register_get(
            "function_context_kind_code",
            |value: &mut RocoErrorDetail| value.function_context_kind_code(),
        );
        engine.register_get(
            "function_context_combat_phase_code",
            |value: &mut RocoErrorDetail| value.function_context_combat_phase_code(),
        );
        engine.register_get("query_kind_code", |value: &mut RocoErrorDetail| {
            value.query_kind_code()
        });
        engine.register_get("query_skill_index", |value: &mut RocoErrorDetail| {
            value.query_skill_index()
        });
        engine.register_get("static_data_kind_code", |value: &mut RocoErrorDetail| {
            value.static_data_kind_code()
        });
        engine.register_get("static_data_position", |value: &mut RocoErrorDetail| {
            value.static_data_position()
        });
        engine.register_get(
            "static_data_function_name",
            |value: &mut RocoErrorDetail| value.static_data_function_name(),
        );
        engine.register_get("static_data_message", |value: &mut RocoErrorDetail| {
            value.static_data_message()
        });
        engine.register_get(
            "static_data_active_config_source_code",
            |value: &mut RocoErrorDetail| value.static_data_active_config_source_code(),
        );
        engine.register_get("system_operation_code", |value: &mut RocoErrorDetail| {
            value.system_operation_code()
        });
        engine.register_get("system_source_code", |value: &mut RocoErrorDetail| {
            value.system_source_code()
        });
        engine.register_get("system_message", |value: &mut RocoErrorDetail| {
            value.system_message()
        });
        engine.register_get(
            "stdlib_bridge_operation_code",
            |value: &mut RocoErrorDetail| value.stdlib_bridge_operation_code(),
        );
        engine.register_get("stdlib_bridge_message", |value: &mut RocoErrorDetail| {
            value.stdlib_bridge_message()
        });
        engine.register_get(
            "activity_operation_kind_code",
            |value: &mut RocoErrorDetail| value.activity_operation_kind_code(),
        );
        engine.register_get(
            "activity_operation_activity_code",
            |value: &mut RocoErrorDetail| value.activity_operation_activity_code(),
        );
        engine.register_get(
            "activity_operation_field_code",
            |value: &mut RocoErrorDetail| value.activity_operation_field_code(),
        );
        engine.register_get("activity_operation_count", |value: &mut RocoErrorDetail| {
            value.activity_operation_count()
        });
        engine.register_get("activity_operation_limit", |value: &mut RocoErrorDetail| {
            value.activity_operation_limit()
        });
        engine.register_get("activity_operation_value", |value: &mut RocoErrorDetail| {
            value.activity_operation_value()
        });
        engine.register_get("combat_action_kind_code", |value: &mut RocoErrorDetail| {
            value.combat_action_kind_code()
        });
        engine.register_get("combat_action_position", |value: &mut RocoErrorDetail| {
            value.combat_action_position()
        });
        engine.register_get("combat_action_skill_id", |value: &mut RocoErrorDetail| {
            value.combat_action_skill_id()
        });
        engine.register_get("combat_action_item_id", |value: &mut RocoErrorDetail| {
            value.combat_action_item_id()
        });
        engine.register_get("combat_wait_kind_code", |value: &mut RocoErrorDetail| {
            value.combat_wait_kind_code()
        });
        engine.register_get(
            "combat_wait_combat_phase_code",
            |value: &mut RocoErrorDetail| value.combat_wait_combat_phase_code(),
        );
        engine.register_get("combat_wait_elapsed_ms", |value: &mut RocoErrorDetail| {
            value.combat_wait_elapsed_ms()
        });
        engine.register_get("combat_runtime_message", |value: &mut RocoErrorDetail| {
            value.combat_runtime_message()
        });
        engine.register_get(
            "combat_command_failure_kind",
            |value: &mut RocoErrorDetail| value.combat_command_failure_kind(),
        );
        engine.register_get(
            "combat_command_failure_kind_code",
            |value: &mut RocoErrorDetail| value.combat_command_failure_kind_code(),
        );
        engine.register_get(
            "pending_response_kind_code",
            |value: &mut RocoErrorDetail| value.pending_response_kind_code(),
        );
        engine.register_get(
            "pending_http_response_code",
            |value: &mut RocoErrorDetail| value.pending_http_response_code(),
        );
        engine.register_get(
            "expected_http_response_code",
            |value: &mut RocoErrorDetail| value.expected_http_response_code(),
        );
        engine.register_get(
            "actual_http_response_code",
            |value: &mut RocoErrorDetail| value.actual_http_response_code(),
        );
        engine.register_get(
            "pending_response_combat_phase_code",
            |value: &mut RocoErrorDetail| value.pending_response_combat_phase_code(),
        );
        engine.register_get(
            "pending_response_net_target_code",
            |value: &mut RocoErrorDetail| value.pending_response_net_target_code(),
        );
        engine.register_get(
            "pending_response_expected_action_kind",
            |value: &mut RocoErrorDetail| value.pending_response_expected_action_kind(),
        );
        engine.register_get(
            "pending_response_expected_spirit_index",
            |value: &mut RocoErrorDetail| value.pending_response_expected_spirit_index(),
        );
        engine.register_get(
            "pending_response_actual_action_kind",
            |value: &mut RocoErrorDetail| value.pending_response_actual_action_kind(),
        );
        engine.register_get(
            "pending_response_actual_spirit_index",
            |value: &mut RocoErrorDetail| value.pending_response_actual_spirit_index(),
        );
        engine.register_get("response_kind_code", |value: &mut RocoErrorDetail| {
            value.response_kind_code()
        });
        engine.register_get("expected_response_code", |value: &mut RocoErrorDetail| {
            value.expected_response_code()
        });
        engine.register_get("actual_response_code", |value: &mut RocoErrorDetail| {
            value.actual_response_code()
        });
        engine.register_get("request_kind_code", |value: &mut RocoErrorDetail| {
            value.request_kind_code()
        });
        engine.register_get(
            "request_wait_context_code",
            |value: &mut RocoErrorDetail| value.request_wait_context_code(),
        );
        engine.register_get(
            "request_system_failure_kind_code",
            |value: &mut RocoErrorDetail| value.request_system_failure_kind_code(),
        );
        engine.register_get(
            "request_combat_intent_kind_code",
            |value: &mut RocoErrorDetail| value.request_combat_intent_kind_code(),
        );
        engine.register_get(
            "request_combat_validation_kind_code",
            |value: &mut RocoErrorDetail| value.request_combat_validation_kind_code(),
        );
        engine.register_get("request_spirit_index", |value: &mut RocoErrorDetail| {
            value.request_spirit_index()
        });
        engine.register_get("request_value", |value: &mut RocoErrorDetail| {
            value.request_value()
        });
        engine.register_get(
            "request_combat_protocol_error_kind_code",
            |value: &mut RocoErrorDetail| value.request_combat_protocol_error_kind_code(),
        );
        engine.register_get(
            "request_combat_protocol_error_value",
            |value: &mut RocoErrorDetail| value.request_combat_protocol_error_value(),
        );
        engine.register_get("session_memory_kind_code", |value: &mut RocoErrorDetail| {
            value.session_memory_kind_code()
        });
        engine.register_get("session_memory_key", |value: &mut RocoErrorDetail| {
            value.session_memory_key()
        });
        engine.register_get(
            "session_memory_expected_kind_code",
            |value: &mut RocoErrorDetail| value.session_memory_expected_kind_code(),
        );
        engine.register_get(
            "session_memory_actual_kind_code",
            |value: &mut RocoErrorDetail| value.session_memory_actual_kind_code(),
        );
        engine.register_get("lookup_kind_code", |value: &mut RocoErrorDetail| {
            value.lookup_kind_code()
        });
        engine.register_get("lookup_entity_code", |value: &mut RocoErrorDetail| {
            value.lookup_entity_code()
        });
        engine.register_get("lookup_key", |value: &mut RocoErrorDetail| {
            value.lookup_key()
        });
        engine.register_get(
            "spirit_operation_kind_code",
            |value: &mut RocoErrorDetail| value.spirit_operation_kind_code(),
        );
        engine.register_get(
            "spirit_operation_spirit_id",
            |value: &mut RocoErrorDetail| value.spirit_operation_spirit_id(),
        );
        engine.register_get(
            "spirit_operation_catch_time",
            |value: &mut RocoErrorDetail| value.spirit_operation_catch_time(),
        );
        engine.register_get("return_code_kind_code", |value: &mut RocoErrorDetail| {
            value.return_code_kind_code()
        });
        engine.register_get("return_code_value", |value: &mut RocoErrorDetail| {
            value.return_code_value()
        });
        engine.register_get("return_code_message", |value: &mut RocoErrorDetail| {
            value.return_code_message()
        });
        engine.register_get(
            "http_business_result_code",
            |value: &mut RocoErrorDetail| value.http_business_result_code(),
        );
        engine.register_get(
            "http_business_request_context",
            |value: &mut RocoErrorDetail| value.http_business_request_context(),
        );
        engine.register_get("http_business_message", |value: &mut RocoErrorDetail| {
            value.http_business_message()
        });
        engine.register_get("network_kind_code", |value: &mut RocoErrorInfo| {
            value.network_kind_code()
        });
        engine.register_get("kind_code", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(RocoErrorInfo::kind_code)
                .unwrap_or_default()
        });
        engine.register_get("detail_kind_code", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(RocoErrorInfo::detail_kind_code)
                .unwrap_or_default()
        });
        engine.register_get("detail", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(|info| info.detail.clone())
                .unwrap_or(RocoErrorDetail::None)
        });
        engine.register_get("network_kind_code", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(RocoErrorInfo::network_kind_code)
                .unwrap_or_default()
        });
        engine.register_get("code", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(|error| error.code.clone())
                .unwrap_or_default()
        });
        engine.register_get("message", |value: &mut Option<RocoErrorInfo>| {
            value
                .as_ref()
                .map(|error| error.message.clone())
                .unwrap_or_default()
        });
        engine.register_get("code", |value: &mut RocoErrorKind| value.kind_code());
        engine.register_get("family_code", |value: &mut RocoErrorKind| {
            value.family_code().to_string()
        });
        engine.register_get("network_kind", |value: &mut RocoErrorKind| match value {
            RocoErrorKind::Network { kind } => Some(kind.clone()),
            _ => None,
        });
        engine.register_get("code", |value: &mut RocoGeneralLockTarget| {
            value.code().to_string()
        });
        register_getters!(RocoInvalidParamInfo, kind, name, value, detail, message);
        engine.register_get("kind_code", |value: &mut RocoInvalidParamInfo| {
            value.kind_code()
        });
        engine.register_get("detail_kind_code", |value: &mut RocoInvalidParamInfo| {
            value.detail_kind_code()
        });
        engine.register_get("expected_text", |value: &mut RocoInvalidParamInfo| {
            value.expected_text()
        });
        engine.register_get("protocol_field", |value: &mut RocoInvalidParamInfo| {
            value.protocol_field()
        });
        engine.register_get("protocol_field_name", |value: &mut RocoInvalidParamInfo| {
            value.protocol_field_name()
        });
        engine.register_get(
            "rejected_source_code",
            |value: &mut RocoInvalidParamInfo| value.rejected_source_code(),
        );
        engine.register_get("code", |value: &mut RocoInvalidParamKind| {
            value.code().to_string()
        });
        engine.register_get("kind_code", |value: &mut RocoInvalidParamDetail| {
            value.kind_code()
        });
        engine.register_get("expected_text", |value: &mut RocoInvalidParamDetail| {
            value.expected_text()
        });
        engine.register_get("protocol_field", |value: &mut RocoInvalidParamDetail| {
            value.protocol_field()
        });
        engine.register_get(
            "protocol_field_name",
            |value: &mut RocoInvalidParamDetail| value.protocol_field_name(),
        );
        engine.register_get(
            "rejected_source_code",
            |value: &mut RocoInvalidParamDetail| value.rejected_source_code(),
        );
        engine.register_get("code", |value: &mut RocoProtocolFieldName| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut Option<RocoProtocolFieldName>| {
            value
                .as_ref()
                .map(|field| field.code().to_string())
                .unwrap_or_default()
        });
        engine.register_get(
            "system_operation_code",
            |value: &mut RocoInvalidParamDetail| value.system_operation_code(),
        );
        engine.register_get(
            "system_source_code",
            |value: &mut RocoInvalidParamDetail| value.system_source_code(),
        );
        engine.register_get("system_message", |value: &mut RocoInvalidParamDetail| {
            value.system_message()
        });
        engine.register_get("kind_code", |value: &mut RocoParamRange| {
            value.kind_code().to_string()
        });
        engine.register_get("min", |value: &mut RocoParamRange| value.min_value());
        engine.register_get("max", |value: &mut RocoParamRange| value.max_value());
        engine.register_get("type_name", |value: &mut RocoParamRange| value.type_name());
        engine.register_get("text", |value: &mut RocoParamRange| value.to_string());
        engine.register_get("kind_code", |value: &mut Option<RocoParamRange>| {
            value
                .as_ref()
                .map(|range| range.kind_code().to_string())
                .unwrap_or_default()
        });
        engine.register_get("min", |value: &mut Option<RocoParamRange>| {
            value
                .as_ref()
                .map(RocoParamRange::min_value)
                .unwrap_or_default()
        });
        engine.register_get("max", |value: &mut Option<RocoParamRange>| {
            value
                .as_ref()
                .map(RocoParamRange::max_value)
                .unwrap_or_default()
        });
        engine.register_get("type_name", |value: &mut Option<RocoParamRange>| {
            value
                .as_ref()
                .map(RocoParamRange::type_name)
                .unwrap_or_default()
        });
        engine.register_get("text", |value: &mut Option<RocoParamRange>| {
            value.as_ref().map(ToString::to_string).unwrap_or_default()
        });
        engine.register_get(
            "system_operation_code",
            |value: &mut RocoInvalidParamInfo| value.system_operation_code(),
        );
        engine.register_get("system_source_code", |value: &mut RocoInvalidParamInfo| {
            value.system_source_code()
        });
        engine.register_get("system_message", |value: &mut RocoInvalidParamInfo| {
            value.detail.system_message()
        });
        engine.register_get("code", |value: &mut RocoNetworkErrorKind| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut RocoBridgeErrorChannel| {
            value.code().to_string()
        });
        engine.register_type_with_name::<RocoHttpBridgeErrorKind>("RocoHttpBridgeErrorKind");
        engine.register_get("code", |value: &mut RocoHttpBridgeErrorKind| {
            value.code().to_string()
        });
        engine.register_type_with_name::<RocoNetBridgeErrorKind>("RocoNetBridgeErrorKind");
        engine.register_get("code", |value: &mut RocoNetBridgeErrorKind| {
            value.code().to_string()
        });
        engine.register_type_with_name::<RocoBridgeErrorKind>("RocoBridgeErrorKind");
        engine.register_get("code", |value: &mut RocoBridgeErrorKind| {
            value.code().to_string()
        });
        register_getters!(RocoBridgeErrorInfo, channel, kind, operation_code, message);
        engine.register_get("kind_code", |value: &mut RocoBridgeErrorInfo| {
            value.kind_code()
        });
        engine.register_get("channel_code", |value: &mut RocoBridgeErrorInfo| {
            value.channel_code()
        });
        engine.register_get("operation_code", |value: &mut RocoBridgeErrorInfo| {
            value.operation_code()
        });
        engine.register_get("description", |value: &mut RocoBridgeErrorInfo| {
            value.description()
        });
        register_getters!(RocoNetResponseParseFailure, target, source);
        engine.register_get("target_code", |value: &mut RocoNetResponseParseFailure| {
            value.target_code()
        });
        engine.register_get("target_label", |value: &mut RocoNetResponseParseFailure| {
            value.target_label()
        });
        engine.register_get(
            "source_kind_code",
            |value: &mut RocoNetResponseParseFailure| value.source_kind_code(),
        );
        engine.register_get(
            "protocol_message",
            |value: &mut RocoNetResponseParseFailure| value.protocol_message(),
        );
        engine.register_get(
            "protocol_reason",
            |value: &mut RocoNetResponseParseFailure| value.protocol_reason(),
        );
        engine.register_get(
            "protocol_reason_code",
            |value: &mut RocoNetResponseParseFailure| value.protocol_reason_code(),
        );
        engine.register_get(
            "protocol_error_type",
            |value: &mut RocoNetResponseParseFailure| value.protocol_error_type_code(),
        );
        engine.register_get(
            "protocol_layer",
            |value: &mut RocoNetResponseParseFailure| value.protocol_layer(),
        );
        engine.register_get(
            "protocol_layer_code",
            |value: &mut RocoNetResponseParseFailure| value.protocol_layer_code(),
        );
        engine.register_get(
            "protocol_kind_code",
            |value: &mut RocoNetResponseParseFailure| value.protocol_kind_code(),
        );
        engine.register_get(
            "unexpected_cmd_id",
            |value: &mut RocoNetResponseParseFailure| value.unexpected_cmd_id(),
        );
        engine.register_get("description", |value: &mut RocoNetResponseParseFailure| {
            value.description()
        });
        engine.register_get("kind_code", |value: &mut RocoNetResponseParseSource| {
            value.kind_code().to_string()
        });
        engine.register_get(
            "protocol_message",
            |value: &mut RocoNetResponseParseSource| value.protocol_message(),
        );
        engine.register_get(
            "protocol_reason",
            |value: &mut RocoNetResponseParseSource| value.protocol_reason(),
        );
        engine.register_get(
            "protocol_reason_code",
            |value: &mut RocoNetResponseParseSource| value.protocol_reason_code(),
        );
        engine.register_get(
            "protocol_error_type",
            |value: &mut RocoNetResponseParseSource| value.protocol_error_type_code(),
        );
        engine.register_get(
            "protocol_layer",
            |value: &mut RocoNetResponseParseSource| value.protocol_layer(),
        );
        engine.register_get(
            "protocol_layer_code",
            |value: &mut RocoNetResponseParseSource| value.protocol_layer_code(),
        );
        engine.register_get(
            "protocol_kind_code",
            |value: &mut RocoNetResponseParseSource| value.protocol_kind_code(),
        );
        engine.register_get(
            "unexpected_cmd_id",
            |value: &mut RocoNetResponseParseSource| value.unexpected_cmd_id(),
        );
        engine.register_get("code", |value: &mut RocoNetResponseParseTarget| {
            value.code().to_string()
        });
        engine.register_get("label", |value: &mut RocoNetResponseParseTarget| {
            value.label().to_string()
        });
        engine.register_get("code", |value: &mut RocoProtocolParseFailureKind| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut RocoProtocolParseReason| {
            value.code().to_string()
        });
        engine.register_get("message", |value: &mut RocoProtocolParseReason| {
            value.message()
        });
        engine.register_get("context_code", |value: &mut RocoProtocolParseReason| {
            value.context_code()
        });
        engine.register_get("field_name", |value: &mut RocoProtocolParseReason| {
            value.field_name()
        });
        engine.register_get("field_code", |value: &mut RocoProtocolParseReason| {
            value.field_code()
        });
        engine.register_get(
            "spirit_storage_context_code",
            |value: &mut RocoProtocolParseReason| value.spirit_storage_context_code(),
        );
        engine.register_get(
            "spirit_storage_field",
            |value: &mut RocoProtocolParseReason| value.spirit_storage_field(),
        );
        engine.register_get(
            "spirit_storage_field_code",
            |value: &mut RocoProtocolParseReason| value.spirit_storage_field_code(),
        );
        engine.register_get("code", |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::code)
                .unwrap_or_default()
        });
        engine.register_get("message", |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::message)
                .unwrap_or_default()
        });
        engine.register_get(
            "context_code",
            |value: &mut Option<RocoProtocolParseReason>| {
                value
                    .as_ref()
                    .map(RocoProtocolParseReason::context_code)
                    .unwrap_or_default()
            },
        );
        engine.register_get(
            "field_name",
            |value: &mut Option<RocoProtocolParseReason>| {
                value.as_ref().and_then(RocoProtocolParseReason::field_name)
            },
        );
        engine.register_get(
            "field_code",
            |value: &mut Option<RocoProtocolParseReason>| {
                value
                    .as_ref()
                    .map(RocoProtocolParseReason::field_code)
                    .unwrap_or_default()
            },
        );
        engine.register_get(
            "spirit_storage_context_code",
            |value: &mut Option<RocoProtocolParseReason>| {
                value
                    .as_ref()
                    .map(RocoProtocolParseReason::spirit_storage_context_code)
                    .unwrap_or_default()
            },
        );
        engine.register_get(
            "spirit_storage_field",
            |value: &mut Option<RocoProtocolParseReason>| {
                value
                    .as_ref()
                    .and_then(RocoProtocolParseReason::spirit_storage_field)
            },
        );
        engine.register_get(
            "spirit_storage_field_code",
            |value: &mut Option<RocoProtocolParseReason>| {
                value
                    .as_ref()
                    .map(RocoProtocolParseReason::spirit_storage_field_code)
                    .unwrap_or_default()
            },
        );
        engine.register_get("code", |value: &mut RocoProtocolParseContext| {
            value.code().to_string()
        });
        engine.register_get("label", |value: &mut RocoProtocolParseContext| {
            value.label().to_string()
        });
        engine.register_get("code", |value: &mut RocoSpiritStorageProtoContext| {
            value.code().to_string()
        });
        engine.register_get("label", |value: &mut RocoSpiritStorageProtoContext| {
            value.label().to_string()
        });
        engine.register_get("code", |value: &mut RocoSpiritStorageProtoField| {
            value.code().to_string()
        });
        engine.register_get("label", |value: &mut RocoSpiritStorageProtoField| {
            value.label().to_string()
        });
        engine.register_get("code", |value: &mut RocoProtocolParseErrorType| {
            value.code().to_string()
        });
        engine.register_get("layer", |value: &mut RocoProtocolParseErrorType| {
            value.layer()
        });
        engine.register_get("layer_code", |value: &mut RocoProtocolParseErrorType| {
            value.layer().code().to_string()
        });
        engine.register_get("code", |value: &mut RocoProtocolParseLayer| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut ScriptBridgeOperation| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut ScriptBridgeFailureReason| {
            value.code().to_string()
        });
        engine.register_get("message", |value: &mut ScriptBridgeFailureReason| {
            value.message().to_string()
        });
        register_getters!(ScriptBridgeFailure, operation, reason);
        engine.register_get("message", |value: &mut ScriptBridgeFailure| value.message());
        engine.register_get("operation_code", |value: &mut ScriptBridgeFailure| {
            value.operation_code()
        });
        engine.register_get("reason_code", |value: &mut ScriptBridgeFailure| {
            value.reason_code()
        });
        engine.register_get("description", |value: &mut ScriptBridgeFailure| {
            value.description()
        });
        engine.register_get("code", |value: &mut ScriptSystemOperation| {
            value.code().to_string()
        });
        engine.register_get("code", |value: &mut ScriptSystemFailureSource| {
            value.code().to_string()
        });
        register_getters!(ScriptSystemFailure, operation, source);
        engine.register_get("operation_code", |value: &mut ScriptSystemFailure| {
            value.operation_code()
        });
        engine.register_get("source_code", |value: &mut ScriptSystemFailure| {
            value.source_code()
        });
        engine.register_get("message", |value: &mut ScriptSystemFailure| value.message());
        engine.register_get("description", |value: &mut ScriptSystemFailure| {
            value.description()
        });
        engine.register_get("code", |value: &mut ScriptCombatCommandFailureKind| {
            value.code().to_string()
        });
        engine.register_get("message", |value: &mut ScriptCombatRuntimeError| {
            value.message()
        });
        engine.register_get(
            "command_failure_kind",
            |value: &mut ScriptCombatRuntimeError| value.command_failure_kind(),
        );
        engine.register_get(
            "command_failure_kind_code",
            |value: &mut ScriptCombatRuntimeError| value.command_failure_kind_code(),
        );
        engine.register_get("backend_kind", |value: &mut ScriptCombatRuntimeError| {
            value.backend_kind()
        });
        engine.register_get(
            "backend_kind_code",
            |value: &mut ScriptCombatRuntimeError| value.backend_kind_code(),
        );
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
