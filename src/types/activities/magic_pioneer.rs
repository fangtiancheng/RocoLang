use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerField {
    pub name: String,
    pub values: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicPioneerInfo {
    pub pet: String,
    pub cmd: String,
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub fields: Vec<MagicPioneerField>,
    pub rewards: Vec<MagicPioneerRewardItem>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, MagicPioneerField, name);
    engine.register_get("values", |value: &mut MagicPioneerField| {
        to_array(&value.values)
    });
    register_getters!(
        engine,
        MagicPioneerRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        MagicPioneerInfo,
        pet,
        cmd,
        result_code,
        message,
        request_context
    );
    engine.register_get("fields", |value: &mut MagicPioneerInfo| {
        to_array(&value.fields)
    });
    engine.register_get("rewards", |value: &mut MagicPioneerInfo| {
        to_array(&value.rewards)
    });
}
