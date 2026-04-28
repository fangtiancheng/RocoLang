//! RocoEngine - 封装 Rhai 引擎并注册标准库

use rhai::{Dynamic, Engine, EvalAltResult, AST};
use std::sync::{Arc, Mutex};

use crate::error::{Result, RocoError};
use crate::stdlib::RocoStdLib;

/// 将 RocoError 转换为 Rhai 错误
fn to_rhai_error(err: RocoError) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(err.to_string().into(), rhai::Position::NONE).into()
}

/// 辅助宏：安全地获取 stdlib 锁并调用需要 &mut self 的方法
macro_rules! call_stdlib_mut {
    ($stdlib:expr, $method:ident $(, $arg:expr)*) => {
        $stdlib
            .lock()
            .map_err(|e| to_rhai_error(RocoError::Other(format!("Lock error: {}", e))))
            .and_then(|mut guard| guard.$method($($arg),*).map_err(to_rhai_error))
    };
}

/// 辅助宏：安全地获取 stdlib 锁并调用只需要 &self 的方法
macro_rules! call_stdlib {
    ($stdlib:expr, $method:ident $(, $arg:expr)*) => {
        $stdlib
            .lock()
            .map_err(|e| to_rhai_error(RocoError::Other(format!("Lock error: {}", e))))
            .and_then(|guard| guard.$method($($arg),*).map_err(to_rhai_error))
    };
}

/// RocoLang 脚本引擎
pub struct RocoEngine {
    engine: Engine,
}

impl RocoEngine {
    /// 创建新的 RocoEngine
    pub fn new(stdlib: Arc<Mutex<dyn RocoStdLib>>) -> Self {
        let mut engine = Engine::new();

        // 注册所有标准库函数
        Self::register_stdlib(&mut engine, stdlib);

        Self { engine }
    }

    /// 注册标准库函数到 Rhai 引擎
    fn register_stdlib(engine: &mut Engine, stdlib: Arc<Mutex<dyn RocoStdLib>>) {
        // ========== 场景相关 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("move_to_scene", move |scene_id: i64| {
                call_stdlib_mut!(stdlib, move_to_scene, scene_id)
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
                call_stdlib_mut!(stdlib, fetch_spirit, catch_time)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("fetch_spirit_by_id", move |spirit_id: i64| {
                call_stdlib_mut!(stdlib, fetch_spirit_by_id, spirit_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("clear_lineup", move || {
                call_stdlib_mut!(stdlib, clear_lineup)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("store_spirit", move |position: i64| {
                call_stdlib_mut!(stdlib, store_spirit, position)
            });
        }

        // ========== 技能/装备 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("learn_skill", move |position: i64, skill_id: i64| {
                call_stdlib_mut!(stdlib, learn_skill, position, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("forget_skill", move |position: i64, slot: i64| {
                call_stdlib_mut!(stdlib, forget_skill, position, slot)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("equip_item", move |position: i64, item_name: &str| {
                call_stdlib_mut!(stdlib, equip_item, position, item_name)
            });
        }

        // ========== 战斗相关 ==========
        {
            let stdlib = stdlib.clone();
            engine.register_fn("invite_pk", move |target_uin: i64| {
                call_stdlib_mut!(stdlib, invite_pk, target_uin)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("accept_pk", move || call_stdlib_mut!(stdlib, accept_pk));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("reject_pk", move || call_stdlib_mut!(stdlib, reject_pk));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("use_skill", move |skill_id: i64| {
                call_stdlib_mut!(stdlib, use_skill, skill_id)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("use_item", move |item_name: &str| {
                call_stdlib_mut!(stdlib, use_item, item_name)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("change_spirit", move |position: i64| {
                call_stdlib_mut!(stdlib, change_spirit, position)
            });
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("defend", move || call_stdlib_mut!(stdlib, defend));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("escape", move || call_stdlib_mut!(stdlib, escape));
        }

        {
            let stdlib = stdlib.clone();
            engine.register_fn("wait_round_end", move || {
                call_stdlib_mut!(stdlib, wait_round_end)
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
