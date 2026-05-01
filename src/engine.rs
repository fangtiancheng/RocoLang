//! RocoEngine - 封装 Rhai 引擎并注册标准库

use rhai::{Array, Dynamic, Engine, EvalAltResult, AST};
use std::sync::{Arc, Mutex};

use crate::error::{Result, RocoError};
use crate::stdlib::RocoStdLib;
use crate::types::{
    ActionResult, BagItemInfo, BattleCapturedSpirit, BattleResult, BattleSpiritResult,
    CombatActions, SpiritBagInfo, SpiritInfo, SpiritSkillInfo, StaticItemInfo, StaticSkillInfo,
    StaticSpiritInfo, UserInfo,
};

/// Print callback 类型别名
type PrintCallback = Arc<Mutex<dyn FnMut(&str) + Send>>;

/// 将 RocoError 转换为 Rhai 错误
fn to_rhai_error(err: RocoError) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(err.to_string().into(), rhai::Position::NONE).into()
}

/// 辅助宏：安全地获取 stdlib 锁并调用方法
/// 所有方法现在都需要 &mut self
macro_rules! call_stdlib {
    ($stdlib:expr, $method:ident $(, $arg:expr)*) => {
        $stdlib
            .lock()
            .map_err(|e| to_rhai_error(RocoError::Other(format!("Lock error: {}", e))))
            .and_then(|mut guard| guard.$method($($arg),*).map_err(to_rhai_error))
    };
}

/// RocoLang 脚本引擎
pub struct RocoEngine {
    engine: Engine,
    print_callback: Option<PrintCallback>,
}

impl RocoEngine {
    /// 创建新的 RocoEngine
    ///
    /// stdlib 必须用 Arc<Mutex<>> 包装，因为：
    /// 1. Rhai 的多个函数闭包需要共享同一个 stdlib（Arc）
    /// 2. 所有 stdlib 方法都需要 &mut self（Mutex）
    /// 3. Rhai 要求闭包是 Sync（Mutex 而非 RefCell）
    pub fn new<T: RocoStdLib + 'static>(stdlib: Arc<Mutex<T>>) -> Self {
        let mut engine = Engine::new();
        engine.set_max_expr_depths(0, 0);

        // 注册所有标准库函数
        Self::register_stdlib(&mut engine, stdlib);
        Self::register_builtin_helpers(&mut engine);

