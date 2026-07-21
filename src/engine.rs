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
    AlchemyFurnaceBagCandidate, AlchemyFurnaceRewardItem, AmendNatureCandidate, AmendNatureInfo,
    AquariusBagCandidate, AquariusCounter, AquariusField, AquariusFirstInfo, AquariusRewardItem,
    AquariusSecondExchangeInfo, AquariusSecondInfo, AquariusSecondStatusInfo, AquariusThirdInfo,
    AriesBagCandidate, AriesCounter, AriesField, AriesFirstInfo, AriesReward, AriesSecondInfo,
    AriesThirdExchangeInfo, AriesThirdInfo, AriesThirdStatusInfo, BatheSunInfo, BloodGiftInfo,
    BloodGiftItemRequirement, BloodGiftOption, CancerItemInfo, CancerMendShapeBagInfo,
    CancerMendShapeInfo, CancerPetInfo, CancerSharpScorpionInfo, CancerUnsealMemoriesBagInfo,
    CancerUnsealMemoriesInfo, CapricornBagCandidate, CapricornInviteListInfo,
    CapricornPalaceNoteItem, CapricornPalaceNotesInfo, CapricornSecondInfo, CapricornSecondTask,
    CapricornStarPalaceInfo, CapricornTeamOperationInfo, CapricornTeamPlayer,
    CapricornTeamSnapshot, CapricornThirdInfo, DarkCityExchangeItem, DarkCityExpeditionInfo,
    DarkCityReputationInfo, DiamondTearInfo, DiamondTearRewardItem, FiresWillInfo, FourSeasonsInfo,
    FourSeasonsMonthlySpiritRewardInfo, FourSeasonsRewardItem, FourSeasonsShopRewardInfo,
    GeminiBagCandidate, GeminiCounter, GeminiField, GeminiFirstInfo, GeminiRewardItem,
    GeminiSecondInfo, GeminiThirdInfo, IceCrystalBagCandidate, IceCrystalBattleInfo,
    IceCrystalInfo, IceCrystalRewardItem, IncubativeMachineActionResult, IncubativeMachineEggInfo,
    IncubativeMachineEggListResult, IncubativeMachineGetSpiritResult,
    IncubativeMachineIncubationInfo, IncubativeMachineIncubationResult, IncubativeMachineInfo,
    LadderFightRecord, LadderInfo, LadderMatchConfig, LadderQuestConfigEntry, LadderQuestInfo,
    LadderRankInfo, LadderRankUser, LadderSpiritCostEntry, LadderSpiritInfo, LeoBagCandidate,
    LeoCounter, LeoField, LeoFirstExchangeInfo, LeoFirstInfo, LeoFirstStatusInfo, LeoSecondInfo,
    LeoThirdInfo, LibraBagCandidate, LibraCounter, LibraField, LibraFirstInfo, LibraSecondInfo,
    LibraThirdExchangeInfo, LibraThirdInfo, LibraThirdStatusInfo, MagicPioneerField,
    MagicPioneerInfo, MagicPioneerRewardItem, ManorFertilizerResult, ManorGroundInfo, ManorInfo,
    ManorItemCount, ManorReapResult, ManorRewardInfo, ManorSowResult, ManorUprootResult,
    ManorWeedResult, MonkeyCultivationInfo, MonkeyEvoInfo, MountainSeaBossInfo, MountainSeaInfo,
    MountainSeaSoulInfo, MultiEvolutionCandidate, MultiEvolutionInfo, MultiEvolutionRewardItem,
    MysteryFusionBattleInfo, MysteryFusionInfo, MysteryFusionMaterialBag,
    MysteryFusionMaterialCandidate, MysteryFusionRecipeInfo, PetEggBeginResult, PetEggCancelResult,
    PetEggInfo, PetEggPreviewResult, PetEggSpeedUpResult, PetEggSpiritInfo, PiscesBagCandidate,
    PiscesCounter, PiscesField, PiscesFirstInfo, PiscesSecondInfo, PiscesThirdInfo, RagingFireInfo,
    RemoteSceneData, RocoRequestContext, RocoRewardKind, SagittariusBagCandidate,
    SagittariusCounter, SagittariusField, SagittariusFirstInfo, SagittariusRewardItem,
    SagittariusScore, SagittariusSecondInfo, SagittariusStarPicture, SagittariusThirdInfo,
    ScorpioBagCandidate, ScorpioCounter, ScorpioField, ScorpioFirstInfo, ScorpioReward,
    ScorpioSecondInfo, ScorpioThirdInfo, SentinelBossInfo, SentinelExchangeInfo,
    SentinelIntelligenceInfo, SentinelSpiritExchangeInfo, SpiritBookEntry, SpiritBookGroup,
    SpiritBookInfo, SpiritBookSpiritState, SpiritBookStates, SpiritBookSummary,
    SpiritEquipmentBagInfo, SpiritEquipmentInfo, StaticGuardianPetPropertyInfo, StaticItemInfo,
    StaticMagicInfo, StaticPluginInfo, StaticSkillInfo, StaticSpiritEvolutionEdge,
    StaticSpiritInfo, StaticSpiritInfoLookupResult, StaticStriveItemInfo, StaticTitleInfo,
    SummonExchangeGroup, SummonExchangeItem, SummonInfo, SummonPoolConfig, SummonPoolState,
    SummonRecord, SummonRewardItem, TalentRefreshResult, TaskProgressResult, TaurusBagCandidate,
    TaurusCounter, TaurusField, TaurusFirstInfo, TaurusSecondInfo, TaurusThirdInfo,
    ThreeStartersBagCandidate, ThreeStartersCounter, ThreeStartersField, ThreeStartersRewardItem,
    TreasureRealmInfo, TypeLadderFightRecord, TypeLadderInfo, TypeLadderRank, TypeLadderRankInfo,
    TypeLadderRankUser, TypeLadderSpiritInfo, UnicornBagCandidate, UnicornBossInfo, UnicornInfo,
    UnicornRewardItem, VirgoBellFoxExchangeInfo, VirgoBellFoxInfo, VirgoBellFoxStatusInfo,
    VirgoCounter, VirgoField, VirgoFindHalidomInfo, VirgoPetInfo, VirgoServeGodInfo,
    WaterSourceInfo,
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
        self.engine.eval(script).map_err(Self::map_eval_error)
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
        macro_rules! register_virgo_cgi_info {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                register_getters!($type, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $type| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $type| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("states", |value: &mut $type| Self::to_array(&value.states));
                engine.register_get("pets", |value: &mut $type| Self::to_array(&value.pets));
            };
        }
        macro_rules! register_libra_cgi_info {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                register_getters!($type, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $type| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $type| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("bag_candidates", |value: &mut $type| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        macro_rules! register_scorpio_cgi_info {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                register_getters!($type, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $type| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $type| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("counts", |value: &mut $type| Self::to_array(&value.counts));
                engine.register_get("rewards", |value: &mut $type| {
                    Self::to_array(&value.rewards)
                });
                engine.register_get("bag_candidates", |value: &mut $type| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        macro_rules! register_sagittarius_cgi_info {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                register_getters!($type, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $type| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $type| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("scores", |value: &mut $type| Self::to_array(&value.scores));
                engine.register_get("star_pictures", |value: &mut $type| {
                    Self::to_array(&value.star_pictures)
                });
                engine.register_get("rewards", |value: &mut $type| {
                    Self::to_array(&value.rewards)
                });
                engine.register_get("bag_candidates", |value: &mut $type| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
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
        register_getters!(
            TalentRefreshResult,
            position,
            pa_ability_old,
            pd_ability_old,
            ma_ability_old,
            md_ability_old,
            sp_ability_old,
            hp_ability_old,
            pa_ability_new,
            pd_ability_new,
            ma_ability_new,
            md_ability_new,
            sp_ability_new,
            hp_ability_new,
            pa_talent_old,
            pd_talent_old,
            ma_talent_old,
            md_talent_old,
            sp_talent_old,
            hp_talent_old,
            pa_talent_new,
            pd_talent_new,
            ma_talent_new,
            md_talent_new,
            sp_talent_new,
            hp_talent_new,
        );
        register_getters!(
            SentinelBossInfo,
            index,
            spirit_id,
            difficulty,
            status,
            max_intelligence,
            intelligence,
        );
        register_getters!(SentinelExchangeInfo, index, item_id, need_bounty, status);
        register_getters!(
            SentinelSpiritExchangeInfo,
            index,
            spirit_id,
            need_intelligence,
            evolve_spirit_id,
            status,
        );
        register_getters!(
            SentinelIntelligenceInfo,
            result_code,
            message,
            fight_id,
            added_bounty,
            refresh_count,
            exchange_refresh_count,
            mission_type,
            fight_times,
            bounty,
            intelligence_count,
        );
        engine.register_get("mission_values", |value: &mut SentinelIntelligenceInfo| {
            Self::to_array(&value.mission_values)
        });
        engine.register_get("bosses", |value: &mut SentinelIntelligenceInfo| {
            Self::to_array(&value.bosses)
        });
        engine.register_get("exchanges", |value: &mut SentinelIntelligenceInfo| {
            Self::to_array(&value.exchanges)
        });
        engine.register_get("spirits", |value: &mut SentinelIntelligenceInfo| {
            Self::to_array(&value.spirits)
        });
        register_getters!(
            MountainSeaBossInfo,
            index,
            boss_type,
            fight_id,
            name,
            status,
        );
        register_getters!(MountainSeaSoulInfo, soul_type, boss_type, name, count);
        register_getters!(
            MountainSeaInfo,
            result_code,
            message,
            fight_id,
            seal_count,
            success,
        );
        engine.register_get("attrs", |value: &mut MountainSeaInfo| {
            Self::to_array(&value.attrs)
        });
        engine.register_get("bosses", |value: &mut MountainSeaInfo| {
            Self::to_array(&value.bosses)
        });
        engine.register_get("souls", |value: &mut MountainSeaInfo| {
            Self::to_array(&value.souls)
        });
        register_getters!(
            DarkCityExpeditionInfo,
            result_code,
            message,
            fight_id,
            fight_index,
            vip,
            vip_pass_enabled,
            schedule,
            schedule_name,
            added_reputation,
        );
        register_getters!(DarkCityExchangeItem, index, item_id, cost);
        register_getters!(DarkCityReputationInfo, result_code, message, reputation);
        engine.register_get("exchanges", |value: &mut DarkCityReputationInfo| {
            Self::to_array(&value.exchanges)
        });
        register_getters!(MysteryFusionBattleInfo, index, battle_id);
        engine.register_get("attr_types", |value: &mut MysteryFusionBattleInfo| {
            Self::to_array(&value.attr_types)
        });
        register_getters!(MysteryFusionRecipeInfo, index, spirit_id, energy_cost);
        engine.register_get(
            "required_spirit_ids",
            |value: &mut MysteryFusionRecipeInfo| Self::to_array(&value.required_spirit_ids),
        );
        register_getters!(
            MysteryFusionInfo,
            result_code,
            message,
            times,
            energy,
            added_energy,
        );
        engine.register_get("battles", |value: &mut MysteryFusionInfo| {
            Self::to_array(&value.battles)
        });
        engine.register_get("recipes", |value: &mut MysteryFusionInfo| {
            Self::to_array(&value.recipes)
        });

        register_getters!(
            MysteryFusionMaterialCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            level,
            personality,
        );
        register_getters!(MysteryFusionMaterialBag, result_code, message);
        engine.register_get("candidates", |value: &mut MysteryFusionMaterialBag| {
            Self::to_array(&value.candidates)
        });
        register_getters!(
            TreasureRealmInfo,
            result_code,
            message,
            battle,
            battle_id,
            schedule,
            possible,
            time,
            got_box,
        );
        engine.register_get("item_counts", |value: &mut TreasureRealmInfo| {
            Self::to_array(&value.item_counts)
        });
        engine.register_get("commits", |value: &mut TreasureRealmInfo| {
            Self::to_array(&value.commits)
        });
        register_getters!(SummonRewardItem, id, item_type, count);
        register_getters!(
            SummonPoolState,
            version,
            token_item_id,
            token_count,
            today_draw_count,
            wish_index,
            succeeded,
            end_time,
        );
        register_getters!(
            SummonPoolConfig,
            version,
            title,
            vip_limit,
            end_time,
            daily_max,
            token_item_id,
            recommend,
            info,
            reward_text,
        );
        engine.register_get("rewards", |value: &mut SummonPoolConfig| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("wish_candidates", |value: &mut SummonPoolConfig| {
            Self::to_array(&value.wish_candidates)
        });
        register_getters!(
            SummonExchangeItem,
            index,
            reward,
            cost,
            need,
            max,
            day_max,
            times,
            day_times,
            add,
        );
        register_getters!(SummonExchangeGroup, kind);
        engine.register_get("items", |value: &mut SummonExchangeGroup| {
            Self::to_array(&value.items)
        });
        register_getters!(
            SummonRecord,
            pool_version,
            title,
            id,
            item_type,
            count,
            year,
            month,
            day,
        );
        register_getters!(SummonInfo, result_code, message, vip, magic, count, show,);
        engine.register_get("pools", |value: &mut SummonInfo| {
            Self::to_array(&value.pools)
        });
        engine.register_get("config_pools", |value: &mut SummonInfo| {
            Self::to_array(&value.config_pools)
        });
        engine.register_get("exchange_groups", |value: &mut SummonInfo| {
            Self::to_array(&value.exchange_groups)
        });
        engine.register_get("rewards", |value: &mut SummonInfo| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("records", |value: &mut SummonInfo| {
            Self::to_array(&value.records)
        });
        register_getters!(CapricornPalaceNoteItem, item_index, item_id, count, need);
        register_getters!(CapricornPalaceNotesInfo, can_summon);
        engine.register_get("items", |value: &mut CapricornPalaceNotesInfo| {
            Self::to_array(&value.items)
        });
        register_getters!(CapricornTeamPlayer, uin, nick);
        register_getters!(CapricornTeamSnapshot, ticks);
        engine.register_get("players", |value: &mut CapricornTeamSnapshot| {
            Self::to_array(&value.players)
        });
        register_getters!(CapricornInviteListInfo, result_code, message, ticks);
        engine.register_get("players", |value: &mut CapricornInviteListInfo| {
            Self::to_array(&value.players)
        });
        register_getters!(CapricornTeamOperationInfo, result_code, message, team,);
        register_getters!(CapricornSecondTask, task_type, data1, data2, step, current,);
        register_getters!(
            CapricornBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );
        register_getters!(
            CapricornStarPalaceInfo,
            result_code,
            message,
            request_context,
        );
        register_getters!(
            CapricornSecondInfo,
            result_code,
            message,
            request_context,
            finish,
            current,
            position,
            second_task,
        );
        engine.register_get("bag_candidates", |value: &mut CapricornSecondInfo| {
            Self::to_array(&value.bag_candidates)
        });
        register_getters!(
            CapricornThirdInfo,
            result_code,
            message,
            request_context,
            finish,
            current,
            remain,
            price,
            limit,
            progress_percent,
            reward_num,
            tips,
        );
        engine.register_get("bag_candidates", |value: &mut CapricornThirdInfo| {
            Self::to_array(&value.bag_candidates)
        });
        register_getters!(CancerItemInfo, id, count, item_type);
        register_getters!(CancerPetInfo, id, catch_time, level, need_money);
        register_getters!(
            CancerSharpScorpionInfo,
            result_code,
            message,
            request_context,
            light_num,
            tail_num,
            boss_left_hp,
            boss_full_hp,
            left_fight_count,
            add_hit_level,
            today_sum_hit,
            exchange_count0,
            exchange_count1,
            display_item,
        );
        register_getters!(
            CancerMendShapeInfo,
            result_code,
            message,
            request_context,
            left_times,
            step,
            complete,
        );
        register_getters!(
            CancerMendShapeBagInfo,
            result_code,
            message,
            request_context,
        );
        engine.register_get("pets", |value: &mut CancerMendShapeBagInfo| {
            Self::to_array(&value.pets)
        });
        register_getters!(
            CancerUnsealMemoriesInfo,
            result_code,
            message,
            request_context,
            advance,
            level,
            power,
            event,
            pass,
            finish,
            schedule,
        );
        register_getters!(
            CancerUnsealMemoriesBagInfo,
            result_code,
            message,
            request_context,
        );
        engine.register_get("pets", |value: &mut CancerUnsealMemoriesBagInfo| {
            Self::to_array(&value.pets)
        });
        register_getters!(VirgoField, name, value);
        register_getters!(VirgoCounter, name, current, limit);
        register_getters!(
            VirgoPetInfo,
            candidate_index,
            spirit_id,
            has_bag_index,
            bag_index,
            catch_time,
            has_level,
            level,
            has_need_money,
            need_money,
        );

        register_virgo_cgi_info!(VirgoServeGodInfo, "VirgoServeGodInfo");
        register_virgo_cgi_info!(VirgoFindHalidomInfo, "VirgoFindHalidomInfo");
        register_virgo_cgi_info!(VirgoBellFoxInfo, "VirgoBellFoxInfo");
        register_getters!(
            VirgoBellFoxStatusInfo,
            result_code,
            message,
            light_num,
            tail_num,
            boss_left_hp,
            boss_full_hp,
            left_fight_count,
            add_hit_level,
            today_sum_hit,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(
            VirgoBellFoxExchangeInfo,
            result_code,
            message,
            item,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(PiscesField, name, value);
        register_getters!(PiscesCounter, name, current, limit);
        register_getters!(
            PiscesBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        macro_rules! register_pisces_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("lights", |value: &mut $ty| Self::to_array(&value.lights));
                engine.register_get("exchanges", |value: &mut $ty| {
                    Self::to_array(&value.exchanges)
                });
                engine.register_get("fights", |value: &mut $ty| Self::to_array(&value.fights));
                engine.register_get("days", |value: &mut $ty| Self::to_array(&value.days));
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }

        register_pisces_info!(PiscesFirstInfo, "PiscesFirstInfo");
        register_pisces_info!(PiscesSecondInfo, "PiscesSecondInfo");
        register_pisces_info!(PiscesThirdInfo, "PiscesThirdInfo");
        register_getters!(TaurusField, name, value);
        register_getters!(TaurusCounter, name, current, limit);
        register_getters!(
            TaurusBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        macro_rules! register_taurus_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("item_counts", |value: &mut $ty| {
                    Self::to_array(&value.item_counts)
                });
                engine.register_get("states", |value: &mut $ty| Self::to_array(&value.states));
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        register_taurus_info!(TaurusFirstInfo, "TaurusFirstInfo");
        register_taurus_info!(TaurusSecondInfo, "TaurusSecondInfo");
        register_taurus_info!(TaurusThirdInfo, "TaurusThirdInfo");
        register_getters!(ThreeStartersField, name, value);
        register_getters!(ThreeStartersCounter, name, current, limit);
        register_getters!(
            ThreeStartersRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            ThreeStartersBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );
        register_getters!(MagicPioneerField, name);
        engine.register_get("values", |value: &mut MagicPioneerField| {
            Self::to_array(&value.values)
        });
        register_getters!(
            MagicPioneerRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            MagicPioneerInfo,
            pet,
            cmd,
            result_code,
            message,
            request_context
        );
        engine.register_get("fields", |value: &mut MagicPioneerInfo| {
            Self::to_array(&value.fields)
        });
        engine.register_get("rewards", |value: &mut MagicPioneerInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            AlchemyFurnaceRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            AlchemyFurnaceBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );
        register_getters!(
            MonkeyCultivationInfo,
            result_code,
            message,
            request_context,
            daytimes,
            finish,
            progress,
            add_progress
        );
        engine.register_get("pill_counts", |value: &mut MonkeyCultivationInfo| {
            Self::to_array(&value.pill_counts)
        });
        engine.register_get("rewards", |value: &mut MonkeyCultivationInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            MonkeyEvoInfo,
            result_code,
            message,
            request_context,
            branch_type,
            done,
            schedule,
            add_progress
        );
        engine.register_get("pill_counts", |value: &mut MonkeyEvoInfo| {
            Self::to_array(&value.pill_counts)
        });
        engine.register_get("bag_candidates", |value: &mut MonkeyEvoInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("rewards", |value: &mut MonkeyEvoInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            RagingFireInfo,
            result_code,
            message,
            request_context,
            vip,
            daytimes,
            finish,
            fusion,
            add_progress
        );
        engine.register_get("required_stone_indexes", |value: &mut RagingFireInfo| {
            Self::to_array(&value.required_stone_indexes)
        });
        engine.register_get("progress", |value: &mut RagingFireInfo| {
            Self::to_array(&value.progress)
        });
        engine.register_get("bag_candidates", |value: &mut RagingFireInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("rewards", |value: &mut RagingFireInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            UnicornRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            UnicornBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );
        register_getters!(UnicornBossInfo, slot, npc_index, spirit_id, fight_id);
        register_getters!(
            UnicornInfo,
            result_code,
            message,
            request_context,
            finish,
            start,
            total,
            book,
            purple_vine_count,
            energy,
            fruit_count,
            increase
        );
        engine.register_get("bosses", |value: &mut UnicornInfo| {
            Self::to_array(&value.bosses)
        });
        engine.register_get("cultivation_times", |value: &mut UnicornInfo| {
            Self::to_array(&value.cultivation_times)
        });
        engine.register_get("evolution_energy_costs", |value: &mut UnicornInfo| {
            Self::to_array(&value.evolution_energy_costs)
        });
        engine.register_get("one_key_diamond_costs", |value: &mut UnicornInfo| {
            Self::to_array(&value.one_key_diamond_costs)
        });
        engine.register_get("bag_candidates", |value: &mut UnicornInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("rewards", |value: &mut UnicornInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            FourSeasonsRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(FourSeasonsShopRewardInfo, reward_id, reward_kind, count);

        register_getters!(
            FourSeasonsMonthlySpiritRewardInfo,
            month,
            reward_index,
            spirit_id,
            ticket_cost
        );
        register_getters!(
            FourSeasonsInfo,
            result_code,
            message,
            request_context,
            month,
            map,
            position_1based,
            times,
            ticket,
            used_tool_index,
            need_item_index,
            add,
            point
        );
        engine.register_get("boxes", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.boxes)
        });
        engine.register_get("tools", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.tools)
        });
        engine.register_get("tool_shop_indexes", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.tool_shop_indexes)
        });
        engine.register_get("tool_shop_flags", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.tool_shop_flags)
        });
        engine.register_get("pass_boxes", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.pass_boxes)
        });
        engine.register_get("tool_costs", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.tool_costs)
        });
        engine.register_get("event_item_counts", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.event_item_counts)
        });
        engine.register_get("shop_rewards", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.shop_rewards)
        });
        engine.register_get("monthly_spirit_rewards", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.monthly_spirit_rewards)
        });
        engine.register_get("rewards", |value: &mut FourSeasonsInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            DiamondTearRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            DiamondTearInfo,
            result_code,
            message,
            request_context,
            buy,
            level,
            count_down,
            tear_state
        );
        engine.register_get("rewards", |value: &mut DiamondTearInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(IceCrystalBattleInfo, battle_index, fight_id);
        register_getters!(
            IceCrystalBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money
        );
        register_getters!(
            IceCrystalRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            IceCrystalInfo,
            result_code,
            message,
            request_context,
            progress,
            battle_times,
            battle_index,
            get_times,
            add,
            current_battle
        );
        engine.register_get("item_counts", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.item_counts)
        });
        engine.register_get("crystal_counts", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.crystal_counts)
        });
        engine.register_get("item_costs", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.item_costs)
        });
        engine.register_get("one_key_diamond_costs", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.one_key_diamond_costs)
        });
        engine.register_get("bag_candidates", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("rewards", |value: &mut IceCrystalInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            MultiEvolutionCandidate,
            candidate_index,
            spirit_id,
            catch_time,
            condition_code,
            condition_name
        );
        register_getters!(
            MultiEvolutionRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            MultiEvolutionInfo,
            result_code,
            message,
            request_context,
            pet_id,
            result_side,
            item_id,
            count,
            available
        );
        engine.register_get("candidates", |value: &mut MultiEvolutionInfo| {
            Self::to_array(&value.candidates)
        });
        engine.register_get("rewards", |value: &mut MultiEvolutionInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(
            WaterSourceInfo,
            result_code,
            message,
            request_context,
            battle,
            schedule,
            time,
            increase
        );
        engine.register_get("fields", |value: &mut WaterSourceInfo| {
            Self::to_array(&value.fields)
        });
        engine.register_get("counters", |value: &mut WaterSourceInfo| {
            Self::to_array(&value.counters)
        });
        engine.register_get("rewards", |value: &mut WaterSourceInfo| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("bag_candidates", |value: &mut WaterSourceInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("water", |value: &mut WaterSourceInfo| {
            Self::to_array(&value.water)
        });
        register_getters!(
            FiresWillInfo,
            result_code,
            message,
            request_context,
            schedule,
            num
        );
        engine.register_get("fields", |value: &mut FiresWillInfo| {
            Self::to_array(&value.fields)
        });
        engine.register_get("counters", |value: &mut FiresWillInfo| {
            Self::to_array(&value.counters)
        });
        engine.register_get("rewards", |value: &mut FiresWillInfo| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("bag_candidates", |value: &mut FiresWillInfo| {
            Self::to_array(&value.bag_candidates)
        });
        engine.register_get("fire", |value: &mut FiresWillInfo| {
            Self::to_array(&value.fire)
        });
        register_getters!(
            BatheSunInfo,
            result_code,
            message,
            request_context,
            battle,
            schedule,
            time,
            num,
            act,
            times,
            sun,
            add
        );
        engine.register_get("fields", |value: &mut BatheSunInfo| {
            Self::to_array(&value.fields)
        });
        engine.register_get("counters", |value: &mut BatheSunInfo| {
            Self::to_array(&value.counters)
        });
        engine.register_get("rewards", |value: &mut BatheSunInfo| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("bag_candidates", |value: &mut BatheSunInfo| {
            Self::to_array(&value.bag_candidates)
        });
        register_getters!(GeminiField, name, value);
        register_getters!(GeminiCounter, name, current, limit);
        register_getters!(
            GeminiRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            GeminiBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        macro_rules! register_gemini_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("scores", |value: &mut $ty| Self::to_array(&value.scores));
                engine.register_get("sun_scores", |value: &mut $ty| {
                    Self::to_array(&value.sun_scores)
                });
                engine.register_get("moon_scores", |value: &mut $ty| {
                    Self::to_array(&value.moon_scores)
                });
                engine.register_get("rewards", |value: &mut $ty| Self::to_array(&value.rewards));
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        register_gemini_info!(GeminiFirstInfo, "GeminiFirstInfo");
        register_gemini_info!(GeminiSecondInfo, "GeminiSecondInfo");
        register_gemini_info!(GeminiThirdInfo, "GeminiThirdInfo");
        register_getters!(SagittariusField, name, value);
        register_getters!(SagittariusCounter, name, current, limit);
        register_getters!(SagittariusScore, score_index, current, limit);
        register_getters!(
            SagittariusStarPicture,
            picture_index,
            is_in,
            progress,
            finish
        );
        register_getters!(
            SagittariusRewardItem,
            reward_id,
            reward_kind,
            raw_reward_type,
            count
        );
        register_getters!(
            SagittariusBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        register_sagittarius_cgi_info!(SagittariusFirstInfo, "SagittariusFirstInfo");
        register_sagittarius_cgi_info!(SagittariusSecondInfo, "SagittariusSecondInfo");
        register_sagittarius_cgi_info!(SagittariusThirdInfo, "SagittariusThirdInfo");
        register_getters!(ScorpioField, name, value);
        register_getters!(ScorpioCounter, name, current, limit);
        register_getters!(ScorpioReward, reward_index, reward_type, reward_count);
        register_getters!(
            ScorpioBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        register_scorpio_cgi_info!(ScorpioFirstInfo, "ScorpioFirstInfo");
        register_scorpio_cgi_info!(ScorpioSecondInfo, "ScorpioSecondInfo");
        register_scorpio_cgi_info!(ScorpioThirdInfo, "ScorpioThirdInfo");
        register_getters!(AriesField, name, value);
        register_getters!(AriesCounter, name, current, limit);
        register_getters!(AriesReward, reward_index, reward_type, reward_count);
        register_getters!(
            AriesBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        macro_rules! register_aries_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("rewards", |value: &mut $ty| Self::to_array(&value.rewards));
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        register_aries_info!(AriesFirstInfo, "AriesFirstInfo");
        register_aries_info!(AriesSecondInfo, "AriesSecondInfo");
        register_aries_info!(AriesThirdInfo, "AriesThirdInfo");
        register_getters!(
            AriesThirdStatusInfo,
            result_code,
            message,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
            boss_left_hp,
            left_fight_count,
        );
        register_getters!(
            AriesThirdExchangeInfo,
            result_code,
            message,
            item,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(LibraField, name, value);
        register_getters!(LibraCounter, name, current, limit);
        register_getters!(
            LibraBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        register_libra_cgi_info!(LibraFirstInfo, "LibraFirstInfo");
        register_libra_cgi_info!(LibraSecondInfo, "LibraSecondInfo");
        register_libra_cgi_info!(LibraThirdInfo, "LibraThirdInfo");
        register_getters!(
            LibraThirdStatusInfo,
            result_code,
            message,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
            boss_left_hp,
            left_fight_count,
        );
        register_getters!(
            LibraThirdExchangeInfo,
            result_code,
            message,
            item,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(LeoField, name, value);
        register_getters!(LeoCounter, name, current, limit);
        register_getters!(
            LeoBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );

        macro_rules! register_leo_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
            };
        }
        register_leo_info!(LeoFirstInfo, "LeoFirstInfo");
        register_leo_info!(LeoSecondInfo, "LeoSecondInfo");
        register_leo_info!(LeoThirdInfo, "LeoThirdInfo");
        register_getters!(
            LeoFirstStatusInfo,
            result_code,
            message,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
            boss_left_hp,
            left_fight_count,
        );
        register_getters!(
            LeoFirstExchangeInfo,
            result_code,
            message,
            item,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(AquariusField, name, value);
        register_getters!(AquariusCounter, name, current, limit);
        register_getters!(
            AquariusBagCandidate,
            candidate_index,
            spirit_id,
            bag_index,
            catch_time,
            level,
            need_money,
        );
        register_getters!(AquariusRewardItem, item_index, item_id, count, item_type);

        macro_rules! register_aquarius_info {
            ($ty:ty, $name:literal) => {
                engine.register_type_with_name::<$ty>($name);
                register_getters!($ty, result_code, message, request_context);
                engine.register_get("fields", |value: &mut $ty| Self::to_array(&value.fields));
                engine.register_get("counters", |value: &mut $ty| {
                    Self::to_array(&value.counters)
                });
                engine.register_get("item_counts", |value: &mut $ty| {
                    Self::to_array(&value.item_counts)
                });
                engine.register_get("states", |value: &mut $ty| Self::to_array(&value.states));
                engine.register_get("bag_candidates", |value: &mut $ty| {
                    Self::to_array(&value.bag_candidates)
                });
                engine.register_get("reward_items", |value: &mut $ty| {
                    Self::to_array(&value.reward_items)
                });
            };
        }
        register_aquarius_info!(AquariusFirstInfo, "AquariusFirstInfo");
        register_aquarius_info!(AquariusSecondInfo, "AquariusSecondInfo");
        register_aquarius_info!(AquariusThirdInfo, "AquariusThirdInfo");
        register_getters!(
            AquariusSecondStatusInfo,
            result_code,
            message,
            light_num,
            tail_num,
            boss_left_hp,
            boss_full_hp,
            left_fight_count,
            add_hit_level,
            today_sum_hit,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(
            AquariusSecondExchangeInfo,
            result_code,
            message,
            item,
            light_num,
            tail_num,
            exchange_count0,
            exchange_count1,
        );
        register_getters!(LadderSpiritInfo, pet_id, pet_level, now_hp, full_hp, skin,);
        engine.register_get("equipment_ids", |value: &mut LadderSpiritInfo| {
            Self::to_array(&value.equipment_ids)
        });
        register_getters!(LadderQuestInfo, status, id, give_up);
        register_getters!(
            LadderFightRecord,
            win,
            score,
            round,
            my_point,
            other_point,
            fight_type,
        );
        engine.register_get("my_spirits", |value: &mut LadderFightRecord| {
            Self::to_array(&value.my_spirits)
        });
        engine.register_get("other_spirits", |value: &mut LadderFightRecord| {
            Self::to_array(&value.other_spirits)
        });
        register_getters!(
            LadderInfo,
            win_nums,
            win_point,
            spirit_info_flag,
            left_time,
            rank_level,
            left_play_times,
            left_reward_times,
            season_reward_flag,
            fight_days,
            next_win_point,
            show_achievement,
            season,
            all_nums,
            left_play_times_df,
            win_point_df,
            win_nums_df,
            all_nums_df,
        );
        engine.register_get("spirits", |value: &mut LadderInfo| {
            Self::to_array(&value.spirits)
        });
        engine.register_get("backup_spirits", |value: &mut LadderInfo| {
            Self::to_array(&value.backup_spirits)
        });
        engine.register_get("day_quests", |value: &mut LadderInfo| {
            Self::to_array(&value.day_quests)
        });
        engine.register_get("achievement_list", |value: &mut LadderInfo| {
            Self::to_array(&value.achievement_list)
        });
        engine.register_get("ban_list", |value: &mut LadderInfo| {
            Self::to_array(&value.ban_list)
        });
        engine.register_get("records", |value: &mut LadderInfo| {
            Self::to_array(&value.records)
        });
        register_getters!(
            LadderRankUser,
            uin,
            name,
            win_nums,
            win_point,
            rank_num,
            achievement_num,
            show_achievement,
            rank_level,
        );
        engine.register_get("medals", |value: &mut LadderRankUser| {
            Self::to_array(&value.medals)
        });
        register_getters!(LadderRankInfo, rank_level, rank_change);
        engine.register_get("users", |value: &mut LadderRankInfo| {
            Self::to_array(&value.users)
        });
        register_getters!(TypeLadderRank, rank, small_rank, star);
        register_getters!(
            TypeLadderSpiritInfo,
            spirit_id,
            level,
            current_hp,
            max_hp,
            attribute,
            eligibility,
            eligibility_code,
            skin,
        );
        register_getters!(TypeLadderFightRecord, win, round);
        engine.register_get("my_spirits", |value: &mut TypeLadderFightRecord| {
            Self::to_array(&value.my_spirits)
        });
        engine.register_get("opponent_spirits", |value: &mut TypeLadderFightRecord| {
            Self::to_array(&value.opponent_spirits)
        });
        register_getters!(
            TypeLadderInfo,
            season,
            win_count,
            battle_count,
            left_play_times,
            proxy,
            grade,
            current_rank,
            max_rank,
            season_reward_available,
            season_reward_flag,
        );
        engine.register_get("allowed_attributes", |value: &mut TypeLadderInfo| {
            Self::to_array(&value.allowed_attributes)
        });
        engine.register_get("banned_spirit_ids", |value: &mut TypeLadderInfo| {
            Self::to_array(&value.banned_spirit_ids)
        });
        engine.register_get("spirits", |value: &mut TypeLadderInfo| {
            Self::to_array(&value.spirits)
        });
        engine.register_get("records", |value: &mut TypeLadderInfo| {
            Self::to_array(&value.records)
        });
        register_getters!(
            TypeLadderRankUser,
            uin,
            name,
            win_count,
            battle_count,
            rank_num,
            score,
        );
        engine.register_get("my_info", |value: &mut TypeLadderRankInfo| {
            value.my_info.clone()
        });
        engine.register_get("users", |value: &mut TypeLadderRankInfo| {
            Self::to_array(&value.users)
        });
        register_getters!(BloodGiftItemRequirement, item_id, count, need);
        register_getters!(
            BloodGiftOption,
            blood_index,
            talent_type,
            talent_name,
            talent_description,
            awakened,
        );
        engine.register_get("required_items", |value: &mut BloodGiftOption| {
            Self::to_array(&value.required_items)
        });
        register_getters!(
            BloodGiftInfo,
            result_code,
            message,
            position,
            equipped_index
        );
        engine.register_get("options", |value: &mut BloodGiftInfo| {
            Self::to_array(&value.options)
        });
        register_getters!(
            AmendNatureCandidate,
            spirit_id,
            catch_time,
            level,
            personality,
            personality_name,
            need_money,
        );
        register_getters!(
            AmendNatureInfo,
            result_code,
            message,
            new_personality,
            new_personality_name,
        );
        engine.register_get("eligible_spirit_ids", |value: &mut AmendNatureInfo| {
            Self::to_array(&value.eligible_spirit_ids)
        });
        engine.register_get("candidates", |value: &mut AmendNatureInfo| {
            Self::to_array(&value.candidates)
        });
        register_getters!(
            SpiritEquipmentInfo,
            server_id,
            catch_time,
            base_attr,
            base_value,
            special_attr,
            special_value,
            spirit_id,
            spirit_catch_time,
        );
        register_getters!(SpiritEquipmentBagInfo, equipment_count, all_num, need);
        engine.register_get("equipments", |value: &mut SpiritEquipmentBagInfo| {
            Self::to_array(&value.equipments)
        });
        register_getters!(
            ManorGroundInfo,
            ground_id,
            ground_status,
            seed,
            plant_status,
            current_time,
            total_time,
            total_produce,
            left_produce,
            has_grass,
            has_insect,
            has_fruit,
            season,
            left_row_times,
        );
        register_getters!(
            ManorInfo,
            qq_uin,
            manor_level,
            manor_exp,
            gold_mass_num,
            gold_money_num,
            guide_type,
            pet_status,
            scarecrow_exp,
            scarecrow_level,
            scarecrow_id,
            home_id,
            parasol_id,
            beautify_id,
            billboard_id,
            scarecrow_ever_play,
            scarecrow_next_exp,
            scarecrow_gift_gotten,
            proficiency_a,
            proficiency_a_exp,
            proficiency_a_exp_pre,
            proficiency_a_exp_next,
            proficiency_b,
            proficiency_b_exp,
            proficiency_b_exp_pre,
            proficiency_b_exp_next,
            proficiency_c,
            proficiency_c_exp,
            proficiency_c_exp_pre,
            proficiency_c_exp_next,
            steal_state,
        );
        engine.register_get("gift_status_a", |value: &mut ManorInfo| {
            Self::to_array(&value.gift_status_a)
        });
        engine.register_get("gift_status_b", |value: &mut ManorInfo| {
            Self::to_array(&value.gift_status_b)
        });
        engine.register_get("gift_status_c", |value: &mut ManorInfo| {
            Self::to_array(&value.gift_status_c)
        });
        engine.register_get("grounds", |value: &mut ManorInfo| {
            Self::to_array(&value.grounds)
        });
        register_getters!(ManorItemCount, item_id, item_count);
        register_getters!(ManorRewardInfo, item_id, count);
        register_getters!(ManorSowResult, exp);
        engine.register_get("ground", |value: &mut ManorSowResult| value.ground.clone());
        register_getters!(
            ManorReapResult,
            qq_uin,
            seed_id,
            result,
            exp,
            fruit_num,
            event_id,
        );
        engine.register_get("ground", |value: &mut ManorReapResult| value.ground.clone());
        engine.register_get("rewards", |value: &mut ManorReapResult| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("ground", |value: &mut ManorUprootResult| {
            value.ground.clone()
        });
        register_getters!(ManorWeedResult, qq_uin, exp);
        engine.register_get("ground", |value: &mut ManorWeedResult| value.ground.clone());
        register_getters!(
            ManorFertilizerResult,
            can_fertilizer,
            deduce_time_in_second,
            fertilizer,
            uin,
        );
        engine.register_get("ground", |value: &mut ManorFertilizerResult| {
            value.ground.clone()
        });
        register_getters!(
            StaticItemInfo,
            id,
            name,
            description,
            unique,
            item_type,
            subtype,
            price,
            expire_time,
        );
        register_getters!(
            StaticStriveItemInfo,
            id,
            name,
            item_type,
            ghp,
            gpa,
            gpd,
            gma,
            gmd,
            gsp,
            src,
        );

        register_getters!(
            StaticGuardianPetPropertyInfo,
            level,
            phase,
            energy,
            attack,
            defend,
            magic_attack,
            magic_defend,
            need_level_to_next_phase,
        );
        register_getters!(StaticTitleInfo, id, title_name);
        register_getters!(
            StaticMagicInfo,
            id,
            name,
            item_id,
            target,
            magic_type,
            duration,
            action_type,
            app,
            description,
        );
        register_getters!(
            StaticPluginInfo,
            name,
            label,
            domain,
            version,
            command_type,
            plugin_class,
            plugin_src,
            plugin_url,
        );
        register_getters!(LadderQuestConfigEntry, id, diff, description);
        register_getters!(LadderSpiritCostEntry, spirit_id, cost);
        register_getters!(LadderMatchConfig, error);
        engine.register_get("match_rewards", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.match_rewards)
        });
        engine.register_get("win_rewards", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.win_rewards)
        });
        engine.register_get("season_rewards", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.season_rewards)
        });
        engine.register_get("task0_descriptions", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.task0_descriptions)
        });
        engine.register_get("task1_descriptions", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.task1_descriptions)
        });
        engine.register_get("spirit_costs", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.spirit_costs)
        });
        engine.register_get("limit_spirits", |value: &mut LadderMatchConfig| {
            Self::to_array(&value.limit_spirits)
        });
        register_getters!(
            StaticSkillInfo,
            id,
            name,
            description,
            description2,
            power,
            pp_max,
            property,
            src,
            attack_type,
            speed,
            damage_type,
            catch_rate,
            super_form_id,
            super_form_src,
        );
        register_getters!(SpiritBookEntry, id, starred, unknown, newed);
        register_getters!(SpiritBookGroup, template_id);
        engine.register_get("spirits", |value: &mut SpiritBookGroup| {
            Self::to_array(&value.spirits)
        });
        register_getters!(
            SpiritBookSummary,
            id,
            name,
            is_new,
            has_cover,
            background,
            page_idx,
            spirit_count,
        );
        register_getters!(
            SpiritBookInfo,
            id,
            name,
            is_new,
            has_cover,
            background,
            page_idx,
        );
        engine.register_get("groups", |value: &mut SpiritBookInfo| {
            Self::to_array(&value.groups)
        });
        register_getters!(SpiritBookStates, uin, count);
        engine.register_get("states", |value: &mut SpiritBookStates| {
            Self::to_array(&value.states)
        });
        engine.register_fn(
            "is_owned",
            |value: &mut SpiritBookStates, spirit_id: i64| value.spirit_owned(spirit_id),
        );
        register_getters!(SpiritBookSpiritState, spirit_id, state, owned);
        register_getters!(
            StaticSpiritInfo,
            id,
            name,
            description,
            src,
            avatar,
            icon_src,
            preview_src,
            move_speed,
            height,
            weight,
            color,
            interest,
            habitat,
            catchrate,
            boss_phyle,
            boss_reward,
            scene_id,
            condition,
            require_level,
            wg,
            mg,
            mk,
            sm,
            sd,
            fy,
            reward,
            evolution_form_id,
            get_form,
            state,
            start_time,
            end_time,
            first_id,
            propo_level,
            is_in_book,
            skinnum,
            exp_type,
        );
        engine.register_get("features", |value: &mut StaticSpiritInfo| {
            Self::to_array(&value.features)
        });
        engine.register_get("group", |value: &mut StaticSpiritInfo| {
            Self::to_array(&value.group)
        });
        engine.register_get("evolution", |value: &mut StaticSpiritInfo| {
            Self::to_array(&value.evolution)
        });
        engine.register_get("evolution_to_ids", |value: &mut StaticSpiritInfo| {
            Self::to_array(&value.evolution_to_ids)
        });
        register_getters!(StaticSpiritEvolutionEdge, target_id, kind);
        engine.register_get("evolution_edges", |value: &mut StaticSpiritInfo| {
            Self::to_array(&value.evolution_edges)
        });
        register_getters!(StaticSpiritInfoLookupResult, ok, code, message);
        engine.register_get("result", |value: &mut StaticSpiritInfoLookupResult| {
            value.result.clone()
        });
    }
}
