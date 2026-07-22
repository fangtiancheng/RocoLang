use serde::{Deserialize, Serialize};

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
