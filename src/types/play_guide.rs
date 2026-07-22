use serde::{Deserialize, Serialize};

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
