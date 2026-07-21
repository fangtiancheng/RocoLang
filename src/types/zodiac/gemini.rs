use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<GeminiField>,
    pub counters: Vec<GeminiCounter>,
    pub scores: Vec<i64>,
    pub sun_scores: Vec<i64>,
    pub moon_scores: Vec<i64>,
    pub rewards: Vec<GeminiRewardItem>,
    pub bag_candidates: Vec<GeminiBagCandidate>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, GeminiField, name, value);
    register_getters!(engine, GeminiCounter, name, current, limit);
    register_getters!(
        engine,
        GeminiRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        GeminiBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_gemini_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("scores", |value: &mut $ty| to_array(&value.scores));
            engine.register_get("sun_scores", |value: &mut $ty| to_array(&value.sun_scores));
            engine.register_get("moon_scores", |value: &mut $ty| {
                to_array(&value.moon_scores)
            });
            engine.register_get("rewards", |value: &mut $ty| to_array(&value.rewards));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }
    register_gemini_info!(GeminiFirstInfo, "GeminiFirstInfo");
    register_gemini_info!(GeminiSecondInfo, "GeminiSecondInfo");
    register_gemini_info!(GeminiThirdInfo, "GeminiThirdInfo");
}
