use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
