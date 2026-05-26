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
pub struct BattleResultQueryResult {
    pub ok: bool,
    pub code: i64,
    pub message: String,
    pub result: Option<BattleResult>,
}

impl BattleResultQueryResult {
    pub fn ok(result: BattleResult) -> Self {
        Self {
            ok: true,
            code: 0,
            message: String::new(),
            result: Some(result),
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: 1,
            message: message.into(),
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
    pub has_top: bool,
    pub top: StarTowerTop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerStorey {
    pub storey_index: i64,
    pub first: i64,
    pub can_quick_fight: bool,
    pub nodes: Vec<StarTowerNode>,
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
    pub has_team: bool,
    pub team: CapricornTeamSnapshot,
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
pub struct CapricornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
    pub has_finish: bool,
    pub finish: i64,
    pub has_current: bool,
    pub current: i64,
    pub has_position: bool,
    pub position: i64,
    pub has_second_task: bool,
    pub second_task: CapricornSecondTask,
    pub has_remain: bool,
    pub remain: i64,
    pub has_price: bool,
    pub price: i64,
    pub has_limit: bool,
    pub limit: i64,
    pub has_progress_percent: bool,
    pub progress_percent: i64,
    pub has_reward_num: bool,
    pub reward_num: i64,
    pub has_tips: bool,
    pub tips: i64,
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
pub struct CancerInfo {
    pub kind: String,
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
    pub light_num: i64,
    pub tail_num: i64,
    pub boss_left_hp: i64,
    pub boss_full_hp: i64,
    pub left_fight_count: i64,
    pub add_hit_level: i64,
    pub today_sum_hit: i64,
    pub exchange_count0: i64,
    pub exchange_count1: i64,
    pub has_display_item: bool,
    pub display_item: CancerItemInfo,
    pub left_times: i64,
    pub step: i64,
    pub complete: i64,
    pub advance: i64,
    pub level: i64,
    pub power: i64,
    pub event: i64,
    pub pass: i64,
    pub finish: i64,
    pub schedule: i64,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirgoInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub has_item: bool,
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiscesInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorpioInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriesInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub has_item: bool,
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub has_item: bool,
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeoInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub has_item: bool,
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
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
    pub spirit_id: i64,
    pub has_bag_index: bool,
    pub bag_index: i64,
    pub has_catch_time: bool,
    pub catch_time: i64,
    pub has_level: bool,
    pub level: i64,
    pub has_need_money: bool,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusRewardItem {
    pub item_index: i64,
    pub item_id: i64,
    pub count: i64,
    pub has_item_type: bool,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquariusInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: String,
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
    pub has_item: bool,
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
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
pub struct TypeLadderRankInfo {
    pub my_info: Option<TypeLadderRankUser>,
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
    pub spirit_id: i64,
    pub spirit_catch_time: i64,
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
