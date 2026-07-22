use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsShopRewardInfo {
    pub reward_id: i64,
    pub reward_kind: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsMonthlySpiritRewardInfo {
    pub month: i64,
    pub reward_index: i64,
    pub spirit_id: i64,
    pub ticket_cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourSeasonsInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub month: RocoOptionalI64,
    pub map: RocoOptionalI64,
    pub position_1based: RocoOptionalI64,
    pub times: RocoOptionalI64,
    pub ticket: RocoOptionalI64,
    pub used_tool_index: RocoOptionalI64,
    pub need_item_index: RocoOptionalI64,
    pub add: RocoOptionalI64,
    pub point: RocoOptionalI64,
    pub boxes: Vec<i64>,
    pub tools: Vec<i64>,
    pub tool_shop_indexes: Vec<i64>,
    pub tool_shop_flags: Vec<i64>,
    pub pass_boxes: Vec<i64>,
    pub tool_costs: Vec<i64>,
    pub event_item_counts: Vec<i64>,
    pub shop_rewards: Vec<FourSeasonsShopRewardInfo>,
    pub monthly_spirit_rewards: Vec<FourSeasonsMonthlySpiritRewardInfo>,
    pub rewards: Vec<FourSeasonsRewardItem>,
}
