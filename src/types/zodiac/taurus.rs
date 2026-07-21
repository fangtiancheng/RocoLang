use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext};

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
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaurusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<TaurusField>,
    pub counters: Vec<TaurusCounter>,
    pub item_counts: Vec<i64>,
    pub states: Vec<i64>,
    pub bag_candidates: Vec<TaurusBagCandidate>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, TaurusField, name, value);
    register_getters!(engine, TaurusCounter, name, current, limit);
    register_getters!(
        engine,
        TaurusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_taurus_info {
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
        };
    }
    register_taurus_info!(TaurusFirstInfo, "TaurusFirstInfo");
    register_taurus_info!(TaurusSecondInfo, "TaurusSecondInfo");
    register_taurus_info!(TaurusThirdInfo, "TaurusThirdInfo");
}
