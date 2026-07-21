use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalDisplayItem, RocoOptionalI64, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, AquariusField, name, value);
    register_getters!(engine, AquariusCounter, name, current, limit);
    register_getters!(
        engine,
        AquariusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        AquariusRewardItem,
        item_index,
        item_id,
        count,
        item_type
    );

    macro_rules! register_aquarius_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("item_counts", |value: &mut $ty| {
                to_array(&value.item_counts)
            });
            engine.register_get("states", |value: &mut $ty| to_array(&value.states));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
            engine.register_get("reward_items", |value: &mut $ty| {
                to_array(&value.reward_items)
            });
        };
    }
    register_aquarius_info!(AquariusFirstInfo, "AquariusFirstInfo");
    register_aquarius_info!(AquariusSecondInfo, "AquariusSecondInfo");
    register_aquarius_info!(AquariusThirdInfo, "AquariusThirdInfo");
    register_getters!(
        engine,
        AquariusSecondStatusInfo,
        result_code,
        message,
        light_num,
        tail_num,
        boss_left_hp,
        boss_full_hp,
        left_fight_count,
        add_hit_level,
        today_sum_hit,
        exchange_count0,
        exchange_count1,
    );
    register_getters!(
        engine,
        AquariusSecondExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
