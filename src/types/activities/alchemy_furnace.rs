use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyFurnaceBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyCultivationInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub daytimes: RocoOptionalI64,
    pub finish: RocoOptionalI64,
    pub progress: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonkeyEvoInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pill_counts: Vec<i64>,
    pub branch_type: RocoOptionalI64,
    pub done: RocoOptionalI64,
    pub schedule: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagingFireInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub vip: RocoOptionalI64,
    pub daytimes: RocoOptionalI64,
    pub required_stone_indexes: Vec<i64>,
    pub progress: Vec<i64>,
    pub finish: RocoOptionalI64,
    pub fusion: RocoOptionalI64,
    pub add_progress: RocoOptionalI64,
    pub bag_candidates: Vec<AlchemyFurnaceBagCandidate>,
    pub rewards: Vec<AlchemyFurnaceRewardItem>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        AlchemyFurnaceRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        AlchemyFurnaceBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        MonkeyCultivationInfo,
        result_code,
        message,
        request_context,
        daytimes,
        finish,
        progress,
        add_progress
    );
    engine.register_get("pill_counts", |value: &mut MonkeyCultivationInfo| {
        to_array(&value.pill_counts)
    });
    engine.register_get("rewards", |value: &mut MonkeyCultivationInfo| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        MonkeyEvoInfo,
        result_code,
        message,
        request_context,
        branch_type,
        done,
        schedule,
        add_progress
    );
    engine.register_get("pill_counts", |value: &mut MonkeyEvoInfo| {
        to_array(&value.pill_counts)
    });
    engine.register_get("bag_candidates", |value: &mut MonkeyEvoInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("rewards", |value: &mut MonkeyEvoInfo| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        RagingFireInfo,
        result_code,
        message,
        request_context,
        vip,
        daytimes,
        finish,
        fusion,
        add_progress
    );
    engine.register_get("required_stone_indexes", |value: &mut RagingFireInfo| {
        to_array(&value.required_stone_indexes)
    });
    engine.register_get("progress", |value: &mut RagingFireInfo| {
        to_array(&value.progress)
    });
    engine.register_get("bag_candidates", |value: &mut RagingFireInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("rewards", |value: &mut RagingFireInfo| {
        to_array(&value.rewards)
    });
}