        Self {
            engine,
            print_callback: None,
        }
    }

    /// 设置 print 输出回调
    ///
    /// 当脚本调用 print() 或 debug() 时，会调用此回调函数
    pub fn set_print_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(callback));
        self.print_callback = Some(callback.clone());

        // 设置 Rhai 的 on_print 回调
        let print_cb = callback.clone();
        self.engine.on_print(move |text| {
            if let Ok(mut cb) = print_cb.lock() {
                cb(text);
            }
        });

        // 设置 Rhai 的 on_debug 回调（用于 debug() 函数）
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

    /// 注册标准库函数到 Rhai 引擎
    fn register_stdlib<T: RocoStdLib + 'static>(engine: &mut Engine, stdlib: Arc<Mutex<T>>) {
        Self::register_static_info_types(engine);

        // ========== 场景相关 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("move_to_scene", move |scene_id: i64, timeout_ms: i64| {
                call_stdlib!(stdlib, move_to_scene, scene_id, timeout_ms)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn(
                "try_move_to_scene",
                move |scene_id: i64, timeout_ms: i64| {
                    call_stdlib!(stdlib, try_move_to_scene, scene_id, timeout_ms)
                },
            );
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_current_scene", move || {
                call_stdlib!(stdlib, get_current_scene)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("is_in_combat", move || call_stdlib!(stdlib, is_in_combat));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_user_info", move || call_stdlib!(stdlib, get_user_info));
        }

        // ========== 宠物管理 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("fetch_spirit", move |catch_time: i64| {
                call_stdlib!(stdlib, fetch_spirit, catch_time)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("fetch_spirit_by_id", move |spirit_id: i64| {
                call_stdlib!(stdlib, fetch_spirit_by_id, spirit_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("challenge_wild_spirit", move |spirit_id: i64| {
                call_stdlib!(stdlib, challenge_wild_spirit, spirit_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("challenge_boss", move |boss_code: i64| {
                call_stdlib!(stdlib, challenge_boss, boss_code)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("clear_lineup", move || call_stdlib!(stdlib, clear_lineup));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("store_spirit", move |position: i64| {
                call_stdlib!(stdlib, store_spirit, position)
            });
        }

        // ========== 技能/装备 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_spirit_bag", move || {
                call_stdlib!(stdlib, get_spirit_bag)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_bag_items", move || {
                call_stdlib!(stdlib, get_bag_items).map(|items| Self::to_array(&items))
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("take_pushed_items", move || {
                call_stdlib!(stdlib, take_pushed_items).map(|items| Self::to_array(&items))
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("recover_all_spirits", move || {
                call_stdlib!(stdlib, recover_all_spirits)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_combat_lineup", move || {
                call_stdlib!(stdlib, get_combat_lineup).map(|spirits| Self::to_array(&spirits))
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("learn_skill", move |position: i64, skill_id: i64| {
                call_stdlib!(stdlib, learn_skill, position, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_skills", move |position: i64| {
                call_stdlib!(stdlib, get_skills, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("equip_item", move |position: i64, item_name: &str| {
                call_stdlib!(stdlib, equip_item, position, item_name)
            });
        }

        // ========== 静态资料查询 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("lookup_item_info", move |item_id: i64| {
                call_stdlib!(stdlib, lookup_item_info, item_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("lookup_skill_info", move |skill_id: i64| {
                call_stdlib!(stdlib, lookup_skill_info, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("lookup_spirit_info", move |spirit_id: i64| {
                call_stdlib!(stdlib, lookup_spirit_info, spirit_id)
            });
        }

        // ========== 战斗相关 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("invite_pk", move |target_uin: i64| {
                call_stdlib!(stdlib, invite_pk, target_uin)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("accept_pk", move || call_stdlib!(stdlib, accept_pk));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("reject_pk", move || call_stdlib!(stdlib, reject_pk));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("use_skill", move |skill_id: i64| {
                call_stdlib!(stdlib, use_skill, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("try_use_skill", move |skill_id: i64| {
                call_stdlib!(stdlib, try_use_skill, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("use_item", move |item_id: i64| {
                call_stdlib!(stdlib, use_item, item_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("try_use_item", move |item_id: i64| {
                call_stdlib!(stdlib, try_use_item, item_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("change_spirit", move |position: i64| {
                call_stdlib!(stdlib, change_spirit, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("try_change_spirit", move |position: i64| {
                call_stdlib!(stdlib, try_change_spirit, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("defend", move || call_stdlib!(stdlib, defend));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("combat_escape", move || call_stdlib!(stdlib, combat_escape));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("try_combat_escape", move || {
                call_stdlib!(stdlib, try_combat_escape)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("wait_round_end", move || {
                call_stdlib!(stdlib, wait_round_end)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_battle_result", move || {
                call_stdlib!(stdlib, get_battle_result)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_combat_actions", move || {
                call_stdlib!(stdlib, get_combat_actions)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("can_use_skill", move |skill_id: i64| {
                call_stdlib!(stdlib, can_use_skill, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("can_use_item", move |item_id: i64| {
                call_stdlib!(stdlib, can_use_item, item_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("can_change_to_spirit", move |position: i64| {
                call_stdlib!(stdlib, can_change_to_spirit, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("can_capture", move || call_stdlib!(stdlib, can_capture));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_battle_history", move || {
                call_stdlib!(stdlib, get_battle_history)
            });
        }

        // ========== 状态查询 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_my_hp", move || call_stdlib!(stdlib, get_my_hp));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_my_max_hp", move || call_stdlib!(stdlib, get_my_max_hp));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_rival_hp", move || call_stdlib!(stdlib, get_rival_hp));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_rival_max_hp", move || {
                call_stdlib!(stdlib, get_rival_max_hp)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_my_pp", move |slot: i64| {
                call_stdlib!(stdlib, get_my_pp, slot)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_my_spirit_info", move |position: i64| {
                call_stdlib!(stdlib, get_my_spirit_info, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_rival_spirit_info", move || {
                call_stdlib!(stdlib, get_rival_spirit_info)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("is_finished", move || call_stdlib!(stdlib, is_finished));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("is_combat_finished", move || {
                call_stdlib!(stdlib, is_combat_finished)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_current_round", move || {
                call_stdlib!(stdlib, get_current_round)
            });
        }

        // ========== 工具函数 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("sleep", move |ms: i64| call_stdlib!(stdlib, sleep, ms));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("format_time", move |timestamp: i64| {
                call_stdlib!(stdlib, format_time, timestamp)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("log", move |message: &str| {
                call_stdlib!(stdlib, log, message)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("assert", move |condition: bool, message: &str| {
                call_stdlib!(stdlib, assert, condition, message)
            });
        }
    }

    /// 执行脚本字符串
    pub fn eval(&mut self, script: &str) -> Result<Dynamic> {
        self.engine
            .eval(script)
            .map_err(|e| RocoError::ScriptError(e.to_string()))
    }

    /// 编译脚本为 AST（可复用）
    pub fn compile(&self, script: &str) -> Result<AST> {
        self.engine
            .compile(script)
            .map_err(|e| RocoError::ScriptError(e.to_string()))
    }

    /// 执行已编译的 AST
    pub fn eval_ast(&mut self, ast: &AST) -> Result<Dynamic> {
        self.engine
            .eval_ast(ast)
            .map_err(|e| RocoError::ScriptError(e.to_string()))
    }

    /// 调用脚本中的函数
    pub fn call_fn<T: Clone + 'static>(
        &mut self,
        ast: &AST,
        fn_name: &str,
        args: impl rhai::FuncArgs,
    ) -> Result<T> {
        let mut scope = rhai::Scope::new();
        self.engine
            .call_fn(&mut scope, ast, fn_name, args)
            .map_err(|e| RocoError::ScriptError(e.to_string()))
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

        engine.register_type_with_name::<CombatActions>("CombatActions");
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
            can_combat_mask,
        );

        engine.register_type_with_name::<ActionResult>("ActionResult");
        register_getters!(ActionResult, ok, code, message);

        engine.register_type_with_name::<UserInfo>("UserInfo");
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
        register_getters!(SpiritInfo, spirit_id, position, catch_time, name, level, hp, max_hp);
        engine.register_get("skills", |value: &mut SpiritInfo| {
            Self::to_array(&value.skills)
        });

        engine.register_type_with_name::<SpiritSkillInfo>("SpiritSkillInfo");
        register_getters!(SpiritSkillInfo, skill_id, pp, inherited);

        engine.register_type_with_name::<BagItemInfo>("BagItemInfo");
        register_getters!(BagItemInfo, item_id, count);

        engine.register_type_with_name::<BattleSpiritResult>("BattleSpiritResult");
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
        register_getters!(BattleCapturedSpirit, spirit_id, level, disposition);

        engine.register_type_with_name::<BattleResult>("BattleResult");
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

        engine.register_type_with_name::<SpiritBagInfo>("SpiritBagInfo");
        engine.register_get("spirits", |value: &mut SpiritBagInfo| {
            Self::to_array(&value.spirits)
        });

        engine.register_type_with_name::<StaticItemInfo>("StaticItemInfo");
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

        engine.register_type_with_name::<StaticSkillInfo>("StaticSkillInfo");
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
        register_getters!(
            StaticSpiritInfo,
            id,
            name,
            description,
            src,
            avatar,
            icon_src,
            preview_src,
            move_type,
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
