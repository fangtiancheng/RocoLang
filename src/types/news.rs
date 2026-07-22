use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReport {
    pub id: i64,
    pub report_type: i64,
    pub begin_time: i64,
    pub end_time: i64,
    pub act_begin_time: Vec<i64>,
    pub act_end_time: Vec<i64>,
    pub name_image_url: String,
    pub app_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReportsResult {
    pub reports: Vec<NewsTimesReport>,
    pub player_status_today: Vec<i64>,
    pub player_status_forever: Vec<i64>,
    pub gift_gotten: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsActiveItem {
    pub id: i64,
    pub scene_id: i64,
    pub npc_x: i64,
    pub npc_y: i64,
    pub time: String,
    pub content: String,
    pub auto_start: bool,
    pub script_url: String,
    pub app_url: String,
}
