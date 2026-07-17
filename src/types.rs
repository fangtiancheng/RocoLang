//! 共享类型定义

use crate::{RocoError, RocoErrorInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoRequestContext {
    pub raw: String,
    pub domain: String,
    pub action: String,
}

impl RocoRequestContext {
    pub fn from_raw(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        let (domain, action) = raw
            .split_once('.')
            .map(|(domain, action)| (domain.to_string(), action.to_string()))
            .unwrap_or_else(|| (raw.clone(), String::new()));
        Self {
            raw,
            domain,
            action,
        }
    }
}

impl From<String> for RocoRequestContext {
    fn from(raw: String) -> Self {
        Self::from_raw(raw)
    }
}

impl From<&str> for RocoRequestContext {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoRewardKind {
    Item,
    Money,
    AssignableExp,
    Furniture,
    Spirit,
    SpiritEquipment,
    TimedDress,
    Unmapped,
}

impl RocoRewardKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Item => "item",
            Self::Money => "money",
            Self::AssignableExp => "assignable_exp",
            Self::Furniture => "furniture",
            Self::Spirit => "spirit",
            Self::SpiritEquipment => "spirit_equipment",
            Self::TimedDress => "timed_dress",
            Self::Unmapped => "unmapped",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoOptionalI64 {
    Missing,
    Present { value: i64 },
}

impl RocoOptionalI64 {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: i64) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub const fn value_or(self, default: i64) -> i64 {
        match self {
            Self::Missing => default,
            Self::Present { value } => value,
        }
    }
}

impl From<Option<i64>> for RocoOptionalI64 {
    fn from(value: Option<i64>) -> Self {
        value.map(Self::present).unwrap_or(Self::Missing)
    }
}

impl From<Option<u32>> for RocoOptionalI64 {
    fn from(value: Option<u32>) -> Self {
        value
            .map(|value| Self::present(i64::from(value)))
            .unwrap_or(Self::Missing)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDisplayItem {
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoOptionalDisplayItem {
    Missing,
    Present { item: RocoDisplayItem },
}

impl RocoOptionalDisplayItem {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(item: RocoDisplayItem) -> Self {
        Self::Present { item }
    }

    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub const fn item_or_default(self) -> RocoDisplayItem {
        match self {
            Self::Missing => RocoDisplayItem {
                item_id: 0,
                item_count: 0,
                item_type: 0,
            },
            Self::Present { item } => item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_context_splits_domain_and_action() {
        let context = RocoRequestContext::from_raw("virgo.query_status");
        assert_eq!(context.raw, "virgo.query_status");
        assert_eq!(context.domain, "virgo");
        assert_eq!(context.action, "query_status");
    }

    #[test]
    fn request_context_preserves_unknown_single_segment() {
        let context = RocoRequestContext::from_raw("legacy");
        assert_eq!(context.raw, "legacy");
        assert_eq!(context.domain, "legacy");
        assert_eq!(context.action, "");
    }

    #[test]
    fn reward_kind_has_stable_script_code() {
        assert_eq!(RocoRewardKind::Item.code(), "item");
        assert_eq!(RocoRewardKind::AssignableExp.code(), "assignable_exp");
        assert_eq!(RocoRewardKind::SpiritEquipment.code(), "spirit_equipment");
        assert_eq!(RocoRewardKind::TimedDress.code(), "timed_dress");
    }

    #[test]
    fn optional_i64_tracks_presence_without_sentinel() {
        let missing = RocoOptionalI64::from(None::<u32>);
        let present = RocoOptionalI64::from(Some(7_u32));
        assert!(!missing.is_present());
        assert_eq!(missing.value_or(42), 42);
        assert!(present.is_present());
        assert_eq!(present.value_or(42), 7);
    }
}

/// 宠物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritInfo {
    pub spirit_id: i64,
    pub position: i64,
    pub catch_time: RocoOptionalI64,
    pub name: String,
    pub level: i64,
    pub personality: i64,
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
pub struct SceneRoleInfo {
    pub uin: i64,
    pub id: i64,
    pub nick_name: String,
    pub level: i64,
    pub loc_x: i64,
    pub loc_y: i64,
    pub pk_state: i64,
    pub is_in_combat: bool,
    pub is_vip: bool,
    pub vip_level: i64,
    pub trainer_level: i64,
    pub trainer_exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPoolSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPoolInfo {
    pub spirit_id: i64,
    pub position: i64,
    pub skills: Vec<SkillPoolSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSwitchResult {
    pub spirit_id: i64,
    pub position: i64,
    pub skill_slot: i64,
    pub skill_id: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillStoneSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillStoneResult {
    pub ok: bool,
    pub result_code: i64,
    pub message: String,
    pub item_id: i64,
    pub position: i64,
    pub needs_replace: bool,
    pub old_skills: Vec<SkillStoneSkillInfo>,
    pub new_skills: Vec<SkillStoneSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpiritInfo {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub storage_time: i64,
    pub level: i64,
    pub sex: i64,
    pub skin_flag: i64,
    pub talent_type: i64,
    pub talent_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpiritDetailInfo {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub storage_time: i64,
    pub name: String,
    pub level: i64,
    pub personality: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub pa: i64,
    pub pd: i64,
    pub ma: i64,
    pub md: i64,
    pub sp: i64,
    pub hp_ability: i64,
    pub skills: Vec<SpiritSkillInfo>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTimeInfo {
    pub stamp: i64,
    pub full_year: i64,
    pub month: i64,
    pub date: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub day: i64,
    pub day_of_year: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTimeResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: ServerTimeInfo,
}

impl ServerTimeResult {
    pub fn ok(result: ServerTimeInfo) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
            result: ServerTimeInfo::default(),
        }
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self {
            ok: false,
            code: 2,
            message,
            error: Some(error.info()),
            result: ServerTimeInfo::default(),
        }
    }
}

/// 家园锻炼奖励道具。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetTrainingRewardItem {
    pub item_id: i64,
    pub count: i64,
}

/// 家园锻炼 CGI 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetTrainingResult {
    pub ok: bool,
    pub result_code: i64,
    pub message: String,
    pub training_type: i64,
    pub pet_id: i64,
    pub rewards: Vec<PetTrainingRewardItem>,
    pub raw_text: String,
}

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
pub struct BattleResultQueryResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: Option<BattleResult>,
}

impl BattleResultQueryResult {
    pub fn ok(result: BattleResult) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result: Some(result),
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
            result: None,
        }
    }

    pub fn unavailable_with_error(error: RocoError) -> Self {
        Self {
            ok: false,
            code: 1,
            message: error.message(),
            error: Some(error.info()),
            result: None,
        }
    }
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
    pub property_list: Vec<i64>,
    pub flair_list: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReport {
    pub id: i64,
    pub report_type: i64,
    pub begin_time: i64,
    pub end_time: i64,
    pub act_begin_time: Vec<i64>,
    pub act_end_time: Vec<i64>,
    pub name_image_url: String,
    pub app_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReportsResult {
    pub reports: Vec<NewsTimesReport>,
    pub player_status_today: Vec<i64>,
    pub player_status_forever: Vec<i64>,
    pub gift_gotten: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsActiveItem {
    pub id: i64,
    pub scene_id: i64,
    pub npc_x: i64,
    pub npc_y: i64,
    pub time: String,
    pub content: String,
    pub auto_start: bool,
    pub script_url: String,
    pub app_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerInfo {
    pub result_code: i64,
    pub message: String,
    pub mop: i64,
    pub boss_id: i64,
    pub countdown: i64,
    pub auto_sell: bool,
    pub money: i64,
    pub clips: Vec<i64>,
    pub storeys: Vec<StarTowerStorey>,
    pub top: RocoOptionalStarTowerTop,
}

impl StarTowerInfo {
    pub fn has_top(&self) -> bool {
        self.top.is_present()
    }

    pub fn top_or_default(&self) -> StarTowerTop {
        self.top.top_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerStorey {
    pub storey_index: i64,
    pub first: i64,
    pub can_quick_fight: bool,
    pub nodes: Vec<StarTowerNode>,
    pub exchange_items: Vec<StarTowerExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerNode {
    pub node_index: i64,
    pub star: i64,
    pub spirit_id: i64,
    pub fight_id: i64,
    pub item_id: i64,
    pub reward: i64,
    pub equip_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerExchangeItem {
    pub index: i64,
    pub item_id: i64,
    pub item_name: String,
    pub spirit_id: RocoOptionalI64,
    pub spirit_name: String,
    pub owned: i64,
    pub required: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTowerTop {
    pub star: i64,
    pub refresh: i64,
    pub fight_desc: String,
    pub task_desc: String,
    pub fight_id: i64,
    pub tokens: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub missions: Vec<StarTowerTopMission>,
    pub rewards: Vec<StarTowerTopReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalStarTowerTop {
    Missing,
    Present { top: StarTowerTop },
}

impl RocoOptionalStarTowerTop {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(top: StarTowerTop) -> Self {
        Self::Present { top }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn top_or_default(&self) -> StarTowerTop {
        match self {
            Self::Missing => StarTowerTop::default(),
            Self::Present { top } => top.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerTopMission {
    pub index: i64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerTopReward {
    pub index: i64,
    pub threshold: i64,
    pub name: String,
    pub amount: String,
    pub state: i64,
    pub claimed: bool,
    pub claimable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelIntelligenceInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub added_bounty: i64,
    pub refresh_count: i64,
    pub exchange_refresh_count: i64,
    pub mission_type: i64,
    pub mission_values: Vec<i64>,
    pub fight_times: i64,
    pub bounty: i64,
    pub intelligence_count: i64,
    pub bosses: Vec<SentinelBossInfo>,
    pub exchanges: Vec<SentinelExchangeInfo>,
    pub spirits: Vec<SentinelSpiritExchangeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelBossInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub difficulty: i64,
    pub status: i64,
    pub max_intelligence: i64,
    pub intelligence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelExchangeInfo {
    pub index: i64,
    pub item_id: i64,
    pub need_bounty: i64,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelSpiritExchangeInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub need_intelligence: i64,
    pub evolve_spirit_id: i64,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub seal_count: i64,
    pub success: i64,
    pub attrs: Vec<i64>,
    pub bosses: Vec<MountainSeaBossInfo>,
    pub souls: Vec<MountainSeaSoulInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaBossInfo {
    pub index: i64,
    pub boss_type: i64,
    pub fight_id: i64,
    pub name: String,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaSoulInfo {
    pub soul_type: i64,
    pub boss_type: i64,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityExpeditionInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub fight_index: i64,
    pub vip: bool,
    pub vip_pass_enabled: bool,
    pub schedule: i64,
    pub schedule_name: String,
    pub added_reputation: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityReputationInfo {
    pub result_code: i64,
    pub message: String,
    pub reputation: i64,
    pub exchanges: Vec<DarkCityExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityExchangeItem {
    pub index: i64,
    pub item_id: i64,
    pub cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionBattleInfo {
    pub index: i64,
    pub battle_id: i64,
    pub attr_types: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionRecipeInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub energy_cost: i64,
    pub required_spirit_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionInfo {
    pub result_code: i64,
    pub message: String,
    pub times: i64,
    pub energy: i64,
    pub added_energy: i64,
    pub battles: Vec<MysteryFusionBattleInfo>,
    pub recipes: Vec<MysteryFusionRecipeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionMaterialCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub bag_index: i64,
    pub level: i64,
    pub personality: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionMaterialBag {
    pub result_code: i64,
    pub message: String,
    pub candidates: Vec<MysteryFusionMaterialCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureRealmInfo {
    pub result_code: i64,
    pub message: String,
    pub battle: i64,
    pub battle_id: i64,
    pub schedule: i64,
    pub possible: i64,
    pub time: i64,
    pub got_box: bool,
    pub item_counts: Vec<i64>,
    pub commits: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonInfo {
    pub result_code: i64,
    pub message: String,
    pub vip: i64,
    pub magic: i64,
    pub count: i64,
    pub show: i64,
    pub pools: Vec<SummonPoolState>,
    pub config_pools: Vec<SummonPoolConfig>,
    pub exchange_groups: Vec<SummonExchangeGroup>,
    pub rewards: Vec<SummonRewardItem>,
    pub records: Vec<SummonRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolState {
    pub version: i64,
    pub token_item_id: i64,
    pub token_count: i64,
    pub today_draw_count: i64,
    pub wish_index: i64,
    pub succeeded: bool,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolConfig {
    pub version: i64,
    pub title: String,
    pub vip_limit: i64,
    pub end_time: i64,
    pub daily_max: i64,
    pub token_item_id: i64,
    pub recommend: String,
    pub info: String,
    pub reward_text: String,
    pub rewards: Vec<SummonRewardItem>,
    pub wish_candidates: Vec<SummonRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeGroup {
    pub kind: String,
    pub items: Vec<SummonExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeItem {
    pub index: i64,
    pub reward: SummonRewardItem,
    pub cost: SummonRewardItem,
    pub need: i64,
    pub max: i64,
    pub day_max: i64,
    pub times: i64,
    pub day_times: i64,
    pub add: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummonRewardItem {
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonRecord {
    pub pool_version: i64,
    pub title: String,
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayGuideRewardItem {
    pub id: i64,
    pub count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekTaskActivity {
    pub activity_id: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekTaskInfo {
    pub result_code: i64,
    pub message: String,
    pub progress: Vec<i64>,
    pub button_states: Vec<i64>,
    pub ticket_item_id: i64,
    pub ticket_count: i64,
    pub new_activities: Vec<WeekTaskActivity>,
    pub old_activities: Vec<WeekTaskActivity>,
    pub rewards: Vec<PlayGuideRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTaskProgress {
    pub index: i64,
    pub current: i64,
    pub target: i64,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondProgressReward {
    pub index: i64,
    pub threshold: i64,
    pub state: i64,
    pub claimable: bool,
    pub claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTaskInfo {
    pub result_code: i64,
    pub message: String,
    pub vip: i64,
    pub reward_type: i64,
    pub tasks: Vec<DiamondTaskProgress>,
    pub rewards: Vec<DiamondProgressReward>,
    pub reward_items: Vec<PlayGuideRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqGameHallGiftInfo {
    pub result_code: i64,
    pub message: String,
    pub rewards: Vec<PlayGuideRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub story_id: i64,
    pub task_id: i64,
    pub status: i64,
    pub task_type: i64,
    pub task_type_sub: i64,
    pub theme_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoList {
    pub result_code: i64,
    pub message: String,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItemChanged {
    pub item_id: i64,
    pub item_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTaskResult {
    pub result_code: i64,
    pub message: String,
    pub money_add: i64,
    pub exp_add: i64,
    pub honor_add: i64,
    pub power_add: i64,
    pub intellect_add: i64,
    pub charm_add: i64,
    pub story_id: i64,
    pub changed_items: Vec<TaskItemChanged>,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAchievement {
    pub theme_id: i64,
    pub finish_time: i64,
    pub story_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAchievementList {
    pub result_code: i64,
    pub message: String,
    pub achievements: Vec<TaskAchievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicGrowupInfo {
    pub ui_ret: i64,
    pub rating_title: i64,
    pub progress: i64,
    pub energy: i64,
    pub spirit_levels: Vec<i64>,
    pub honor_times: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionProgress {
    pub state: i64,
    pub now_value: i64,
    pub max_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionStatusRaw {
    pub result_code: i64,
    pub message: String,
    pub task_id: i64,
    pub condition_status_bytes: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionStatus {
    pub result_code: i64,
    pub message: String,
    pub task_id: i64,
    pub conditions: Vec<TaskConditionProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionApplyResult {
    pub result_code: i64,
    pub message: String,
    pub npc_id: i64,
    pub changed_items: Vec<TaskItemChanged>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMachineRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMachineInfo {
    pub result_code: i64,
    pub message: String,
    pub can_play: bool,
    pub coin: i64,
    pub main_pet_id: RocoOptionalI64,
    pub storage_full: bool,
    pub pet_id: RocoOptionalI64,
    pub rewards: Vec<JumpMachineRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornPalaceNoteItem {
    pub item_index: i64,
    pub item_id: i64,
    pub count: i64,
    pub need: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornPalaceNotesInfo {
    pub items: Vec<CapricornPalaceNoteItem>,
    pub can_summon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornTeamPlayer {
    pub uin: i64,
    pub nick: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapricornTeamSnapshot {
    pub players: Vec<CapricornTeamPlayer>,
    pub ticks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornInviteListInfo {
    pub result_code: i64,
    pub message: String,
    pub players: Vec<CapricornTeamPlayer>,
    pub ticks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornTeamOperationInfo {
    pub result_code: i64,
    pub message: String,
    pub team: RocoOptionalCapricornTeamSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalCapricornTeamSnapshot {
    Missing,
    Present { team: CapricornTeamSnapshot },
}

impl RocoOptionalCapricornTeamSnapshot {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(team: CapricornTeamSnapshot) -> Self {
        Self::Present { team }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn team_or_default(&self) -> CapricornTeamSnapshot {
        match self {
            Self::Missing => CapricornTeamSnapshot::default(),
            Self::Present { team } => team.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapricornSecondTask {
    pub task_type: i64,
    pub data1: i64,
    pub data2: i64,
    pub step: i64,
    pub current: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalCapricornSecondTask {
    Missing,
    Present { task: CapricornSecondTask },
}

impl RocoOptionalCapricornSecondTask {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(task: CapricornSecondTask) -> Self {
        Self::Present { task }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn task_or_default(&self) -> CapricornSecondTask {
        match self {
            Self::Missing => CapricornSecondTask::default(),
            Self::Present { task } => task.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornStarPalaceInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub finish: RocoOptionalI64,
    pub current: RocoOptionalI64,
    pub position: RocoOptionalI64,
    pub second_task: RocoOptionalCapricornSecondTask,
    pub bag_candidates: Vec<CapricornBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub finish: RocoOptionalI64,
    pub current: RocoOptionalI64,
    pub remain: RocoOptionalI64,
    pub price: RocoOptionalI64,
    pub limit: RocoOptionalI64,
    pub progress_percent: RocoOptionalI64,
    pub reward_num: RocoOptionalI64,
    pub tips: RocoOptionalI64,
    pub bag_candidates: Vec<CapricornBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerItemInfo {
    pub id: i64,
    pub count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerPetInfo {
    pub id: i64,
    pub catch_time: i64,
    pub level: i64,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerSharpScorpionInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub light_num: i64,
    pub tail_num: i64,
    pub boss_left_hp: i64,
    pub boss_full_hp: i64,
    pub left_fight_count: i64,
    pub add_hit_level: i64,
    pub today_sum_hit: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub display_item: RocoOptionalDisplayItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerMendShapeInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub left_times: i64,
    pub step: i64,
    pub complete: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerMendShapeBagInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pets: Vec<CancerPetInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerUnsealMemoriesInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub advance: i64,
    pub level: i64,
    pub power: i64,
    pub event: i64,
    pub pass: i64,
    pub finish: i64,
    pub schedule: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerUnsealMemoriesBagInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pets: Vec<CancerPetInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoPetInfo {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub catch_time: RocoOptionalI64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoServeGodInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<VirgoField>,
    pub counters: Vec<VirgoCounter>,
    pub states: Vec<i64>,
    pub pets: Vec<VirgoPetInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoFindHalidomInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<VirgoField>,
    pub counters: Vec<VirgoCounter>,
    pub states: Vec<i64>,
    pub pets: Vec<VirgoPetInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoBellFoxInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<VirgoField>,
    pub counters: Vec<VirgoCounter>,
    pub states: Vec<i64>,
    pub pets: Vec<VirgoPetInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoBellFoxStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub boss_left_hp: i64,
    pub boss_full_hp: i64,
    pub left_fight_count: i64,
    pub add_hit_level: i64,
    pub today_sum_hit: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoBellFoxExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<PiscesField>,
    pub counters: Vec<PiscesCounter>,
    pub lights: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub fights: Vec<i64>,
    pub days: Vec<i64>,
    pub bag_candidates: Vec<PiscesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStartersBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerField {
    pub name: String,
    pub values: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerInfo {
    pub pet: String,
    pub cmd: String,
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<MagicPioneerField>,
    pub rewards: Vec<MagicPioneerRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyCultivationInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub daytimes: RocoOptionalI64,
    pub finish: RocoOptionalI64,
    pub progress: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyEvoInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub branch_type: RocoOptionalI64,
    pub done: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagingFireInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub vip: RocoOptionalI64,
    pub daytimes: RocoOptionalI64,
    pub required_stone_indexes: Vec<i64>,
    pub progress: Vec<i64>,
    pub finish: RocoOptionalI64,
    pub fusion: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBossInfo {
    pub slot: i64,
    pub npc_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub fight_id: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub bosses: Vec<UnicornBossInfo>,
    pub finish: RocoOptionalI64,
    pub start: RocoOptionalI64,
    pub total: RocoOptionalI64,
    pub book: RocoOptionalI64,
    pub cultivation_times: Vec<i64>,
    pub evolution_energy_costs: Vec<i64>,
    pub one_key_diamond_costs: Vec<i64>,
    pub purple_vine_count: RocoOptionalI64,
    pub energy: RocoOptionalI64,
    pub fruit_count: RocoOptionalI64,
    pub increase: RocoOptionalI64,
    pub bag_candidates: Vec<UnicornBagCandidate>,
    pub rewards: Vec<UnicornRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsShopRewardInfo {
    pub reward_id: i64,
    pub reward_kind: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsMonthlySpiritRewardInfo {
    pub month: i64,
    pub reward_index: i64,
    pub spirit_id: i64,
    pub ticket_cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub month: RocoOptionalI64,
    pub map: RocoOptionalI64,
    pub position_1based: RocoOptionalI64,
    pub times: RocoOptionalI64,
    pub ticket: RocoOptionalI64,
    pub used_tool_index: RocoOptionalI64,
    pub need_item_index: RocoOptionalI64,
    pub add: RocoOptionalI64,
    pub point: RocoOptionalI64,
    pub boxes: Vec<i64>,
    pub tools: Vec<i64>,
    pub tool_shop_indexes: Vec<i64>,
    pub tool_shop_flags: Vec<i64>,
    pub pass_boxes: Vec<i64>,
    pub tool_costs: Vec<i64>,
    pub event_item_counts: Vec<i64>,
    pub shop_rewards: Vec<FourSeasonsShopRewardInfo>,
    pub monthly_spirit_rewards: Vec<FourSeasonsMonthlySpiritRewardInfo>,
    pub rewards: Vec<FourSeasonsRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTearRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTearInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub buy: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub count_down: RocoOptionalI64,
    pub tear_state: RocoOptionalI64,
    pub rewards: Vec<DiamondTearRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalBattleInfo {
    pub battle_index: i64,
    pub fight_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalIceCrystalBattleInfo {
    Missing,
    Present { battle: IceCrystalBattleInfo },
}

impl RocoOptionalIceCrystalBattleInfo {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(battle: IceCrystalBattleInfo) -> Self {
        Self::Present { battle }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn battle_or_default(&self) -> IceCrystalBattleInfo {
        match self {
            Self::Missing => IceCrystalBattleInfo {
                battle_index: 0,
                fight_id: 0,
            },
            Self::Present { battle } => battle.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub progress: RocoOptionalI64,
    pub battle_times: RocoOptionalI64,
    pub battle_index: RocoOptionalI64,
    pub get_times: RocoOptionalI64,
    pub add: RocoOptionalI64,
    pub item_counts: Vec<i64>,
    pub crystal_counts: Vec<i64>,
    pub item_costs: Vec<i64>,
    pub one_key_diamond_costs: Vec<i64>,
    pub current_battle: RocoOptionalIceCrystalBattleInfo,
    pub bag_candidates: Vec<IceCrystalBagCandidate>,
    pub rewards: Vec<IceCrystalRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub catch_time: i64,
    pub condition_code: i64,
    pub condition_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub candidates: Vec<MultiEvolutionCandidate>,
    pub rewards: Vec<MultiEvolutionRewardItem>,
    pub pet_id: RocoOptionalI64,
    pub result_side: RocoOptionalI64,
    pub item_id: RocoOptionalI64,
    pub count: i64,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSourceInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub battle: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub time: RocoOptionalI64,
    pub increase: RocoOptionalI64,
    pub water: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiresWillInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub schedule: RocoOptionalI64,
    pub num: RocoOptionalI64,
    pub fire: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatheSunInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ThreeStartersField>,
    pub counters: Vec<ThreeStartersCounter>,
    pub rewards: Vec<ThreeStartersRewardItem>,
    pub bag_candidates: Vec<ThreeStartersBagCandidate>,
    pub battle: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub time: RocoOptionalI64,
    pub num: RocoOptionalI64,
    pub act: RocoOptionalI64,
    pub times: RocoOptionalI64,
    pub sun: RocoOptionalI64,
    pub add: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusScore {
    pub score_index: i64,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusStarPicture {
    pub picture_index: i64,
    pub is_in: i64,
    pub progress: i64,
    pub finish: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioReward {
    pub reward_index: i64,
    pub reward_type: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<ScorpioField>,
    pub counters: Vec<ScorpioCounter>,
    pub counts: Vec<i64>,
    pub rewards: Vec<ScorpioReward>,
    pub bag_candidates: Vec<ScorpioBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesReward {
    pub reward_index: i64,
    pub reward_type: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AriesField>,
    pub counters: Vec<AriesCounter>,
    pub rewards: Vec<AriesReward>,
    pub bag_candidates: Vec<AriesBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub boss_left_hp: i64,
    pub left_fight_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesThirdExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LibraField>,
    pub counters: Vec<LibraCounter>,
    pub bag_candidates: Vec<LibraBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LibraField>,
    pub counters: Vec<LibraCounter>,
    pub bag_candidates: Vec<LibraBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LibraField>,
    pub counters: Vec<LibraCounter>,
    pub bag_candidates: Vec<LibraBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraThirdStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub boss_left_hp: i64,
    pub left_fight_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraThirdExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LeoField>,
    pub counters: Vec<LeoCounter>,
    pub bag_candidates: Vec<LeoBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LeoField>,
    pub counters: Vec<LeoCounter>,
    pub bag_candidates: Vec<LeoBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<LeoField>,
    pub counters: Vec<LeoCounter>,
    pub bag_candidates: Vec<LeoBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoFirstStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub boss_left_hp: i64,
    pub left_fight_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoFirstExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusRewardItem {
    pub item_index: i64,
    pub item_id: i64,
    pub count: i64,
    pub item_type: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<AquariusField>,
    pub counters: Vec<AquariusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<AquariusBagCandidate>,
    pub reward_items: Vec<AquariusRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondStatusInfo {
    pub result_code: i64,
    pub message: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub boss_left_hp: i64,
    pub boss_full_hp: i64,
    pub left_fight_count: i64,
    pub add_hit_level: i64,
    pub today_sum_hit: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusSecondExchangeInfo {
    pub result_code: i64,
    pub message: String,
    pub item: RocoOptionalDisplayItem,
    pub light_num: i64,
    pub tail_num: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderInfo {
    pub win_nums: i64,
    pub win_point: i64,
    pub spirits: Vec<LadderSpiritInfo>,
    pub spirit_info_flag: i64,
    pub backup_spirits: Vec<LadderSpiritInfo>,
    pub left_time: i64,
    pub rank_level: i64,
    pub left_play_times: i64,
    pub left_reward_times: i64,
    pub season_reward_flag: i64,
    pub fight_days: i64,
    pub next_win_point: i64,
    pub day_quests: Vec<LadderQuestInfo>,
    pub show_achievement: i64,
    pub achievement_list: Vec<i64>,
    pub ban_list: Vec<i64>,
    pub records: Vec<LadderFightRecord>,
    pub season: i64,
    pub all_nums: i64,
    pub left_play_times_df: i64,
    pub win_point_df: i64,
    pub win_nums_df: i64,
    pub all_nums_df: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderSpiritInfo {
    pub pet_id: i64,
    pub pet_level: i64,
    pub equipment_ids: Vec<i64>,
    pub now_hp: i64,
    pub full_hp: i64,
    pub skin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderQuestInfo {
    pub status: i64,
    pub id: i64,
    pub give_up: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderFightRecord {
    pub win: i64,
    pub score: i64,
    pub round: i64,
    pub my_point: i64,
    pub other_point: i64,
    pub my_spirits: Vec<i64>,
    pub other_spirits: Vec<i64>,
    pub fight_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderRankUser {
    pub uin: i64,
    pub name: String,
    pub win_nums: i64,
    pub win_point: i64,
    pub rank_num: i64,
    pub medals: Vec<i64>,
    pub achievement_num: i64,
    pub show_achievement: i64,
    pub rank_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderRankInfo {
    pub users: Vec<LadderRankUser>,
    pub rank_level: i64,
    pub rank_change: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeLadderRank {
    pub rank: i64,
    pub small_rank: i64,
    pub star: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderSpiritInfo {
    pub spirit_id: i64,
    pub level: i64,
    pub current_hp: i64,
    pub max_hp: i64,
    pub attribute: i64,
    pub eligibility: String,
    pub eligibility_code: i64,
    pub skin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderFightRecord {
    pub win: i64,
    pub round: i64,
    pub my_spirits: Vec<i64>,
    pub opponent_spirits: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderInfo {
    pub season: i64,
    pub win_count: i64,
    pub battle_count: i64,
    pub left_play_times: i64,
    pub proxy: i64,
    pub grade: i64,
    pub current_rank: TypeLadderRank,
    pub max_rank: TypeLadderRank,
    pub allowed_attributes: Vec<i64>,
    pub banned_spirit_ids: Vec<i64>,
    pub spirits: Vec<TypeLadderSpiritInfo>,
    pub records: Vec<TypeLadderFightRecord>,
    pub season_reward_available: bool,
    pub season_reward_flag: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeLadderRankUser {
    pub uin: i64,
    pub name: String,
    pub win_count: i64,
    pub battle_count: i64,
    pub rank_num: i64,
    pub score: TypeLadderRank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalTypeLadderRankUser {
    Missing,
    Present { user: TypeLadderRankUser },
}

impl RocoOptionalTypeLadderRankUser {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(user: TypeLadderRankUser) -> Self {
        Self::Present { user }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn user_or_default(&self) -> TypeLadderRankUser {
        match self {
            Self::Missing => TypeLadderRankUser::default(),
            Self::Present { user } => user.clone(),
        }
    }
}

impl From<Option<TypeLadderRankUser>> for RocoOptionalTypeLadderRankUser {
    fn from(value: Option<TypeLadderRankUser>) -> Self {
        value.map(Self::present).unwrap_or(Self::Missing)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderRankInfo {
    pub my_info: RocoOptionalTypeLadderRankUser,
    pub users: Vec<TypeLadderRankUser>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSpiritState {
    pub position: i64,
    pub spirit_id: i64,
    pub level: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSideState {
    pub uin: i64,
    pub active_position: i64,
    pub spirits: Vec<CombatSpiritState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    pub round: i64,
    pub weather: i64,
    pub weather_round: i64,
    pub my_side: CombatSideState,
    pub rival_side: CombatSideState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActionSnapshot {
    pub is_finished: bool,
    pub state: CombatState,
    pub actions: CombatActions,
}

/// Standard result shape for operation-style `try_*` APIs.
///
/// `try_*` functions should not raise expected business failures such as
/// unavailable actions or server rejections. They should return `ok = false`
/// with a non-zero `code` and a readable `message`. Programming errors such as
/// invalid argument types may still be raised by the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
}

impl ActionResult {
    pub fn ok() -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
        }
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        Self {
            ok: false,
            code: 2,
            message: error.message(),
            error: Some(error.info()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatActionResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub ack_received: bool,
    pub combat_finished: bool,
    pub next_action_ready: bool,
}

impl CombatActionResult {
    pub fn ok(combat_finished: bool, next_action_ready: bool) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            ack_received: true,
            combat_finished,
            next_action_ready,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            error: None,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn unavailable_error(error: impl Into<RocoError>) -> Self {
        let error = error.into();
        Self {
            ok: false,
            code: 1,
            message: error.message(),
            error: Some(error.info()),
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            error: None,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn failed_error(error: impl Into<RocoError>) -> Self {
        let error = error.into();
        Self {
            ok: false,
            code: 2,
            message: error.message(),
            error: Some(error.info()),
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }

    pub fn from_action_result(action: ActionResult) -> Self {
        Self {
            ok: false,
            code: action.code,
            message: action.message,
            error: action.error,
            ack_received: false,
            combat_finished: false,
            next_action_ready: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameRewardItem {
    pub id: i64,
    pub count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameExtraField {
    pub key: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameSubmitResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub game_id: i64,
    pub score: i64,
    pub game_type: i64,
    pub items: Vec<MiniGameRewardItem>,
    pub extra_fields: Vec<MiniGameExtraField>,
}

impl MiniGameSubmitResult {
    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 2,
            message: message.into(),
            game_id: 0,
            score: 0,
            game_type: 0,
            items: Vec::new(),
            extra_fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniGameSubmitTryResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub error: Option<RocoErrorInfo>,
    pub result: MiniGameSubmitResult,
}

impl MiniGameSubmitTryResult {
    pub const CODE_NETWORK_ERROR: i64 = 1001;

    pub fn ok(result: MiniGameSubmitResult) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            error: None,
            result,
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self::failed_with_code(2, message)
    }

    pub fn network_error_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self::failed_with_code_and_error(Self::CODE_NETWORK_ERROR, message, Some(error.info()))
    }

    pub fn failed_with_error(error: RocoError) -> Self {
        let message = error.message();
        Self::failed_with_code_and_error(2, message, Some(error.info()))
    }

    fn failed_with_code(code: i64, message: impl Into<String>) -> Self {
        Self::failed_with_code_and_error(code, message, None)
    }

    fn failed_with_code_and_error(
        code: i64,
        message: impl Into<String>,
        error: Option<RocoErrorInfo>,
    ) -> Self {
        let message = message.into();
        Self {
            ok: false,
            code,
            error,
            result: MiniGameSubmitResult::failed(message.clone()),
            message,
        }
    }
}

/// Result returned after refreshing a spirit's talent values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentRefreshResult {
    pub position: i64,
    pub pa_ability_old: i64,
    pub pd_ability_old: i64,
    pub ma_ability_old: i64,
    pub md_ability_old: i64,
    pub sp_ability_old: i64,
    pub hp_ability_old: i64,
    pub pa_ability_new: i64,
    pub pd_ability_new: i64,
    pub ma_ability_new: i64,
    pub md_ability_new: i64,
    pub sp_ability_new: i64,
    pub hp_ability_new: i64,
    pub pa_talent_old: i64,
    pub pd_talent_old: i64,
    pub ma_talent_old: i64,
    pub md_talent_old: i64,
    pub sp_talent_old: i64,
    pub hp_talent_old: i64,
    pub pa_talent_new: i64,
    pub pd_talent_new: i64,
    pub ma_talent_new: i64,
    pub md_talent_new: i64,
    pub sp_talent_new: i64,
    pub hp_talent_new: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftItemRequirement {
    pub item_id: i64,
    pub count: i64,
    pub need: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftOption {
    pub blood_index: i64,
    pub talent_type: i64,
    pub talent_name: String,
    pub talent_description: String,
    pub awakened: bool,
    pub required_items: Vec<BloodGiftItemRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftInfo {
    pub result_code: i64,
    pub message: String,
    pub position: i64,
    pub equipped_index: i64,
    pub options: Vec<BloodGiftOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendNatureCandidate {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub level: i64,
    pub personality: i64,
    pub personality_name: String,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendNatureInfo {
    pub result_code: i64,
    pub message: String,
    pub eligible_spirit_ids: Vec<i64>,
    pub candidates: Vec<AmendNatureCandidate>,
    pub new_personality: i64,
    pub new_personality_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritEquipmentInfo {
    pub server_id: i64,
    pub catch_time: i64,
    pub base_attr: i64,
    pub base_value: i64,
    pub special_attr: i64,
    pub special_value: i64,
    pub spirit_id: RocoOptionalI64,
    pub spirit_catch_time: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritEquipmentBagInfo {
    pub equipment_count: i64,
    pub all_num: i64,
    pub need: i64,
    pub equipments: Vec<SpiritEquipmentInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorGroundInfo {
    pub ground_id: i64,
    pub ground_status: i64,
    pub seed: i64,
    pub plant_status: i64,
    pub current_time: i64,
    pub total_time: i64,
    pub total_produce: i64,
    pub left_produce: i64,
    pub has_grass: bool,
    pub has_insect: bool,
    pub has_fruit: bool,
    pub season: i64,
    pub left_row_times: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorInfo {
    pub qq_uin: i64,
    pub manor_level: i64,
    pub manor_exp: i64,
    pub gold_mass_num: i64,
    pub gold_money_num: i64,
    pub guide_type: i64,
    pub pet_status: i64,
    pub scarecrow_exp: i64,
    pub scarecrow_level: i64,
    pub scarecrow_id: i64,
    pub home_id: i64,
    pub parasol_id: i64,
    pub beautify_id: i64,
    pub billboard_id: i64,
    pub scarecrow_ever_play: bool,
    pub scarecrow_next_exp: i64,
    pub scarecrow_gift_gotten: bool,
    pub proficiency_a: i64,
    pub proficiency_a_exp: i64,
    pub proficiency_a_exp_pre: i64,
    pub proficiency_a_exp_next: i64,
    pub proficiency_b: i64,
    pub proficiency_b_exp: i64,
    pub proficiency_b_exp_pre: i64,
    pub proficiency_b_exp_next: i64,
    pub proficiency_c: i64,
    pub proficiency_c_exp: i64,
    pub proficiency_c_exp_pre: i64,
    pub proficiency_c_exp_next: i64,
    pub steal_state: i64,
    pub gift_status_a: Vec<i64>,
    pub gift_status_b: Vec<i64>,
    pub gift_status_c: Vec<i64>,
    pub grounds: Vec<ManorGroundInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorItemCount {
    pub item_id: i64,
    pub item_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorRewardInfo {
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorSowResult {
    pub exp: i64,
    pub ground: ManorGroundInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorReapResult {
    pub qq_uin: i64,
    pub seed_id: i64,
    pub result: i64,
    pub exp: i64,
    pub fruit_num: i64,
    pub ground: ManorGroundInfo,
    pub event_id: i64,
    pub rewards: Vec<ManorRewardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorUprootResult {
    pub ground: ManorGroundInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorWeedResult {
    pub qq_uin: i64,
    pub exp: i64,
    pub ground: ManorGroundInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorFertilizerResult {
    pub can_fertilizer: bool,
    pub deduce_time_in_second: i64,
    pub fertilizer: i64,
    pub uin: i64,
    pub ground: ManorGroundInfo,
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
pub struct LadderQuestConfigEntry {
    pub id: i64,
    pub diff: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderSpiritCostEntry {
    pub spirit_id: i64,
    pub cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderMatchConfig {
    pub match_rewards: Vec<String>,
    pub win_rewards: Vec<String>,
    pub season_rewards: Vec<String>,
    pub task0_descriptions: Vec<LadderQuestConfigEntry>,
    pub task1_descriptions: Vec<LadderQuestConfigEntry>,
    pub spirit_costs: Vec<LadderSpiritCostEntry>,
    pub limit_spirits: Vec<i64>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticTalentInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
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
pub struct SpiritBookEntry {
    pub id: i64,
    pub starred: bool,
    pub unknown: bool,
    pub newed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookGroup {
    pub template_id: i64,
    pub spirits: Vec<SpiritBookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookSummary {
    pub id: i64,
    pub name: String,
    pub is_new: bool,
    pub has_cover: bool,
    pub background: String,
    pub page_idx: i64,
    pub spirit_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookInfo {
    pub id: i64,
    pub name: String,
    pub is_new: bool,
    pub has_cover: bool,
    pub background: String,
    pub page_idx: i64,
    pub groups: Vec<SpiritBookGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookStates {
    pub uin: i64,
    pub count: i64,
    pub states: Vec<i64>,
}

impl SpiritBookStates {
    pub fn spirit_state(&self, spirit_id: i64) -> SpiritBookState {
        let state_code = spirit_id
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| self.states.get(index).copied())
            .unwrap_or(0);
        SpiritBookState::from_code(state_code)
    }

    pub fn spirit_owned(&self, spirit_id: i64) -> bool {
        self.spirit_state(spirit_id).is_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookSpiritState {
    pub spirit_id: i64,
    pub state: i64,
    pub owned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiritBookState {
    Unknown,
    Found,
    Caught,
    Released,
}

impl SpiritBookState {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Found,
            2 => Self::Caught,
            3 => Self::Released,
            _ => Self::Unknown,
        }
    }

    pub const fn code(self) -> i64 {
        match self {
            Self::Unknown => 0,
            Self::Found => 1,
            Self::Caught => 2,
            Self::Released => 3,
        }
    }

    pub const fn is_owned(self) -> bool {
        matches!(self, Self::Caught | Self::Released)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritEvolutionEdge {
    pub target_id: i64,
    pub kind: i64,
}

impl StaticSpiritEvolutionEdge {
    pub const KIND_ORDINARY: i64 = 1;
    pub const KIND_DISPLAY: i64 = 2;

    pub fn ordinary(target_id: i64) -> Self {
        Self {
            target_id,
            kind: Self::KIND_ORDINARY,
        }
    }

    pub fn display(target_id: i64) -> Self {
        Self {
            target_id,
            kind: Self::KIND_DISPLAY,
        }
    }
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
    pub evolution_edges: Vec<StaticSpiritEvolutionEdge>,
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

impl StaticSpiritInfo {
    pub fn unknown(spirit_id: i64) -> Self {
        Self {
            id: spirit_id,
            name: format!("未知({spirit_id})"),
            description: String::new(),
            features: Vec::new(),
            group: Vec::new(),
            src: String::new(),
            avatar: String::new(),
            icon_src: String::new(),
            preview_src: String::new(),
            move_speed: 0,
            height: String::new(),
            weight: String::new(),
            color: String::new(),
            interest: String::new(),
            habitat: String::new(),
            evolution: Vec::new(),
            catchrate: 0,
            boss_phyle: String::new(),
            boss_reward: String::new(),
            scene_id: 0,
            condition: String::new(),
            require_level: String::new(),
            wg: 0,
            mg: 0,
            mk: 0,
            sm: 0,
            sd: 0,
            fy: 0,
            reward: 0,
            evolution_form_id: 0,
            evolution_to_ids: Vec::new(),
            evolution_edges: Vec::new(),
            get_form: String::new(),
            state: 0,
            start_time: String::new(),
            end_time: String::new(),
            first_id: 0,
            propo_level: 0,
            is_in_book: false,
            skinnum: 0,
            exp_type: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSpiritInfoLookupResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub result: StaticSpiritInfo,
}

impl StaticSpiritInfoLookupResult {
    pub fn ok(result: StaticSpiritInfo) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            result,
        }
    }

    pub fn not_found(spirit_id: i64, message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
            result: StaticSpiritInfo::unknown(spirit_id),
        }
    }
}
