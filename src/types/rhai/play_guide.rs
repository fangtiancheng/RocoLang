use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, PlayGuideRewardItem, id, count, item_type);
    register_getters!(engine, WeekTaskActivity, activity_id, reward_count);
    register_getters!(
        engine,
        WeekTaskInfo,
        result_code,
        message,
        ticket_item_id,
        ticket_count
    );
    engine.register_get("progress", |value: &mut WeekTaskInfo| {
        to_array(&value.progress)
    });
    engine.register_get("button_states", |value: &mut WeekTaskInfo| {
        to_array(&value.button_states)
    });
    engine.register_get("new_activities", |value: &mut WeekTaskInfo| {
        to_array(&value.new_activities)
    });
    engine.register_get("old_activities", |value: &mut WeekTaskInfo| {
        to_array(&value.old_activities)
    });
    engine.register_get("rewards", |value: &mut WeekTaskInfo| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        DiamondTaskProgress,
        index,
        current,
        target,
        completed
    );
    register_getters!(
        engine,
        DiamondProgressReward,
        index,
        threshold,
        state,
        claimable,
        claimed
    );
    register_getters!(
        engine,
        DiamondTaskInfo,
        result_code,
        message,
        vip,
        reward_type
    );
    engine.register_get("tasks", |value: &mut DiamondTaskInfo| {
        to_array(&value.tasks)
    });
    engine.register_get("rewards", |value: &mut DiamondTaskInfo| {
        to_array(&value.rewards)
    });
    engine.register_get("reward_items", |value: &mut DiamondTaskInfo| {
        to_array(&value.reward_items)
    });
    register_getters!(engine, QqGameHallGiftInfo, result_code, message);
    engine.register_get("rewards", |value: &mut QqGameHallGiftInfo| {
        to_array(&value.rewards)
    });
}
