use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalDisplayItem, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, CancerItemInfo, id, count, item_type);
    register_getters!(engine, CancerPetInfo, id, catch_time, level, need_money);
    register_getters!(
        engine,
        CancerSharpScorpionInfo,
        result_code,
        message,
        request_context,
        light_num,
        tail_num,
        boss_left_hp,
        boss_full_hp,
        left_fight_count,
        add_hit_level,
        today_sum_hit,
        exchange_count0,
        exchange_count1,
        display_item,
    );
    register_getters!(
        engine,
        CancerMendShapeInfo,
        result_code,
        message,
        request_context,
        left_times,
        step,
        complete,
    );
    register_getters!(
        engine,
        CancerMendShapeBagInfo,
        result_code,
        message,
        request_context,
    );
    engine.register_get("pets", |value: &mut CancerMendShapeBagInfo| {
        to_array(&value.pets)
    });
    register_getters!(
        engine,
        CancerUnsealMemoriesInfo,
        result_code,
        message,
        request_context,
        advance,
        level,
        power,
        event,
        pass,
        finish,
        schedule,
    );
    register_getters!(
        engine,
        CancerUnsealMemoriesBagInfo,
        result_code,
        message,
        request_context,
    );
    engine.register_get("pets", |value: &mut CancerUnsealMemoriesBagInfo| {
        to_array(&value.pets)
    });
}
