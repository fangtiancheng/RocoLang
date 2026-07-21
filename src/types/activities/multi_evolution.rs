use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub catch_time: i64,
    pub condition_code: i64,
    pub condition_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub candidates: Vec<MultiEvolutionCandidate>,
    pub rewards: Vec<MultiEvolutionRewardItem>,
    pub pet_id: RocoOptionalI64,
    pub result_side: RocoOptionalI64,
    pub item_id: RocoOptionalI64,
    pub count: i64,
    pub available: bool,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        MultiEvolutionCandidate,
        candidate_index,
        spirit_id,
        catch_time,
        condition_code,
        condition_name
    );
    register_getters!(
        engine,
        MultiEvolutionRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        MultiEvolutionInfo,
        result_code,
        message,
        request_context,
        pet_id,
        result_side,
        item_id,
        count,
        available
    );
    engine.register_get("candidates", |value: &mut MultiEvolutionInfo| {
        to_array(&value.candidates)
    });
    engine.register_get("rewards", |value: &mut MultiEvolutionInfo| {
        to_array(&value.rewards)
    });
}
