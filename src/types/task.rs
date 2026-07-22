use serde::{Deserialize, Serialize};

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
