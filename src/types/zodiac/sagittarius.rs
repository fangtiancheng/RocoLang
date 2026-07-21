use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusCounter {
    pub name: String,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusScore {
    pub score_index: i64,
    pub current: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusStarPicture {
    pub picture_index: i64,
    pub is_in: i64,
    pub progress: i64,
    pub finish: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusFirstInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagittariusThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<SagittariusField>,
    pub counters: Vec<SagittariusCounter>,
    pub scores: Vec<SagittariusScore>,
    pub star_pictures: Vec<SagittariusStarPicture>,
    pub rewards: Vec<SagittariusRewardItem>,
    pub bag_candidates: Vec<SagittariusBagCandidate>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, SagittariusField, name, value);
    register_getters!(engine, SagittariusCounter, name, current, limit);
    register_getters!(engine, SagittariusScore, score_index, current, limit);
    register_getters!(
        engine,
        SagittariusStarPicture,
        picture_index,
        is_in,
        progress,
        finish
    );
    register_getters!(
        engine,
        SagittariusRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        SagittariusBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    engine.register_type_with_name::<SagittariusFirstInfo>("SagittariusFirstInfo");
    register_getters!(
        engine,
        SagittariusFirstInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusFirstInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusFirstInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusFirstInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusFirstInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusFirstInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusFirstInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<SagittariusSecondInfo>("SagittariusSecondInfo");
    register_getters!(
        engine,
        SagittariusSecondInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusSecondInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusSecondInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusSecondInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusSecondInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusSecondInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusSecondInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_type_with_name::<SagittariusThirdInfo>("SagittariusThirdInfo");
    register_getters!(
        engine,
        SagittariusThirdInfo,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut SagittariusThirdInfo| {
        to_array(&value.fields)
    });
    engine.register_get("counters", |value: &mut SagittariusThirdInfo| {
        to_array(&value.counters)
    });
    engine.register_get("scores", |value: &mut SagittariusThirdInfo| {
        to_array(&value.scores)
    });
    engine.register_get("star_pictures", |value: &mut SagittariusThirdInfo| {
        to_array(&value.star_pictures)
    });
    engine.register_get("rewards", |value: &mut SagittariusThirdInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("bag_candidates", |value: &mut SagittariusThirdInfo| {
        to_array(&value.bag_candidates)
    });
}
