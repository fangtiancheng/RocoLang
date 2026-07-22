use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
