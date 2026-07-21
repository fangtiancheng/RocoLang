use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalDisplayItem, RocoOptionalI64, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, LibraField, name, value);
    register_getters!(engine, LibraCounter, name, current, limit);
    register_getters!(
        engine,
        LibraBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<LibraFirstInfo>("LibraFirstInfo");
    register_getters!(
        engine,
        LibraFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<LibraSecondInfo>("LibraSecondInfo");
    register_getters!(
        engine,
        LibraSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<LibraThirdInfo>("LibraThirdInfo");
    register_getters!(
        engine,
        LibraThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut LibraThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut LibraThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("bag_candidates", |value: &mut LibraThirdInfo| {
        to_array(&value.bag_candidates)
    });
    register_getters!(
        engine,
        LibraThirdStatusInfo,
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
        LibraThirdExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
