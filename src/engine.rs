//! RocoEngine - Rhai engine wrapper and stdlib registration

use rhai::{Array, Dynamic, Engine, AST};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::debugger::{
    dynamic_preview, RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent,
    RocoDebugHooks, RocoDebugLocalVariable, RocoDebugStackFrame,
};
use crate::error::{Result, RocoError, RocoScriptError};
use crate::stdlib::{
    combat, dark_city, game, lookup, manor, mountain_sea, news, news_times, profile, role, scene,
    sentinel_intelligence, session, spirit, star_tower, system, RocoStdLib,
};
use crate::types::{
    ActionResult, BagItemInfo, BattleCapturedSpirit, BattleResult, BattleResultQueryResult,
    BattleSpiritResult, BloodGiftInfo, BloodGiftItemRequirement, BloodGiftOption, CombatActions,
    CombatSideState, CombatSpiritState, CombatState, DarkCityExchangeItem, DarkCityExpeditionInfo,
    DarkCityReputationInfo, ManorFertilizerResult, ManorGroundInfo, ManorInfo, ManorItemCount,
    ManorReapResult, ManorRewardInfo, ManorSowResult, ManorUprootResult, ManorWeedResult,
    MountainSeaBossInfo, MountainSeaInfo, MountainSeaSoulInfo, NewsActiveItem, NewsTimesReport,
    NewsTimesReportsResult, SceneRoleInfo, SceneSpiritInfo, SentinelBossInfo, SentinelExchangeInfo,
    SentinelIntelligenceInfo, SentinelSpiritExchangeInfo, SkillPoolInfo, SkillPoolSkillInfo,
    SkillStoneResult, SkillStoneSkillInfo, SkillSwitchResult, SpiritBagInfo,
    SpiritEquipmentBagInfo, SpiritEquipmentInfo, SpiritInfo, SpiritSkillInfo, StarTowerInfo,
    StarTowerNode, StarTowerStorey, StarTowerTop, StarTowerTopMission, StarTowerTopReward,
    StaticGuardianPetPropertyInfo, StaticItemInfo, StaticMagicInfo, StaticPluginInfo,
    StaticSkillInfo, StaticSpiritInfo, StaticStriveItemInfo, StaticTitleInfo, StorageSpiritInfo,
    TalentRefreshResult, UserInfo,
};

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

        let mut spirit_module = rhai::Module::new();
        spirit::register(&mut spirit_module, stdlib.clone());
        engine.register_static_module("spirit", spirit_module.into());

        let mut manor_module = rhai::Module::new();
        manor::register(&mut manor_module, stdlib.clone());
        engine.register_static_module("manor", manor_module.into());

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

        let mut dark_city_module = rhai::Module::new();
        dark_city::register(&mut dark_city_module, stdlib.clone());
        engine.register_static_module("dark_city", dark_city_module.into());

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

    fn map_eval_error(error: Box<rhai::EvalAltResult>) -> RocoError {
        let position = error.position();
        let kind = match error.as_ref() {
            rhai::EvalAltResult::ErrorParsing(..) => "parse",
            rhai::EvalAltResult::ErrorTerminated(..) => "terminated",
            rhai::EvalAltResult::ErrorRuntime(..) => "runtime",
            rhai::EvalAltResult::ErrorInFunctionCall(..) => "function_call",
            rhai::EvalAltResult::ErrorInModule(..) => "module",
            _ => "script",
        }
        .to_string();
        let source = match error.as_ref() {
            rhai::EvalAltResult::ErrorInFunctionCall(_, source, ..) if !source.is_empty() => {
                Some(source.clone())
            }
            _ => None,
        };
        RocoError::ScriptError(RocoScriptError {
            kind,
            message: error.to_string(),
            source,
            line: position.line(),
            column: position.position(),
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
                                source: frame.source.as_ref().map(ToString::to_string),
                                line: frame.pos.line(),
                                column: frame.pos.position(),
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
                            source: source.map(ToString::to_string),
                            line: pos.line(),
                            column: pos.position(),
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
            kind: "parse".to_string(),
            message: error.to_string(),
            source,
            line: position.line(),
            column: position.position(),
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
        macro_rules! register_to_string {
            ($type:ty) => {
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

        engine.register_type_with_name::<CombatActions>("CombatActions");
        register_to_string!(CombatActions);
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

        engine.register_type_with_name::<CombatSpiritState>("CombatSpiritState");
        register_to_string!(CombatSpiritState);
        register_getters!(CombatSpiritState, position, spirit_id, level, hp, max_hp);
        engine.register_get("skills", |value: &mut CombatSpiritState| {
            Self::to_array(&value.skills)
        });

        engine.register_type_with_name::<CombatSideState>("CombatSideState");
        register_to_string!(CombatSideState);
        register_getters!(CombatSideState, active_position);
        engine.register_get("spirits", |value: &mut CombatSideState| {
            Self::to_array(&value.spirits)
        });

        engine.register_type_with_name::<CombatState>("CombatState");
        register_to_string!(CombatState);
        register_getters!(CombatState, round, weather, weather_round);
        engine.register_get("my_side", |value: &mut CombatState| value.my_side.clone());
        engine.register_get("rival_side", |value: &mut CombatState| {
            value.rival_side.clone()
        });

        engine.register_type_with_name::<ActionResult>("ActionResult");
        register_to_string!(ActionResult);
        register_getters!(ActionResult, ok, code, message);

        engine.register_type_with_name::<TalentRefreshResult>("TalentRefreshResult");
        register_to_string!(TalentRefreshResult);
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

        engine.register_type_with_name::<UserInfo>("UserInfo");
        register_to_string!(UserInfo);
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

        engine.register_type_with_name::<SpiritInfo>("SpiritInfo");
        register_to_string!(SpiritInfo);
        register_getters!(SpiritInfo, spirit_id, position, catch_time, name, level, hp, max_hp);
        engine.register_get("skills", |value: &mut SpiritInfo| {
            Self::to_array(&value.skills)
        });

        engine.register_type_with_name::<SpiritSkillInfo>("SpiritSkillInfo");
        register_to_string!(SpiritSkillInfo);
        register_getters!(SpiritSkillInfo, skill_id, pp, inherited);

        engine.register_type_with_name::<SkillPoolSkillInfo>("SkillPoolSkillInfo");
        register_to_string!(SkillPoolSkillInfo);
        register_getters!(SkillPoolSkillInfo, skill_id, pp, inherited, position);

        engine.register_type_with_name::<SkillPoolInfo>("SkillPoolInfo");
        register_to_string!(SkillPoolInfo);
        register_getters!(SkillPoolInfo, spirit_id, position);
        engine.register_get("skills", |value: &mut SkillPoolInfo| {
            Self::to_array(&value.skills)
        });

        engine.register_type_with_name::<SkillSwitchResult>("SkillSwitchResult");
        register_to_string!(SkillSwitchResult);
        register_getters!(SkillSwitchResult, spirit_id, position, skill_slot, skill_id);

        engine.register_type_with_name::<SkillStoneSkillInfo>("SkillStoneSkillInfo");
        register_to_string!(SkillStoneSkillInfo);
        register_getters!(SkillStoneSkillInfo, skill_id, pp, inherited);

        engine.register_type_with_name::<SkillStoneResult>("SkillStoneResult");
        register_to_string!(SkillStoneResult);
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

        engine.register_type_with_name::<StorageSpiritInfo>("StorageSpiritInfo");
        register_to_string!(StorageSpiritInfo);
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

        engine.register_type_with_name::<SceneSpiritInfo>("SceneSpiritInfo");
        register_to_string!(SceneSpiritInfo);
        register_getters!(
            SceneSpiritInfo,
            spirit_id,
            count,
            area_index,
            is_rare,
            is_boss,
            is_npc_boss,
        );

        engine.register_type_with_name::<SceneRoleInfo>("SceneRoleInfo");
        register_to_string!(SceneRoleInfo);
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

        engine.register_type_with_name::<BagItemInfo>("BagItemInfo");
        register_to_string!(BagItemInfo);
        register_getters!(BagItemInfo, item_id, count);

        engine.register_type_with_name::<BattleSpiritResult>("BattleSpiritResult");
        register_to_string!(BattleSpiritResult);
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

        engine.register_type_with_name::<BattleCapturedSpirit>("BattleCapturedSpirit");
        register_to_string!(BattleCapturedSpirit);
        register_getters!(BattleCapturedSpirit, spirit_id, level, disposition);

        engine.register_type_with_name::<NewsTimesReport>("NewsTimesReport");
        register_to_string!(NewsTimesReport);
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
        engine.register_type_with_name::<NewsTimesReportsResult>("NewsTimesReportsResult");
        register_to_string!(NewsTimesReportsResult);
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

        engine.register_type_with_name::<NewsActiveItem>("NewsActiveItem");
        register_to_string!(NewsActiveItem);
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

        engine.register_type_with_name::<StarTowerNode>("StarTowerNode");
        register_to_string!(StarTowerNode);
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

        engine.register_type_with_name::<StarTowerStorey>("StarTowerStorey");
        register_to_string!(StarTowerStorey);
        register_getters!(StarTowerStorey, storey_index, first, can_quick_fight);
        engine.register_get("nodes", |value: &mut StarTowerStorey| {
            Self::to_array(&value.nodes)
        });

        engine.register_type_with_name::<StarTowerTop>("StarTowerTop");
        register_to_string!(StarTowerTop);
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

        engine.register_type_with_name::<StarTowerTopMission>("StarTowerTopMission");
        register_to_string!(StarTowerTopMission);
        register_getters!(StarTowerTopMission, index, description, completed);

        engine.register_type_with_name::<StarTowerTopReward>("StarTowerTopReward");
        register_to_string!(StarTowerTopReward);
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

        engine.register_type_with_name::<StarTowerInfo>("StarTowerInfo");
        register_to_string!(StarTowerInfo);
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

        engine.register_type_with_name::<SentinelBossInfo>("SentinelBossInfo");
        register_to_string!(SentinelBossInfo);
        register_getters!(
            SentinelBossInfo,
            index,
            spirit_id,
            difficulty,
            status,
            max_intelligence,
            intelligence,
        );

        engine.register_type_with_name::<SentinelExchangeInfo>("SentinelExchangeInfo");
        register_to_string!(SentinelExchangeInfo);
        register_getters!(SentinelExchangeInfo, index, item_id, need_bounty, status);

        engine.register_type_with_name::<SentinelSpiritExchangeInfo>("SentinelSpiritExchangeInfo");
        register_to_string!(SentinelSpiritExchangeInfo);
        register_getters!(
            SentinelSpiritExchangeInfo,
            index,
            spirit_id,
            need_intelligence,
            evolve_spirit_id,
            status,
        );

        engine.register_type_with_name::<SentinelIntelligenceInfo>("SentinelIntelligenceInfo");
        register_to_string!(SentinelIntelligenceInfo);
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

        engine.register_type_with_name::<MountainSeaBossInfo>("MountainSeaBossInfo");
        register_to_string!(MountainSeaBossInfo);
        register_getters!(
            MountainSeaBossInfo,
            index,
            boss_type,
            fight_id,
            name,
            status,
        );

        engine.register_type_with_name::<MountainSeaSoulInfo>("MountainSeaSoulInfo");
        register_to_string!(MountainSeaSoulInfo);
        register_getters!(MountainSeaSoulInfo, soul_type, boss_type, name, count);

        engine.register_type_with_name::<MountainSeaInfo>("MountainSeaInfo");
        register_to_string!(MountainSeaInfo);
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

        engine.register_type_with_name::<DarkCityExpeditionInfo>("DarkCityExpeditionInfo");
        register_to_string!(DarkCityExpeditionInfo);
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

        engine.register_type_with_name::<DarkCityExchangeItem>("DarkCityExchangeItem");
        register_to_string!(DarkCityExchangeItem);
        register_getters!(DarkCityExchangeItem, index, item_id, cost);

        engine.register_type_with_name::<DarkCityReputationInfo>("DarkCityReputationInfo");
        register_to_string!(DarkCityReputationInfo);
        register_getters!(DarkCityReputationInfo, result_code, message, reputation);
        engine.register_get("exchanges", |value: &mut DarkCityReputationInfo| {
            Self::to_array(&value.exchanges)
        });

        engine.register_type_with_name::<BattleResult>("BattleResult");
        register_to_string!(BattleResult);
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

        engine.register_type_with_name::<BattleResultQueryResult>("BattleResultQueryResult");
        register_to_string!(BattleResultQueryResult);
        register_getters!(BattleResultQueryResult, ok, code, message);
        engine.register_get("result", |value: &mut BattleResultQueryResult| {
            value.result.clone()
        });

        engine.register_type_with_name::<BloodGiftItemRequirement>("BloodGiftItemRequirement");
        register_to_string!(BloodGiftItemRequirement);
        register_getters!(BloodGiftItemRequirement, item_id, count, need);

        engine.register_type_with_name::<BloodGiftOption>("BloodGiftOption");
        register_to_string!(BloodGiftOption);
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

        engine.register_type_with_name::<BloodGiftInfo>("BloodGiftInfo");
        register_to_string!(BloodGiftInfo);
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

        engine.register_type_with_name::<SpiritEquipmentInfo>("SpiritEquipmentInfo");
        register_to_string!(SpiritEquipmentInfo);
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

        engine.register_type_with_name::<SpiritEquipmentBagInfo>("SpiritEquipmentBagInfo");
        register_to_string!(SpiritEquipmentBagInfo);
        register_getters!(SpiritEquipmentBagInfo, equipment_count, all_num, need);
        engine.register_get("equipments", |value: &mut SpiritEquipmentBagInfo| {
            Self::to_array(&value.equipments)
        });

        engine.register_type_with_name::<ManorGroundInfo>("ManorGroundInfo");
        register_to_string!(ManorGroundInfo);
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

        engine.register_type_with_name::<ManorInfo>("ManorInfo");
        register_to_string!(ManorInfo);
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

        engine.register_type_with_name::<ManorItemCount>("ManorItemCount");
        register_to_string!(ManorItemCount);
        register_getters!(ManorItemCount, item_id, item_count);

        engine.register_type_with_name::<ManorRewardInfo>("ManorRewardInfo");
        register_to_string!(ManorRewardInfo);
        register_getters!(ManorRewardInfo, item_id, count);

        engine.register_type_with_name::<ManorSowResult>("ManorSowResult");
        register_to_string!(ManorSowResult);
        register_getters!(ManorSowResult, exp);
        engine.register_get("ground", |value: &mut ManorSowResult| value.ground.clone());

        engine.register_type_with_name::<ManorReapResult>("ManorReapResult");
        register_to_string!(ManorReapResult);
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

        engine.register_type_with_name::<ManorUprootResult>("ManorUprootResult");
        register_to_string!(ManorUprootResult);
        engine.register_get("ground", |value: &mut ManorUprootResult| {
            value.ground.clone()
        });

        engine.register_type_with_name::<ManorWeedResult>("ManorWeedResult");
        register_to_string!(ManorWeedResult);
        register_getters!(ManorWeedResult, qq_uin, exp);
        engine.register_get("ground", |value: &mut ManorWeedResult| value.ground.clone());

        engine.register_type_with_name::<ManorFertilizerResult>("ManorFertilizerResult");
        register_to_string!(ManorFertilizerResult);
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

        engine.register_type_with_name::<SpiritBagInfo>("SpiritBagInfo");
        register_to_string!(SpiritBagInfo);
        engine.register_get("spirits", |value: &mut SpiritBagInfo| {
            Self::to_array(&value.spirits)
        });

        engine.register_type_with_name::<StaticItemInfo>("StaticItemInfo");
        register_to_string!(StaticItemInfo);
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

        engine.register_type_with_name::<StaticStriveItemInfo>("StaticStriveItemInfo");
        register_to_string!(StaticStriveItemInfo);
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

        engine.register_type_with_name::<StaticGuardianPetPropertyInfo>(
            "StaticGuardianPetPropertyInfo",
        );
        register_to_string!(StaticGuardianPetPropertyInfo);
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

        engine.register_type_with_name::<StaticTitleInfo>("StaticTitleInfo");
        register_to_string!(StaticTitleInfo);
        register_getters!(StaticTitleInfo, id, title_name);

        engine.register_type_with_name::<StaticMagicInfo>("StaticMagicInfo");
        register_to_string!(StaticMagicInfo);
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

        engine.register_type_with_name::<StaticPluginInfo>("StaticPluginInfo");
        register_to_string!(StaticPluginInfo);
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

        engine.register_type_with_name::<StaticSkillInfo>("StaticSkillInfo");
        register_to_string!(StaticSkillInfo);
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

        engine.register_type_with_name::<StaticSpiritInfo>("StaticSpiritInfo");
        register_to_string!(StaticSpiritInfo);
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
    }
}
