use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalDisplayItem, RocoOptionalI64, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, AriesField, name, value);
    register_getters!(engine, AriesCounter, name, current, limit);
    register_getters!(engine, AriesReward, reward_index, reward_type, reward_count);
    register_getters!(
        engine,
        AriesBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_aries_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("rewards", |value: &mut $ty| to_array(&value.rewards));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }
    register_aries_info!(AriesFirstInfo, "AriesFirstInfo");
    register_aries_info!(AriesSecondInfo, "AriesSecondInfo");
    register_aries_info!(AriesThirdInfo, "AriesThirdInfo");
    register_getters!(
        engine,
        AriesThirdStatusInfo,
        result_code,
        message,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
        boss_left_hp,
        left_fight_count,
    );
    register_getters!(
        engine,
        AriesThirdExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
