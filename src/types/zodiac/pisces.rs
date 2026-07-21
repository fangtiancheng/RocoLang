use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, PiscesField, name, value);
    register_getters!(engine, PiscesCounter, name, current, limit);
    register_getters!(
        engine,
        PiscesBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_pisces_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("lights", |value: &mut $ty| to_array(&value.lights));
            engine.register_get("exchanges", |value: &mut $ty| to_array(&value.exchanges));
            engine.register_get("fights", |value: &mut $ty| to_array(&value.fights));
            engine.register_get("days", |value: &mut $ty| to_array(&value.days));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }

    register_pisces_info!(PiscesFirstInfo, "PiscesFirstInfo");
    register_pisces_info!(PiscesSecondInfo, "PiscesSecondInfo");
    register_pisces_info!(PiscesThirdInfo, "PiscesThirdInfo");
}
