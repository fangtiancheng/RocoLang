use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        FourSeasonsRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        FourSeasonsShopRewardInfo,
        reward_id,
        reward_kind,
        count
    );

    register_getters!(
        engine,
        FourSeasonsMonthlySpiritRewardInfo,
        month,
        reward_index,
        spirit_id,
        ticket_cost
    );
    register_getters!(
        engine,
        FourSeasonsInfo,
        result_code,
        message,
        request_context,
        month,
        map,
        position_1based,
        times,
        ticket,
        used_tool_index,
        need_item_index,
        add,
        point
    );
    engine.register_get("boxes", |value: &mut FourSeasonsInfo| {
        to_array(&value.boxes)
    });
    engine.register_get("tools", |value: &mut FourSeasonsInfo| {
        to_array(&value.tools)
    });
    engine.register_get("tool_shop_indexes", |value: &mut FourSeasonsInfo| {
        to_array(&value.tool_shop_indexes)
    });
    engine.register_get("tool_shop_flags", |value: &mut FourSeasonsInfo| {
        to_array(&value.tool_shop_flags)
    });
    engine.register_get("pass_boxes", |value: &mut FourSeasonsInfo| {
        to_array(&value.pass_boxes)
    });
    engine.register_get("tool_costs", |value: &mut FourSeasonsInfo| {
        to_array(&value.tool_costs)
    });
    engine.register_get("event_item_counts", |value: &mut FourSeasonsInfo| {
        to_array(&value.event_item_counts)
    });
    engine.register_get("shop_rewards", |value: &mut FourSeasonsInfo| {
        to_array(&value.shop_rewards)
    });
    engine.register_get("monthly_spirit_rewards", |value: &mut FourSeasonsInfo| {
        to_array(&value.monthly_spirit_rewards)
    });
    engine.register_get("rewards", |value: &mut FourSeasonsInfo| {
        to_array(&value.rewards)
    });
}
