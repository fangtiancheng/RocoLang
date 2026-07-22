use serde::{Deserialize, Serialize};

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
pub struct ManorPlantStatus {
    pub uin: i64,
    pub has_fruit: bool,
    pub has_insect: bool,
    pub has_grass: bool,
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
pub struct ManorReclaimResult {
    pub ground_id: i64,
    pub result: i64,
    pub exp: i64,
    pub rewards: Vec<ManorRewardInfo>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorStrawmanPlayResult {
    pub qq_uin: i64,
    pub magic_id: i64,
    pub money: i64,
    pub ground_id: i64,
    pub rewards: Vec<ManorRewardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorStrawmanRewardResult {
    pub total_exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorCocoTreeStatus {
    pub growth_value: i64,
    pub level: i64,
    pub can_pick_fruit: bool,
    pub watered: bool,
    pub time_past: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorCocoTreeReward {
    pub item_id: i64,
    pub item_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorCocoTreeFeedResult {
    pub status: ManorCocoTreeStatus,
    pub rewards: Vec<ManorCocoTreeReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorFriendCocoTreeStatus {
    pub feed_type: i64,
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorFriendCocoTreeFeedResult {
    pub manor_exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorFriendSummary {
    pub uin: i64,
    pub score: i64,
    pub plant_score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManorFriendDetail {
    pub uin: i64,
    pub vip_level: i64,
    pub uin_code: String,
    pub version: i64,
    pub roco_nickname: String,
    pub qq_nickname: String,
    pub qq_icon_url: String,
}
