use serde::{Deserialize, Serialize};

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
