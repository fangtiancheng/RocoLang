use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureRealmInfo {
    pub result_code: i64,
    pub message: String,
    pub battle: i64,
    pub battle_id: i64,
    pub schedule: i64,
    pub possible: i64,
    pub time: i64,
    pub got_box: bool,
    pub item_counts: Vec<i64>,
    pub commits: Vec<i64>,
}
