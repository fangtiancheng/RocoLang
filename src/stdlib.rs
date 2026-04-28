//! 标准库 trait 定义
//!
//! 服务器和客户端需要各自实现这个 trait

use crate::error::Result;
use crate::types::*;

/// RocoLang 标准库接口
///
/// 所有预定义函数都在这里声明
/// 服务器侧和客户端侧需要各自实现
pub trait RocoStdLib: Send + Sync {
    // ==================== 场景相关 ====================

    /// 移动到指定场景
    fn move_to_scene(&mut self, scene_id: i64) -> Result<bool>;

    /// 获取当前场景 ID
    fn get_current_scene(&self) -> Result<i64>;

    // ==================== 宠物管理 ====================

    /// 从仓库取出宠物（通过 catch_time）
    fn fetch_spirit(&mut self, catch_time: i64) -> Result<bool>;

    /// 从仓库取出宠物（通过 ID，取第一只）
    fn fetch_spirit_by_id(&mut self, spirit_id: i64) -> Result<bool>;

    /// 清空当前阵容
    fn clear_lineup(&mut self) -> Result<bool>;

    /// 将指定位置的宠物放回仓库
    fn store_spirit(&mut self, position: i64) -> Result<bool>;

    /// 获取背包信息
    fn get_spirit_bag(&self) -> Result<SpiritBagInfo>;

    /// 获取当前阵容
    fn get_lineup(&self) -> Result<Vec<SpiritInfo>>;

    // ==================== 技能/装备 ====================

    /// 学习技能
    fn learn_skill(&mut self, position: i64, skill_id: i64) -> Result<bool>;

    /// 遗忘技能
    fn forget_skill(&mut self, position: i64, slot: i64) -> Result<bool>;

    /// 装备道具
    fn equip_item(&mut self, position: i64, item_name: &str) -> Result<bool>;

    // ==================== 战斗相关 ====================

    /// 邀请 PK
    fn invite_pk(&mut self, target_uin: i64) -> Result<BattleInfo>;

    /// 接受 PK 邀请
    fn accept_pk(&mut self) -> Result<bool>;

    /// 拒绝 PK 邀请
    fn reject_pk(&mut self) -> Result<bool>;

    /// 使用技能
    fn use_skill(&mut self, skill_id: i64) -> Result<bool>;

    /// 使用道具
    fn use_item(&mut self, item_name: &str) -> Result<bool>;

    /// 更换宠物
    fn change_spirit(&mut self, position: i64) -> Result<bool>;

    /// 防御
    fn defend(&mut self) -> Result<bool>;

    /// 逃跑
    fn escape(&mut self) -> Result<bool>;

    /// 等待回合结束
    fn wait_round_end(&mut self) -> Result<RoundResult>;

    /// 获取战斗结果
    fn get_battle_result(&self) -> Result<BattleResult>;

    /// 获取战斗历史（JSON 字符串）
    fn get_battle_history(&self) -> Result<String>;

    // ==================== 状态查询 ====================

    /// 获取我方当前血量
    fn get_my_hp(&self) -> Result<i64>;

    /// 获取我方最大血量
    fn get_my_max_hp(&self) -> Result<i64>;

    /// 获取对手当前血量
    fn get_rival_hp(&self) -> Result<i64>;

    /// 获取对手最大血量
    fn get_rival_max_hp(&self) -> Result<i64>;

    /// 获取我方技能 PP
    fn get_my_pp(&self, slot: i64) -> Result<i64>;

    /// 获取我方宠物信息
    fn get_my_spirit_info(&self, position: i64) -> Result<SpiritInfo>;

    /// 获取对手宠物信息（可见部分）
    fn get_rival_spirit_info(&self) -> Result<SpiritInfo>;

    /// 战斗是否结束
    fn is_finished(&self) -> Result<bool>;

    /// 获取当前回合数
    fn get_current_round(&self) -> Result<i64>;

    // ==================== 工具函数 ====================

    /// 休眠（毫秒）
    fn sleep(&self, ms: i64) -> Result<()>;

    /// 日志输出
    fn log(&self, message: &str) -> Result<()>;

    /// 断言
    fn assert(&self, condition: bool, message: &str) -> Result<()>;
}
