//! 共享类型定义

use serde::{Deserialize, Serialize};

/// 宠物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritInfo {
    pub spirit_id: i64,
    pub position: i64,
    pub catch_time: i64,
    pub name: String,
    pub level: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSpiritInfo {
    pub spirit_id: i64,
    pub count: i64,
    pub area_index: i64,
    pub is_rare: bool,
    pub is_boss: bool,
    pub is_npc_boss: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BagItemInfo {
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub uin: i64,
    pub id: i64,
    pub nick_name: String,
    pub level: i64,
    pub is_vip: bool,
    pub vip_level: i64,
    pub vip_expiring_days: i64,
    pub vip_lulu: i64,
    pub trainer_level: i64,
    pub trainer_exp: i64,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BattleResult {
    pub winner: Option<i64>,
    pub total_rounds: i64,
    pub finish_code: i64,
    pub trainer_exp: i64,
    pub next_level_trainer_exp: i64,
    pub honour_point: i64,
    pub exp_add_bits: i64,
    pub obtained_items: Vec<BagItemInfo>,
    pub spirit_results: Vec<BattleSpiritResult>,
    pub captured_spirits: Vec<BattleCapturedSpirit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSpiritResult {
    pub position: i64,
    pub exp: i64,
    pub level_delta: i64,
    pub level: i64,
    pub next_exp: i64,
    pub effort: i64,
    pub new_skill_ids: Vec<i64>,
    pub evolve_spirit_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleCapturedSpirit {
    pub spirit_id: i64,
    pub level: i64,
    pub disposition: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActions {
    pub can_submit_action: bool,
    pub can_use_skill: bool,
    pub can_capture: bool,
    pub can_use_item: bool,
    pub can_change_spirit: bool,
    pub can_escape: bool,
    pub can_use_any_skill: bool,
    pub can_change_to_any_spirit: bool,
    pub can_combat_mask: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
}

impl ActionResult {
    pub fn ok() -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
        }
    }
}

/// Result returned after refreshing a spirit's talent values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentRefreshResult {
    pub position: i64,
    pub pa_old: i64,
    pub pd_old: i64,
    pub ma_old: i64,
    pub md_old: i64,
    pub sp_old: i64,
    pub hp_old: i64,
    pub pa_new: i64,
    pub pd_new: i64,
    pub ma_new: i64,
    pub md_new: i64,
    pub sp_new: i64,
    pub hp_new: i64,
    pub pa_level_old: i64,
    pub pd_level_old: i64,
    pub ma_level_old: i64,
    pub md_level_old: i64,
    pub sp_level_old: i64,
    pub hp_level_old: i64,
    pub pa_level_new: i64,
    pub pd_level_new: i64,
    pub ma_level_new: i64,
    pub md_level_new: i64,
    pub sp_level_new: i64,
    pub hp_level_new: i64,
}

/// Skill information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub skill_id: i64,
    pub skill_name: String,
    pub pp: i64,
    pub max_pp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticItemInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unique: bool,
    pub item_type: i64,
    pub subtype: i64,
    pub price: i64,
    pub expire_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticStriveItemInfo {
    pub id: i64,
    pub name: String,
    pub item_type: i64,
    pub ghp: i64,
    pub gpa: i64,
    pub gpd: i64,
    pub gma: i64,
    pub gmd: i64,
    pub gsp: i64,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGuardianPetPropertyInfo {
    pub level: i64,
    pub phase: i64,
    pub energy: i64,
    pub attack: i64,
    pub defend: i64,
    pub magic_attack: i64,
    pub magic_defend: i64,
    pub need_level_to_next_phase: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticTitleInfo {
    pub id: i64,
    pub title_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticMagicInfo {
    pub id: i64,
    pub name: String,
    pub item_id: String,
    pub target: i64,
    pub magic_type: i64,
    pub duration: i64,
    pub action_type: i64,
    pub app: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticPluginInfo {
    pub name: String,
    pub label: String,
    pub domain: String,
    pub version: String,
    pub command_type: String,
    pub plugin_class: String,
    pub plugin_src: String,
    pub plugin_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSkillInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub description2: String,
    pub power: String,
    pub pp_max: i64,
    pub property: i64,
    pub src: String,
    pub attack_type: i64,
    pub speed: i64,
    pub damage_type: i64,
    pub catch_rate: i64,
    pub super_form_id: i64,
    pub super_form_src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub features: Vec<i64>,
    pub group: Vec<i64>,
    pub src: String,
    pub avatar: String,
    pub icon_src: String,
    pub preview_src: String,
    pub move_type: i64,
    pub move_speed: i64,
    pub height: String,
    pub weight: String,
    pub color: String,
    pub interest: String,
    pub habitat: String,
    pub evolution: Vec<i64>,
    pub catchrate: i64,
    pub boss_phyle: String,
    pub boss_reward: String,
    pub scene_id: i64,
    pub condition: String,
    pub require_level: String,
    pub wg: i64,
    pub mg: i64,
    pub mk: i64,
    pub sm: i64,
    pub sd: i64,
    pub fy: i64,
    pub reward: i64,
    pub evolution_form_id: i64,
    pub evolution_to_ids: Vec<i64>,
    pub get_form: String,
    pub state: i64,
    pub start_time: String,
    pub end_time: String,
    pub first_id: i64,
    pub propo_level: i64,
    pub is_in_book: bool,
    pub skinnum: i64,
    pub exp_type: i64,
}
