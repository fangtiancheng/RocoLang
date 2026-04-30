//! RocoEngine - 封装 Rhai 引擎并注册标准库

use rhai::{Dynamic, Engine, EvalAltResult, AST};
use std::sync::{Arc, Mutex};

use crate::error::{Result, RocoError};
use crate::stdlib::RocoStdLib;

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

        // 注册所有标准库函数
        Self::register_stdlib(&mut engine, stdlib);

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
        // ========== 场景相关 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("move_to_scene", move |scene_id: i64, timeout_ms: i64| {
                call_stdlib!(stdlib, move_to_scene, scene_id, timeout_ms)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_current_scene", move || {
                call_stdlib!(stdlib, get_current_scene)
            });
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
            engine.register_fn("get_lineup", move || call_stdlib!(stdlib, get_lineup));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("get_lineup_count", move || {
                call_stdlib!(stdlib, get_lineup_count)
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
            engine.register_fn("use_item", move |item_name: &str| {
                call_stdlib!(stdlib, use_item, item_name)
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
            engine.register_fn("defend", move || call_stdlib!(stdlib, defend));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("escape", move || call_stdlib!(stdlib, escape));
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
            engine.register_fn("get_my_power_skill", move || {
                call_stdlib!(stdlib, get_my_power_skill)
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
}
