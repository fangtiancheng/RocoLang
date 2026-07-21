use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayGuideRewardItem {
    pub id: i64,
    pub count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekTaskActivity {
    pub activity_id: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekTaskInfo {
    pub result_code: i64,
    pub message: String,
    pub progress: Vec<i64>,
    pub button_states: Vec<i64>,
    pub ticket_item_id: i64,
    pub ticket_count: i64,
    pub new_activities: Vec<WeekTaskActivity>,
    pub old_activities: Vec<WeekTaskActivity>,
    pub rewards: Vec<PlayGuideRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTaskProgress {
    pub index: i64,
    pub current: i64,
    pub target: i64,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondProgressReward {
    pub index: i64,
    pub threshold: i64,
    pub state: i64,
    pub claimable: bool,
    pub claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiamondTaskInfo {
    pub result_code: i64,
    pub message: String,
    pub vip: i64,
    pub reward_type: i64,
    pub tasks: Vec<DiamondTaskProgress>,
    pub rewards: Vec<DiamondProgressReward>,
    pub reward_items: Vec<PlayGuideRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqGameHallGiftInfo {
    pub result_code: i64,
    pub message: String,
    pub rewards: Vec<PlayGuideRewardItem>,
}

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
