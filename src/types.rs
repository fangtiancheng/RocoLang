//! 共享类型定义

use serde::{Deserialize, Serialize};

/// 宠物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritInfo {
    pub catch_time: i64,
    pub name: String,
    pub level: i64,
    pub hp: i64,
    pub max_hp: i64,
}

/// 宠物背包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBagInfo {
    pub spirits: Vec<SpiritInfo>,
}

/// 战斗信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInfo {
    pub battle_id: String,
    pub my_uin: i64,
    pub rival_uin: i64,
    pub started: bool,
}

/// 回合结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round: i64,
    pub my_hp: i64,
    pub rival_hp: i64,
    pub finished: bool,
}

/// 战斗结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    pub winner: Option<i64>,
    pub total_rounds: i64,
}

/// 技能信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub skill_id: i64,
    pub skill_name: String,
    pub pp: i64,
    pub max_pp: i64,
}
