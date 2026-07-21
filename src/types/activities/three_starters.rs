use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ThreeStartersField, name, value);
    register_getters!(engine, ThreeStartersCounter, name, current, limit);
    register_getters!(
        engine,
        ThreeStartersRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        ThreeStartersBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        WaterSourceInfo,
        result_code,
        message,
        request_context,
        battle,
        schedule,
        time,
        increase
    );
    engine.register_get("fields", |value: &mut WaterSourceInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut WaterSourceInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut WaterSourceInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut WaterSourceInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("water", |value: &mut WaterSourceInfo| {
        to_array(&value.water)
    });
    register_getters!(
        engine,
        FiresWillInfo,
        result_code,
        message,
        request_context,
        schedule,
        num
    );
    engine.register_get("fields", |value: &mut FiresWillInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut FiresWillInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut FiresWillInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut FiresWillInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("fire", |value: &mut FiresWillInfo| to_array(&value.fire));
    register_getters!(
        engine,
        BatheSunInfo,
        result_code,
        message,
        request_context,
        battle,
        schedule,
        time,
        num,
        act,
        times,
        sun,
        add
    );
    engine.register_get("fields", |value: &mut BatheSunInfo| to_array(&value.fields));
    engine.register_get("counters", |value: &mut BatheSunInfo| {
        to_array(&value.counters)
    });
    engine.register_get("rewards", |value: &mut BatheSunInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut BatheSunInfo| {
        to_array(&value.bag_candidates)
    });
}
