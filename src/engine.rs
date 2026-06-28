//! RocoEngine - Rhai engine wrapper and stdlib registration

use rhai::{Array, Dynamic, Engine, AST};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::debugger::{
    dynamic_preview, RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent,
    RocoDebugHooks, RocoDebugLocalVariable, RocoDebugStackFrame,
};
use crate::error::{
    Result, RocoError, RocoErrorInfo, RocoScriptError, RocoScriptErrorKind, RocoScriptLocation,
    RocoScriptPosition,
};
use crate::stdlib::{
    alchemy_furnace, aquarius, aries, cancer, capricorn, combat, combat_result, combat_status,
    dark_city, diamond_tear, evolution_edge_kind, four_seasons, game, gemini, ice_crystal, ladder,
    leo, libra, lookup, magic_pioneer, manor, mountain_sea, multi_evolution, mystery_fusion, news,
    news_times, personality, pet_training, pisces, play_guide, profile, role, sagittarius, scene,
    scorpio, sentinel_intelligence, session, spirit, spirit_book, spirit_book_state, star_tower,
    summon, system, taurus, three_starters, treasure_realm, type_ladder, unicorn, virgo, weather,
    RocoStdLib,
};
use crate::types::{
    ActionResult, AlchemyFurnaceBagCandidate, AlchemyFurnaceRewardItem, AmendNatureCandidate,
    AmendNatureInfo, AquariusBagCandidate, AquariusCounter, AquariusField, AquariusFirstInfo,
    AquariusRewardItem, AquariusSecondExchangeInfo, AquariusSecondInfo, AquariusSecondStatusInfo,
    AquariusThirdInfo, AriesBagCandidate, AriesCounter, AriesField, AriesFirstInfo, AriesReward,
    AriesSecondInfo, AriesThirdExchangeInfo, AriesThirdInfo, AriesThirdStatusInfo, BagItemInfo,
    BatheSunInfo, BattleCapturedSpirit, BattleResult, BattleResultQueryResult, BattleSpiritResult,
    BloodGiftInfo, BloodGiftItemRequirement, BloodGiftOption, CancerItemInfo,
    CancerMendShapeBagInfo, CancerMendShapeInfo, CancerPetInfo, CancerSharpScorpionInfo,
    CancerUnsealMemoriesBagInfo, CancerUnsealMemoriesInfo, CapricornBagCandidate,
    CapricornInviteListInfo, CapricornPalaceNoteItem, CapricornPalaceNotesInfo,
    CapricornSecondInfo, CapricornSecondTask, CapricornStarPalaceInfo, CapricornTeamOperationInfo,
    CapricornTeamPlayer, CapricornTeamSnapshot, CapricornThirdInfo, CombatActionResult,
    CombatActionSnapshot, CombatActions, CombatSideState, CombatSpiritState, CombatState,
    DarkCityExchangeItem, DarkCityExpeditionInfo, DarkCityReputationInfo, DiamondProgressReward,
    DiamondTaskInfo, DiamondTaskProgress, DiamondTearInfo, DiamondTearRewardItem, FiresWillInfo,
    FourSeasonsInfo, FourSeasonsMonthlySpiritRewardInfo, FourSeasonsRewardItem,
    FourSeasonsShopRewardInfo, GeminiBagCandidate, GeminiCounter, GeminiField, GeminiFirstInfo,
    GeminiRewardItem, GeminiSecondInfo, GeminiThirdInfo, IceCrystalBagCandidate,
    IceCrystalBattleInfo, IceCrystalInfo, IceCrystalRewardItem, LadderFightRecord, LadderInfo,
    LadderMatchConfig, LadderQuestConfigEntry, LadderQuestInfo, LadderRankInfo, LadderRankUser,
    LadderSpiritCostEntry, LadderSpiritInfo, LeoBagCandidate, LeoCounter, LeoField,
    LeoFirstExchangeInfo, LeoFirstInfo, LeoFirstStatusInfo, LeoSecondInfo, LeoThirdInfo,
    LibraBagCandidate, LibraCounter, LibraField, LibraFirstInfo, LibraSecondInfo,
    LibraThirdExchangeInfo, LibraThirdInfo, LibraThirdStatusInfo, MagicPioneerField,
    MagicPioneerInfo, MagicPioneerRewardItem, ManorFertilizerResult, ManorGroundInfo, ManorInfo,
    ManorItemCount, ManorReapResult, ManorRewardInfo, ManorSowResult, ManorUprootResult,
    ManorWeedResult, MiniGameExtraField, MiniGameRewardItem, MiniGameSubmitResult,
    MiniGameSubmitTryResult, MonkeyCultivationInfo, MonkeyEvoInfo, MountainSeaBossInfo,
    MountainSeaInfo, MountainSeaSoulInfo, MultiEvolutionCandidate, MultiEvolutionInfo,
    MultiEvolutionRewardItem, MysteryFusionBattleInfo, MysteryFusionInfo, MysteryFusionMaterialBag,
    MysteryFusionMaterialCandidate, MysteryFusionRecipeInfo, NewsActiveItem, NewsTimesReport,
    NewsTimesReportsResult, PetTrainingResult, PetTrainingRewardItem, PiscesBagCandidate,
    PiscesCounter, PiscesField, PiscesFirstInfo, PiscesSecondInfo, PiscesThirdInfo,
    PlayGuideRewardItem, QqGameHallGiftInfo, RagingFireInfo, SagittariusBagCandidate,
    SagittariusCounter, SagittariusField, SagittariusFirstInfo, SagittariusRewardItem,
    SagittariusScore, SagittariusSecondInfo, SagittariusStarPicture, SagittariusThirdInfo,
    SceneRoleInfo, SceneSpiritInfo, ScorpioBagCandidate, ScorpioCounter, ScorpioField,
    ScorpioFirstInfo, ScorpioReward, ScorpioSecondInfo, ScorpioThirdInfo, SentinelBossInfo,
    SentinelExchangeInfo, SentinelIntelligenceInfo, SentinelSpiritExchangeInfo, ServerTimeInfo,
    ServerTimeResult, SkillPoolInfo, SkillPoolSkillInfo, SkillStoneResult, SkillStoneSkillInfo,
    SkillSwitchResult, SpiritBagInfo, SpiritBookEntry, SpiritBookGroup, SpiritBookInfo,
    SpiritBookSpiritState, SpiritBookStates, SpiritBookSummary, SpiritEquipmentBagInfo,
    SpiritEquipmentInfo, SpiritInfo, SpiritSkillInfo, StarTowerInfo, StarTowerNode,
    StarTowerStorey, StarTowerTop, StarTowerTopMission, StarTowerTopReward,
    StaticGuardianPetPropertyInfo, StaticItemInfo, StaticMagicInfo, StaticPluginInfo,
    StaticSkillInfo, StaticSpiritEvolutionEdge, StaticSpiritInfo, StaticSpiritInfoLookupResult,
    StaticStriveItemInfo, StaticTitleInfo, StorageSpiritInfo, SummonExchangeGroup,
    SummonExchangeItem, SummonInfo, SummonPoolConfig, SummonPoolState, SummonRecord,
    SummonRewardItem, TalentRefreshResult, TaurusBagCandidate, TaurusCounter, TaurusField,
    TaurusFirstInfo, TaurusSecondInfo, TaurusThirdInfo, ThreeStartersBagCandidate,
    ThreeStartersCounter, ThreeStartersField, ThreeStartersRewardItem, TreasureRealmInfo,
    TypeLadderFightRecord, TypeLadderInfo, TypeLadderRank, TypeLadderRankInfo, TypeLadderRankUser,
    TypeLadderSpiritInfo, UnicornBagCandidate, UnicornBossInfo, UnicornInfo, UnicornRewardItem,
    UserInfo, VirgoBellFoxExchangeInfo, VirgoBellFoxInfo, VirgoBellFoxStatusInfo, VirgoCounter,
    VirgoField, VirgoFindHalidomInfo, VirgoPetInfo, VirgoServeGodInfo, WaterSourceInfo,
    WeekTaskActivity, WeekTaskInfo,
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

        let mut scene_module = rhai::Module::new();
        scene::register(&mut scene_module, stdlib.clone());
        engine.register_static_module("scene", scene_module.into());

        let mut session_module = rhai::Module::new();
        session::register(&mut session_module, stdlib.clone());
        engine.register_static_module("session", session_module.into());

        let mut profile_module = rhai::Module::new();
        profile::register(&mut profile_module, stdlib.clone());
        engine.register_static_module("profile", profile_module.into());

        let mut role_module = rhai::Module::new();
        role::register(&mut role_module, stdlib.clone());
        engine.register_static_module("role", role_module.into());

        let mut game_module = rhai::Module::new();
        game::register(&mut game_module, stdlib.clone());
        engine.register_static_module("game", game_module.into());

        let mut ladder_module = rhai::Module::new();
        ladder::register(&mut ladder_module, stdlib.clone());
        engine.register_static_module("ladder", ladder_module.into());

        let mut type_ladder_module = rhai::Module::new();
        type_ladder::register(&mut type_ladder_module, stdlib.clone());
        engine.register_static_module("type_ladder", type_ladder_module.into());

        let mut spirit_module = rhai::Module::new();
        spirit::register(&mut spirit_module, stdlib.clone());
        engine.register_static_module("spirit", spirit_module.into());

        let mut spirit_book_module = rhai::Module::new();
        spirit_book::register(&mut spirit_book_module, stdlib.clone());
        engine.register_static_module("spirit_book", spirit_book_module.into());

        let mut spirit_book_state_module = rhai::Module::new();
        spirit_book_state::register(&mut spirit_book_state_module);
        engine.register_static_module("spirit_book_state", spirit_book_state_module.into());

        let mut evolution_edge_kind_module = rhai::Module::new();
        evolution_edge_kind::register(&mut evolution_edge_kind_module);
        engine.register_static_module("evolution_edge_kind", evolution_edge_kind_module.into());

        let mut personality_module = rhai::Module::new();
        personality::register(&mut personality_module);
        engine.register_static_module("personality", personality_module.into());

        let mut weather_module = rhai::Module::new();
        weather::register(&mut weather_module);
        engine.register_static_module("weather", weather_module.into());

        let mut combat_status_module = rhai::Module::new();
        combat_status::register(&mut combat_status_module);
        engine.register_static_module("combat_status", combat_status_module.into());

        let mut combat_result_module = rhai::Module::new();
        combat_result::register(&mut combat_result_module);
        engine.register_static_module("combat_result", combat_result_module.into());

        let mut manor_module = rhai::Module::new();
        manor::register(&mut manor_module, stdlib.clone());
        engine.register_static_module("manor", manor_module.into());

        let mut pet_training_module = rhai::Module::new();
        pet_training::register(&mut pet_training_module, stdlib.clone());
        engine.register_static_module("pet_training", pet_training_module.into());

        let mut news_module = rhai::Module::new();
        news::register(&mut news_module, stdlib.clone());
        engine.register_static_module("news", news_module.into());

        let mut news_times_module = rhai::Module::new();
        news_times::register(&mut news_times_module, stdlib.clone());
        engine.register_static_module("news_times", news_times_module.into());

        let mut star_tower_module = rhai::Module::new();
        star_tower::register(&mut star_tower_module, stdlib.clone());
        engine.register_static_module("star_tower", star_tower_module.into());

        let mut sentinel_intelligence_module = rhai::Module::new();
        sentinel_intelligence::register(&mut sentinel_intelligence_module, stdlib.clone());
        engine.register_static_module("sentinel_intelligence", sentinel_intelligence_module.into());

        let mut mountain_sea_module = rhai::Module::new();
        mountain_sea::register(&mut mountain_sea_module, stdlib.clone());
        engine.register_static_module("mountain_sea", mountain_sea_module.into());

        let mut magic_pioneer_module = rhai::Module::new();
        magic_pioneer::register(&mut magic_pioneer_module, stdlib.clone());
        engine.register_static_module("magic_pioneer", magic_pioneer_module.into());

        let mut alchemy_furnace_module = rhai::Module::new();
        alchemy_furnace::register(&mut alchemy_furnace_module, stdlib.clone());
        engine.register_static_module("alchemy_furnace", alchemy_furnace_module.into());

        let mut unicorn_module = rhai::Module::new();
        unicorn::register(&mut unicorn_module, stdlib.clone());
        engine.register_static_module("unicorn", unicorn_module.into());

        let mut four_seasons_module = rhai::Module::new();
        four_seasons::register(&mut four_seasons_module, stdlib.clone());
        engine.register_static_module("four_seasons", four_seasons_module.into());

        let mut ice_crystal_module = rhai::Module::new();
        ice_crystal::register(&mut ice_crystal_module, stdlib.clone());
        engine.register_static_module("ice_crystal", ice_crystal_module.into());

        let mut multi_evolution_module = rhai::Module::new();
        multi_evolution::register(&mut multi_evolution_module, stdlib.clone());
        engine.register_static_module("multi_evolution", multi_evolution_module.into());

        let mut diamond_tear_module = rhai::Module::new();
        diamond_tear::register(&mut diamond_tear_module, stdlib.clone());
        engine.register_static_module("diamond_tear", diamond_tear_module.into());

        let mut dark_city_module = rhai::Module::new();
        dark_city::register(&mut dark_city_module, stdlib.clone());
        engine.register_static_module("dark_city", dark_city_module.into());

        let mut mystery_fusion_module = rhai::Module::new();
        mystery_fusion::register(&mut mystery_fusion_module, stdlib.clone());
        engine.register_static_module("mystery_fusion", mystery_fusion_module.into());

        let mut treasure_realm_module = rhai::Module::new();
        treasure_realm::register(&mut treasure_realm_module, stdlib.clone());
        engine.register_static_module("treasure_realm", treasure_realm_module.into());

        let mut summon_module = rhai::Module::new();
        summon::register(&mut summon_module, stdlib.clone());
        engine.register_static_module("summon", summon_module.into());

        let mut play_guide_module = rhai::Module::new();
        play_guide::register(&mut play_guide_module, stdlib.clone());
        engine.register_static_module("play_guide", play_guide_module.into());

        let mut capricorn_module = rhai::Module::new();
        capricorn::register(&mut capricorn_module, stdlib.clone());
        engine.register_static_module("capricorn", capricorn_module.into());

        let mut cancer_module = rhai::Module::new();
        cancer::register(&mut cancer_module, stdlib.clone());
        engine.register_static_module("cancer", cancer_module.into());

        let mut virgo_module = rhai::Module::new();
        virgo::register(&mut virgo_module, stdlib.clone());
        engine.register_static_module("virgo", virgo_module.into());

        let mut pisces_module = rhai::Module::new();
        pisces::register(&mut pisces_module, stdlib.clone());
        engine.register_static_module("pisces", pisces_module.into());

        let mut taurus_module = rhai::Module::new();
        taurus::register(&mut taurus_module, stdlib.clone());
        engine.register_static_module("taurus", taurus_module.into());

        let mut three_starters_module = rhai::Module::new();
        three_starters::register(&mut three_starters_module, stdlib.clone());
        engine.register_static_module("three_starters", three_starters_module.into());

        let mut gemini_module = rhai::Module::new();
        gemini::register(&mut gemini_module, stdlib.clone());
        engine.register_static_module("gemini", gemini_module.into());

        let mut sagittarius_module = rhai::Module::new();
        sagittarius::register(&mut sagittarius_module, stdlib.clone());
        engine.register_static_module("sagittarius", sagittarius_module.into());

        let mut aquarius_module = rhai::Module::new();
        aquarius::register(&mut aquarius_module, stdlib.clone());
        engine.register_static_module("aquarius", aquarius_module.into());

        let mut aries_module = rhai::Module::new();
        aries::register(&mut aries_module, stdlib.clone());
        engine.register_static_module("aries", aries_module.into());

        let mut scorpio_module = rhai::Module::new();
        scorpio::register(&mut scorpio_module, stdlib.clone());
        engine.register_static_module("scorpio", scorpio_module.into());

        let mut libra_module = rhai::Module::new();
        libra::register(&mut libra_module, stdlib.clone());
        engine.register_static_module("libra", libra_module.into());

        let mut leo_module = rhai::Module::new();
        leo::register(&mut leo_module, stdlib.clone());
        engine.register_static_module("leo", leo_module.into());

        let mut lookup_module = rhai::Module::new();
        lookup::register(&mut lookup_module, stdlib.clone());
        engine.register_static_module("lookup", lookup_module.into());

        let mut combat_module = rhai::Module::new();
        combat::register(&mut combat_module, stdlib.clone());
        engine.register_static_module("combat", combat_module.into());

        let mut system_module = rhai::Module::new();
        system::register(&mut system_module, stdlib.clone());
        engine.register_static_module("system", system_module.into());

        let mut global_system_module = rhai::Module::new();
        system::register_functions(&mut global_system_module, stdlib);
        engine.register_global_module(global_system_module.into());
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

    fn map_eval_error(error: Box<rhai::EvalAltResult>) -> RocoError {
        let position = error.position();
        let kind = match error.as_ref() {
            rhai::EvalAltResult::ErrorParsing(..) => RocoScriptErrorKind::Parse,
            rhai::EvalAltResult::ErrorTerminated(..) => RocoScriptErrorKind::Terminated,
            rhai::EvalAltResult::ErrorRuntime(..) => RocoScriptErrorKind::Runtime,
            rhai::EvalAltResult::ErrorInFunctionCall(..) => RocoScriptErrorKind::FunctionCall,
            rhai::EvalAltResult::ErrorInModule(..) => RocoScriptErrorKind::Module,
            _ => RocoScriptErrorKind::Eval,
        };
        let source = match error.as_ref() {
            rhai::EvalAltResult::ErrorInFunctionCall(_, source, ..) if !source.is_empty() => {
                Some(source.clone())
            }
            _ => None,
        };
        RocoError::ScriptError(RocoScriptError {
            kind,
            message: error.to_string(),
            location: Self::script_location(source, position),
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
                            rhai::debugger::DebuggerEvent::Start
                            | rhai::debugger::DebuggerEvent::End => unreachable!(),
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
        RocoError::ScriptError(RocoScriptError {
            kind: RocoScriptErrorKind::Parse,
            message: error.to_string(),
            location: Self::script_location(source, position),
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
                                error.to_string().into(),
                                rhai::Position::NONE,
                            ))
                        })
                    },
                );
            };
        }
        for_each_roco_type!(register_roco_type_basics);

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
        register_getters!(
            CombatActions,
            can_submit_action,
            can_use_skill,
            can_capture,
            can_use_item,
            can_change_spirit,
            can_escape,
            can_use_any_skill,
            can_change_to_any_spirit,
        );
        register_getters!(CombatSpiritState, position, spirit_id, level, hp, max_hp);
        engine.register_get("skills", |value: &mut CombatSpiritState| {
            Self::to_array(&value.skills)
        });
        register_getters!(CombatSideState, active_position);
        engine.register_get("spirits", |value: &mut CombatSideState| {
            Self::to_array(&value.spirits)
        });
        register_getters!(CombatState, round, weather, weather_round);
        engine.register_get("my_side", |value: &mut CombatState| value.my_side.clone());
        engine.register_get("rival_side", |value: &mut CombatState| {
            value.rival_side.clone()
        });
        register_getters!(CombatActionSnapshot, is_finished);
        engine.register_get("state", |value: &mut CombatActionSnapshot| {
            value.state.clone()
        });
        engine.register_get("actions", |value: &mut CombatActionSnapshot| {
            value.actions.clone()
        });
        engine.register_type_with_name::<RocoErrorInfo>("RocoErrorInfo");
        register_getters!(RocoErrorInfo, kind, code, message);
        register_getters!(ActionResult, ok, code, message, error);
        register_getters!(
            CombatActionResult,
            ok,
            code,
            message,
            error,
            ack_received,
            combat_finished,
            next_action_ready
        );
        register_getters!(MiniGameRewardItem, id, count, item_type);
        register_getters!(MiniGameExtraField, key, value);
        register_getters!(
            MiniGameSubmitResult,
            ok,
            code,
            message,
            game_id,
            score,
            game_type
        );
        engine.register_get("items", |value: &mut MiniGameSubmitResult| {
            Self::to_array(&value.items)
        });
        engine.register_get("extra_fields", |value: &mut MiniGameSubmitResult| {
            Self::to_array(&value.extra_fields)
        });
        register_getters!(MiniGameSubmitTryResult, ok, code, message, error);
        engine.register_get("result", |value: &mut MiniGameSubmitTryResult| {
            value.result.clone()
        });
        register_getters!(
            TalentRefreshResult,
            position,
            pa_old,
            pd_old,
            ma_old,
            md_old,
            sp_old,
            hp_old,
            pa_new,
            pd_new,
            ma_new,
            md_new,
            sp_new,
            hp_new,
            pa_level_old,
            pd_level_old,
            ma_level_old,
            md_level_old,
            sp_level_old,
            hp_level_old,
            pa_level_new,
            pd_level_new,
            ma_level_new,
            md_level_new,
            sp_level_new,
            hp_level_new,
        );
        register_getters!(
            UserInfo,
            uin,
            id,
            nick_name,
            level,
            is_vip,
            vip_level,
            vip_expiring_days,
            vip_lulu,
            trainer_level,
            trainer_exp,
        );
        register_getters!(
            ServerTimeInfo,
            stamp,
            full_year,
            month,
            date,
            hours,
            minutes,
            seconds,
            day,
            day_of_year,
        );
        register_getters!(ServerTimeResult, ok, code, message, error);
        engine.register_get("result", |value: &mut ServerTimeResult| {
            value.result.clone()
        });
        register_getters!(SpiritInfo, spirit_id, position, catch_time, name, level, hp, max_hp);
        engine.register_get("skills", |value: &mut SpiritInfo| {
            Self::to_array(&value.skills)
        });
        register_getters!(SpiritSkillInfo, skill_id, pp, inherited);
        register_getters!(SkillPoolSkillInfo, skill_id, pp, inherited, position);
        register_getters!(SkillPoolInfo, spirit_id, position);
        engine.register_get("skills", |value: &mut SkillPoolInfo| {
            Self::to_array(&value.skills)
        });
        register_getters!(SkillSwitchResult, spirit_id, position, skill_slot, skill_id);
        register_getters!(SkillStoneSkillInfo, skill_id, pp, inherited);
        register_getters!(
            SkillStoneResult,
            ok,
            result_code,
            message,
            item_id,
            position,
            needs_replace,
        );
        engine.register_get("old_skills", |value: &mut SkillStoneResult| {
            Self::to_array(&value.old_skills)
        });
        engine.register_get("new_skills", |value: &mut SkillStoneResult| {
            Self::to_array(&value.new_skills)
        });
        register_getters!(
            StorageSpiritInfo,
            spirit_id,
            catch_time,
            storage_time,
            level,
            sex,
            skin_flag,
            talent_type,
            talent_level,
        );
        register_getters!(
            SceneSpiritInfo,
            spirit_id,
            count,
            area_index,
            is_rare,
            is_boss,
            is_npc_boss,
        );
        register_getters!(
            SceneRoleInfo,
            uin,
            id,
            nick_name,
            level,
            loc_x,
            loc_y,
            pk_state,
            is_in_combat,
            is_vip,
            vip_level,
            trainer_level,
            trainer_exp,
        );
        register_getters!(BagItemInfo, item_id, count);
        register_getters!(
            BattleSpiritResult,
            position,
            exp,
            level_delta,
            level,
            next_exp,
            effort,
            evolve_spirit_id,
        );
        engine.register_get("new_skill_ids", |value: &mut BattleSpiritResult| {
            Self::to_array(&value.new_skill_ids)
        });
        register_getters!(BattleCapturedSpirit, spirit_id, level, disposition);
        engine.register_get("property_list", |value: &mut BattleCapturedSpirit| {
            Self::to_array(&value.property_list)
        });
        engine.register_get("flair_list", |value: &mut BattleCapturedSpirit| {
            Self::to_array(&value.flair_list)
        });
        register_getters!(
            NewsTimesReport,
            id,
            report_type,
            begin_time,
            end_time,
            name_image_url,
            app_url,
        );
        engine.register_get("act_begin_time", |value: &mut NewsTimesReport| {
            Self::to_array(&value.act_begin_time)
        });
        engine.register_get("act_end_time", |value: &mut NewsTimesReport| {
            Self::to_array(&value.act_end_time)
        });
        register_getters!(NewsTimesReportsResult, gift_gotten);
        engine.register_get("reports", |value: &mut NewsTimesReportsResult| {
            Self::to_array(&value.reports)
        });
        engine.register_get(
            "player_status_today",
            |value: &mut NewsTimesReportsResult| Self::to_array(&value.player_status_today),
        );
        engine.register_get(
            "player_status_forever",
            |value: &mut NewsTimesReportsResult| Self::to_array(&value.player_status_forever),
        );
        register_getters!(
            NewsActiveItem,
            id,
            scene_id,
            npc_x,
            npc_y,
            time,
            content,
            auto_start,
            script_url,
            app_url,
        );
        register_getters!(
            StarTowerNode,
            node_index,
            star,
            spirit_id,
            fight_id,
            item_id,
            reward,
            equip_id,
        );
        register_getters!(StarTowerStorey, storey_index, first, can_quick_fight);
        engine.register_get("nodes", |value: &mut StarTowerStorey| {
            Self::to_array(&value.nodes)
        });
        register_getters!(StarTowerTop, star, refresh, fight_desc, task_desc, fight_id,);
        engine.register_get("tokens", |value: &mut StarTowerTop| {
            Self::to_array(&value.tokens)
        });
        engine.register_get("exchanges", |value: &mut StarTowerTop| {
            Self::to_array(&value.exchanges)
        });
        engine.register_get("missions", |value: &mut StarTowerTop| {
            Self::to_array(&value.missions)
        });
        engine.register_get("rewards", |value: &mut StarTowerTop| {
            Self::to_array(&value.rewards)
        });
        register_getters!(StarTowerTopMission, index, description, completed);
        register_getters!(
            StarTowerTopReward,
            index,
            threshold,
            name,
            amount,
            state,
            claimed,
            claimable,
        );
        register_getters!(
            StarTowerInfo,
            result_code,
            message,
            mop,
            boss_id,
            countdown,
            auto_sell,
            money,
            has_top,
        );
        engine.register_get("clips", |value: &mut StarTowerInfo| {
            Self::to_array(&value.clips)
        });
        engine.register_get("storeys", |value: &mut StarTowerInfo| {
            Self::to_array(&value.storeys)
        });
        engine.register_get("top", |value: &mut StarTowerInfo| value.top.clone());
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
        register_getters!(PlayGuideRewardItem, id, count, item_type);
        register_getters!(WeekTaskActivity, activity_id, reward_count);
        register_getters!(
            WeekTaskInfo,
            result_code,
            message,
            ticket_item_id,
            ticket_count,
        );
        engine.register_get("progress", |value: &mut WeekTaskInfo| {
            Self::to_array(&value.progress)
        });
        engine.register_get("button_states", |value: &mut WeekTaskInfo| {
            Self::to_array(&value.button_states)
        });
        engine.register_get("new_activities", |value: &mut WeekTaskInfo| {
            Self::to_array(&value.new_activities)
        });
        engine.register_get("old_activities", |value: &mut WeekTaskInfo| {
            Self::to_array(&value.old_activities)
        });
        engine.register_get("rewards", |value: &mut WeekTaskInfo| {
            Self::to_array(&value.rewards)
        });
        register_getters!(DiamondTaskProgress, index, current, target, completed,);
        register_getters!(
            DiamondProgressReward,
            index,
            threshold,
            state,
            claimable,
            claimed,
        );
        register_getters!(DiamondTaskInfo, result_code, message, vip, reward_type);
        engine.register_get("tasks", |value: &mut DiamondTaskInfo| {
            Self::to_array(&value.tasks)
        });
        engine.register_get("rewards", |value: &mut DiamondTaskInfo| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("reward_items", |value: &mut DiamondTaskInfo| {
            Self::to_array(&value.reward_items)
        });
        register_getters!(QqGameHallGiftInfo, result_code, message);
        engine.register_get("rewards", |value: &mut QqGameHallGiftInfo| {
            Self::to_array(&value.rewards)
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
        register_getters!(
            CapricornTeamOperationInfo,
            result_code,
            message,
            has_team,
            team,
        );
        register_getters!(CapricornSecondTask, task_type, data1, data2, step, current,);
        register_getters!(
            CapricornBagCandidate,
            candidate_index,
            spirit_id,
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_finish,
            finish,
            has_current,
            current,
            has_position,
            position,
            has_second_task,
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
            has_finish,
            finish,
            has_current,
            current,
            has_remain,
            remain,
            has_price,
            price,
            has_limit,
            limit,
            has_progress_percent,
            progress_percent,
            has_reward_num,
            reward_num,
            has_tips,
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
            has_display_item,
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
            has_item,
            item_id,
            item_count,
            item_type,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
            need_money,
        );
        register_getters!(
            MonkeyCultivationInfo,
            result_code,
            message,
            request_context,
            has_daytimes,
            daytimes,
            has_finish,
            finish,
            has_progress,
            progress,
            has_add_progress,
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
            has_branch_type,
            branch_type,
            has_done,
            done,
            has_schedule,
            schedule,
            has_add_progress,
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
            has_vip,
            vip,
            has_daytimes,
            daytimes,
            has_finish,
            finish,
            has_fusion,
            fusion,
            has_add_progress,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
            need_money,
        );
        register_getters!(
            UnicornBossInfo,
            slot,
            npc_index,
            has_spirit_id,
            spirit_id,
            has_fight_id,
            fight_id
        );
        register_getters!(
            UnicornInfo,
            result_code,
            message,
            request_context,
            has_finish,
            finish,
            has_start,
            start,
            has_total,
            total,
            has_book,
            book,
            has_purple_vine_count,
            purple_vine_count,
            has_energy,
            energy,
            has_fruit_count,
            fruit_count,
            has_increase,
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
            has_month,
            month,
            has_map,
            map,
            has_position_1based,
            position_1based,
            has_times,
            times,
            has_ticket,
            ticket,
            has_used_tool_index,
            used_tool_index,
            has_need_item_index,
            need_item_index,
            has_add,
            add,
            has_point,
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
            has_buy,
            buy,
            has_level,
            level,
            has_count_down,
            count_down,
            has_tear_state,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_progress,
            progress,
            has_battle_times,
            battle_times,
            has_battle_index,
            battle_index,
            has_get_times,
            get_times,
            has_add,
            add,
            has_current_battle,
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
            has_pet_id,
            pet_id,
            has_result_side,
            result_side,
            has_item_id,
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
            has_battle,
            battle,
            has_schedule,
            schedule,
            has_time,
            time,
            has_increase,
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
            has_schedule,
            schedule,
            has_num,
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
            has_battle,
            battle,
            has_schedule,
            schedule,
            has_time,
            time,
            has_num,
            num,
            has_act,
            act,
            has_times,
            times,
            has_sun,
            sun,
            has_add,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_item,
            item_id,
            item_count,
            item_type,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_item,
            item_id,
            item_count,
            item_type,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
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
            has_item,
            item_id,
            item_count,
            item_type,
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
            has_bag_index,
            bag_index,
            has_catch_time,
            catch_time,
            has_level,
            level,
            has_need_money,
            need_money,
        );
        register_getters!(
            AquariusRewardItem,
            item_index,
            item_id,
            count,
            has_item_type,
            item_type
        );

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
            has_item,
            item_id,
            item_count,
            item_type,
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
        engine.register_get("has_my_info", |value: &mut TypeLadderRankInfo| {
            value.my_info.is_some()
        });
        engine.register_get("my_info_or_default", |value: &mut TypeLadderRankInfo| {
            value.my_info.clone().unwrap_or_default()
        });
        engine.register_get("users", |value: &mut TypeLadderRankInfo| {
            Self::to_array(&value.users)
        });
        register_getters!(
            BattleResult,
            winner,
            total_rounds,
            finish_code,
            trainer_exp,
            next_level_trainer_exp,
            honour_point,
            exp_add_bits,
        );
        engine.register_get("obtained_items", |value: &mut BattleResult| {
            Self::to_array(&value.obtained_items)
        });
        engine.register_get("spirit_results", |value: &mut BattleResult| {
            Self::to_array(&value.spirit_results)
        });
        engine.register_get("captured_spirits", |value: &mut BattleResult| {
            Self::to_array(&value.captured_spirits)
        });
        register_getters!(BattleResultQueryResult, ok, code, message, error);
        engine.register_get("result", |value: &mut BattleResultQueryResult| {
            value.result.clone()
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
        register_getters!(PetTrainingRewardItem, item_id, count);
        register_getters!(
            PetTrainingResult,
            ok,
            result_code,
            message,
            training_type,
            pet_id,
            raw_text,
        );
        engine.register_get("rewards", |value: &mut PetTrainingResult| {
            Self::to_array(&value.rewards)
        });
        engine.register_get("spirits", |value: &mut SpiritBagInfo| {
            Self::to_array(&value.spirits)
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
