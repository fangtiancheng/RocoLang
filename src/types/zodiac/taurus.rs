use serde::{Deserialize, Serialize};

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
