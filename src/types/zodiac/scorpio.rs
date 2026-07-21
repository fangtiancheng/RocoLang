use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, ScorpioField, name, value);
    register_getters!(engine, ScorpioCounter, name, current, limit);
    register_getters!(
        engine,
        ScorpioReward,
        reward_index,
        reward_type,
        reward_count
    );
    register_getters!(
        engine,
        ScorpioBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<ScorpioFirstInfo>("ScorpioFirstInfo");
    register_getters!(
        engine,
        ScorpioFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioFirstInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioFirstInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<ScorpioSecondInfo>("ScorpioSecondInfo");
    register_getters!(
        engine,
        ScorpioSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioSecondInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioSecondInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<ScorpioThirdInfo>("ScorpioThirdInfo");
    register_getters!(
        engine,
        ScorpioThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut ScorpioThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut ScorpioThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("counts", |value: &mut ScorpioThirdInfo| {
        to_array(&value.counts)
    });
    engine.register_get("rewards", |value: &mut ScorpioThirdInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut ScorpioThirdInfo| {
        to_array(&value.bag_candidates)
    });
}
