use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskProgressResult {
    pub result_code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub story_id: i64,
    pub task_id: i64,
    pub status: i64,
    pub task_type: i64,
    pub task_type_sub: i64,
    pub theme_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfoList {
    pub result_code: i64,
    pub message: String,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItemChanged {
    pub item_id: i64,
    pub item_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTaskResult {
    pub result_code: i64,
    pub message: String,
    pub money_add: i64,
    pub exp_add: i64,
    pub honor_add: i64,
    pub power_add: i64,
    pub intellect_add: i64,
    pub charm_add: i64,
    pub story_id: i64,
    pub changed_items: Vec<TaskItemChanged>,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAchievement {
    pub theme_id: i64,
    pub finish_time: i64,
    pub story_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAchievementList {
    pub result_code: i64,
    pub message: String,
    pub achievements: Vec<TaskAchievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicGrowupInfo {
    pub ui_ret: i64,
    pub rating_title: i64,
    pub progress: i64,
    pub energy: i64,
    pub spirit_levels: Vec<i64>,
    pub honor_times: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionProgress {
    pub state: i64,
    pub now_value: i64,
    pub max_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionStatus {
    pub result_code: i64,
    pub message: String,
    pub task_id: i64,
    pub conditions: Vec<TaskConditionProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConditionApplyResult {
    pub result_code: i64,
    pub message: String,
    pub npc_id: i64,
    pub changed_items: Vec<TaskItemChanged>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, TaskProgressResult, result_code, message);
    register_getters!(
        engine,
        TaskInfo,
        story_id,
        task_id,
        status,
        task_type,
        task_type_sub,
        theme_id
    );
    register_getters!(engine, TaskInfoList, result_code, message);
    engine.register_get("tasks", |value: &mut TaskInfoList| to_array(&value.tasks));
    register_getters!(engine, TaskItemChanged, item_id, item_count);
    register_getters!(
        engine,
        CompleteTaskResult,
        result_code,
        message,
        money_add,
        exp_add,
        honor_add,
        power_add,
        intellect_add,
        charm_add,
        story_id
    );
    engine.register_get("changed_items", |value: &mut CompleteTaskResult| {
        to_array(&value.changed_items)
    });
    engine.register_get("tasks", |value: &mut CompleteTaskResult| {
        to_array(&value.tasks)
    });
    register_getters!(engine, TaskAchievement, theme_id, finish_time, story_id);
    register_getters!(engine, TaskAchievementList, result_code, message);
    engine.register_get("achievements", |value: &mut TaskAchievementList| {
        to_array(&value.achievements)
    });
    register_getters!(
        engine,
        MagicGrowupInfo,
        ui_ret,
        rating_title,
        progress,
        energy
    );
    engine.register_get("spirit_levels", |value: &mut MagicGrowupInfo| {
        to_array(&value.spirit_levels)
    });
    engine.register_get("honor_times", |value: &mut MagicGrowupInfo| {
        to_array(&value.honor_times)
    });
    register_getters!(engine, TaskConditionProgress, state, now_value, max_value);
    register_getters!(engine, TaskConditionStatus, result_code, message, task_id);
    engine.register_get("conditions", |value: &mut TaskConditionStatus| {
        to_array(&value.conditions)
    });
    register_getters!(
        engine,
        TaskConditionApplyResult,
        result_code,
        message,
        npc_id
    );
    engine.register_get("changed_items", |value: &mut TaskConditionApplyResult| {
        to_array(&value.changed_items)
    });
}
