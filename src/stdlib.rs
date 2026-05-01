//! 标准库 trait 定义
//!
//! 服务器和客户端需要各自实现这个 trait

use crate::error::Result;
use crate::types::*;

/// RocoLang 标准库接口
///
/// 所有预定义函数都在这里声明
/// 服务器侧和客户端侧需要各自实现
///
/// 注意：只要求 Send，不要求 Sync，因为每个脚本实例都有独立的 stdlib，
/// 不需要在多个线程间共享同一个实例。
pub trait RocoStdLib: Send {
    // ==================== 场景相关 ====================

    /// 移动到指定场景（同步阻塞，等待服务器确认）
    ///
    /// # 参数
    /// - scene_id: 目标场景 ID
    /// - timeout_ms: 超时时间（毫秒）
    ///
    /// # 返回
    /// Success returns the confirmed scene id. Failure returns `RocoError`.
    fn move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<i64>;

    fn try_move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<ActionResult> {
        match self.get_current_scene() {
            Ok(current_scene) if current_scene == scene_id => return Ok(ActionResult::ok()),
            Ok(_) => {}
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.move_to_scene(scene_id, timeout_ms) {
            Ok(confirmed_scene) if confirmed_scene == scene_id => Ok(ActionResult::ok()),
            Ok(confirmed_scene) => Ok(ActionResult::failed(format!(
                "server confirmed scene {}, expected {}",
                confirmed_scene, scene_id
            ))),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    /// 获取当前场景 ID
    fn get_current_scene(&mut self) -> Result<i64>;

    fn query_server_time(&mut self) -> Result<i64> {
        Err(crate::error::RocoError::StdLibError(
            "query_server_time not implemented".to_string(),
        ))
    }

    fn try_query_server_time(&mut self) -> Result<ActionResult> {
        match self.query_server_time() {
            Ok(stamp) => Ok(ActionResult {
                ok: true,
                code: 0,
                message: stamp.to_string(),
            }),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn session_get_int(&mut self, _key: &str, default_value: i64) -> Result<i64> {
        Ok(default_value)
    }

    fn session_set_int(&mut self, _key: &str, _value: i64) -> Result<bool> {
        Ok(false)
    }

    fn session_get_string(&mut self, _key: &str, default_value: &str) -> Result<String> {
        Ok(default_value.to_string())
    }

    fn session_set_string(&mut self, _key: &str, _value: &str) -> Result<bool> {
        Ok(false)
    }

    fn session_get_bool(&mut self, _key: &str, default_value: bool) -> Result<bool> {
        Ok(default_value)
    }

    fn session_set_bool(&mut self, _key: &str, _value: bool) -> Result<bool> {
        Ok(false)
    }

    fn session_delete(&mut self, _key: &str) -> Result<bool> {
        Ok(false)
    }

    fn session_clear(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn is_in_combat(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn get_user_info(&mut self) -> Result<UserInfo> {
        Err(crate::error::RocoError::StdLibError(
            "get_user_info not implemented".to_string(),
        ))
    }

    // ==================== 宠物管理 ====================

    /// 从仓库取出宠物（通过 catch_time）
    fn fetch_spirit(&mut self, catch_time: i64) -> Result<bool>;

    /// 从仓库取出宠物（通过 ID，取第一只）
    fn fetch_spirit_by_id(&mut self, spirit_id: i64) -> Result<bool>;

    /// 清空当前阵容
    fn challenge_wild_spirit(&mut self, spirit_id: i64) -> Result<bool>;

    fn challenge_boss(&mut self, boss_code: i64) -> Result<bool>;

    fn clear_lineup(&mut self) -> Result<bool>;

    /// 将指定位置的宠物放回仓库
    fn store_spirit(&mut self, position: i64) -> Result<bool>;

    /// 获取背包信息
    fn get_spirit_bag(&mut self) -> Result<SpiritBagInfo>;

    fn get_bag_items(&mut self) -> Result<Vec<BagItemInfo>> {
        Ok(Vec::new())
    }

    fn take_pushed_items(&mut self) -> Result<Vec<BagItemInfo>> {
        Ok(Vec::new())
    }

    fn recover_all_spirits(&mut self) -> Result<bool> {
        Err(crate::error::RocoError::StdLibError(
            "recover_all_spirits not implemented".to_string(),
        ))
    }

    fn get_combat_lineup(&mut self) -> Result<Vec<SpiritInfo>>;

    // ==================== 技能/装备 ====================

    /// 学习技能
    fn learn_skill(&mut self, position: i64, skill_id: i64) -> Result<bool>;

    /// 获取指定位置宠物的技能列表（最多4个技能）
    fn get_skills(&mut self, position: i64) -> Result<[Option<SkillInfo>; 4]>;

    /// 装备道具
    fn equip_item(&mut self, position: i64, item_name: &str) -> Result<bool>;

    // ==================== 静态资料查询 ====================

    fn lookup_item_info(&mut self, item_id: i64) -> Result<StaticItemInfo>;

    fn lookup_skill_info(&mut self, skill_id: i64) -> Result<StaticSkillInfo>;

    fn lookup_spirit_info(&mut self, spirit_id: i64) -> Result<StaticSpiritInfo>;

    // ==================== 战斗相关 ====================

    /// 邀请 PK
    fn invite_pk(&mut self, target_uin: i64) -> Result<BattleInfo>;

    /// 接受 PK 邀请
    fn accept_pk(&mut self) -> Result<bool>;

    /// 拒绝 PK 邀请
    fn reject_pk(&mut self) -> Result<bool>;

    /// 使用技能
    fn use_skill(&mut self, skill_id: i64) -> Result<bool>;

    fn try_use_skill(&mut self, skill_id: i64) -> Result<ActionResult> {
        match self.can_use_skill(skill_id) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("skill unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.use_skill(skill_id) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("use_skill returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    /// 使用道具
    fn use_item(&mut self, item_id: i64) -> Result<bool>;

    fn try_use_item(&mut self, item_id: i64) -> Result<ActionResult> {
        match self.can_use_item(item_id) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("item unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.use_item(item_id) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("use_item returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    /// 更换宠物
    fn change_spirit(&mut self, position: i64) -> Result<bool>;

    fn try_change_spirit(&mut self, position: i64) -> Result<ActionResult> {
        match self.can_change_to_spirit(position) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("target spirit unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.change_spirit(position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("change_spirit returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    /// 防御
    fn defend(&mut self) -> Result<bool>;

    /// 逃跑
    fn combat_escape(&mut self) -> Result<bool>;

    fn try_combat_escape(&mut self) -> Result<ActionResult> {
        match self.get_combat_actions() {
            Ok(actions) if actions.can_escape => {}
            Ok(_) => return Ok(ActionResult::unavailable("combat escape unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.combat_escape() {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("combat_escape returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    /// 等待回合结束
    fn wait_round_end(&mut self) -> Result<RoundResult>;

    /// 获取战斗结果
    fn get_battle_result(&mut self) -> Result<BattleResult>;

    /// 获取当前回合可提交的战斗动作类别
    fn get_combat_actions(&mut self) -> Result<CombatActions>;

    /// 指定技能当前是否可用：同时检查当前回合动作类别、技能是否存在、PP 是否足够
    fn can_use_skill(&mut self, skill_id: i64) -> Result<bool>;

    /// 指定道具当前是否可用：同时检查当前回合动作类别和战斗背包数量
    fn can_use_item(&mut self, item_id: i64) -> Result<bool>;

    /// 指定位置是否可切换：同时检查当前回合动作类别、位置是否存在、是否未阵亡、是否非当前宠
    fn can_change_to_spirit(&mut self, position: i64) -> Result<bool>;

    /// 当前回合是否允许捕捉
    fn can_capture(&mut self) -> Result<bool>;

    /// 获取战斗历史（JSON 字符串）
    fn get_battle_history(&mut self) -> Result<String>;

    // ==================== 状态查询 ====================

    /// 获取我方当前血量
    fn get_my_hp(&mut self) -> Result<i64>;

    /// 获取我方最大血量
    fn get_my_max_hp(&mut self) -> Result<i64>;

    /// 获取对手当前血量
    fn get_rival_hp(&mut self) -> Result<i64>;

    /// 获取对手最大血量
    fn get_rival_max_hp(&mut self) -> Result<i64>;

    /// 获取我方技能 PP
    fn get_my_pp(&mut self, slot: i64) -> Result<i64>;

    fn get_my_spirit_info(&mut self, position: i64) -> Result<SpiritInfo>;

    /// 获取对手宠物信息（可见部分）
    fn get_rival_spirit_info(&mut self) -> Result<SpiritInfo>;

    /// 战斗是否结束
    fn is_combat_finished(&mut self) -> Result<bool>;

    /// Deprecated: use is_combat_finished().
    fn is_finished(&mut self) -> Result<bool> {
        self.is_combat_finished()
    }

    /// 获取当前回合数
    fn get_current_round(&mut self) -> Result<i64>;

    // ==================== 工具函数 ====================

    /// 休眠（毫秒）
    fn sleep(&mut self, ms: i64) -> Result<()>;

    fn now_ms(&mut self) -> Result<i64> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| crate::error::RocoError::StdLibError(error.to_string()))?;
        i64::try_from(now.as_millis()).map_err(|_| {
            crate::error::RocoError::StdLibError("current timestamp exceeds i64 range".to_string())
        })
    }

    fn sleep_until_ms(&mut self, target_ms: i64) -> Result<()> {
        let now = self.now_ms()?;
        if target_ms <= now {
            return Ok(());
        }
        self.sleep(target_ms - now)
    }

    fn format_time(&mut self, timestamp: i64) -> Result<String> {
        Ok(timestamp.to_string())
    }

    /// 日志输出
    fn log(&mut self, message: &str) -> Result<()>;

    /// 断言
    fn assert(&mut self, condition: bool, message: &str) -> Result<()>;
}
