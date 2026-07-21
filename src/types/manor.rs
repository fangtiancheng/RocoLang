use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        ManorGroundInfo,
        ground_id,
        ground_status,
        seed,
        plant_status,
        current_time,
        total_time,
        total_produce,
        left_produce,
        has_grass,
        has_insect,
        has_fruit,
        season,
        left_row_times
    );
    register_getters!(
        engine,
        ManorInfo,
        qq_uin,
        manor_level,
        manor_exp,
        gold_mass_num,
        gold_money_num,
        guide_type,
        pet_status,
        scarecrow_exp,
        scarecrow_level,
        scarecrow_id,
        home_id,
        parasol_id,
        beautify_id,
        billboard_id,
        scarecrow_ever_play,
        scarecrow_next_exp,
        scarecrow_gift_gotten,
        proficiency_a,
        proficiency_a_exp,
        proficiency_a_exp_pre,
        proficiency_a_exp_next,
        proficiency_b,
        proficiency_b_exp,
        proficiency_b_exp_pre,
        proficiency_b_exp_next,
        proficiency_c,
        proficiency_c_exp,
        proficiency_c_exp_pre,
        proficiency_c_exp_next,
        steal_state
    );
    engine.register_get("gift_status_a", |value: &mut ManorInfo| {
        to_array(&value.gift_status_a)
    });
    engine.register_get("gift_status_b", |value: &mut ManorInfo| {
        to_array(&value.gift_status_b)
    });
    engine.register_get("gift_status_c", |value: &mut ManorInfo| {
        to_array(&value.gift_status_c)
    });
    engine.register_get("grounds", |value: &mut ManorInfo| to_array(&value.grounds));
    register_getters!(engine, ManorItemCount, item_id, item_count);
    register_getters!(
        engine,
        ManorPlantStatus,
        uin,
        has_fruit,
        has_insect,
        has_grass
    );
    register_getters!(engine, ManorRewardInfo, item_id, count);
    register_getters!(engine, ManorSowResult, exp);
    engine.register_get("ground", |value: &mut ManorSowResult| value.ground.clone());
    register_getters!(engine, ManorReclaimResult, ground_id, result, exp);
    engine.register_get("rewards", |value: &mut ManorReclaimResult| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        ManorReapResult,
        qq_uin,
        seed_id,
        result,
        exp,
        fruit_num,
        event_id
    );
    engine.register_get("ground", |value: &mut ManorReapResult| value.ground.clone());
    engine.register_get("rewards", |value: &mut ManorReapResult| {
        to_array(&value.rewards)
    });
    engine.register_get("ground", |value: &mut ManorUprootResult| {
        value.ground.clone()
    });
    register_getters!(engine, ManorWeedResult, qq_uin, exp);
    engine.register_get("ground", |value: &mut ManorWeedResult| value.ground.clone());
    register_getters!(
        engine,
        ManorFertilizerResult,
        can_fertilizer,
        deduce_time_in_second,
        fertilizer,
        uin
    );
    engine.register_get("ground", |value: &mut ManorFertilizerResult| {
        value.ground.clone()
    });
    register_getters!(
        engine,
        ManorStrawmanPlayResult,
        qq_uin,
        magic_id,
        money,
        ground_id
    );
    engine.register_get("rewards", |value: &mut ManorStrawmanPlayResult| {
        to_array(&value.rewards)
    });
    register_getters!(engine, ManorStrawmanRewardResult, total_exp);
    register_getters!(
        engine,
        ManorCocoTreeStatus,
        growth_value,
        level,
        can_pick_fruit,
        watered,
        time_past
    );
    register_getters!(engine, ManorCocoTreeReward, item_id, item_count);
    engine.register_get("status", |value: &mut ManorCocoTreeFeedResult| {
        value.status.clone()
    });
    engine.register_get("rewards", |value: &mut ManorCocoTreeFeedResult| {
        to_array(&value.rewards)
    });
    register_getters!(engine, ManorFriendCocoTreeStatus, feed_type, level);
    register_getters!(engine, ManorFriendCocoTreeFeedResult, manor_exp);
    register_getters!(engine, ManorFriendSummary, uin, score, plant_score);
    register_getters!(
        engine,
        ManorFriendDetail,
        uin,
        vip_level,
        uin_code,
        version,
        roco_nickname,
        qq_nickname,
        qq_icon_url
    );
}
