use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdventureItem {
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdventureStatus {
    pub last_point: i64,
    pub got_daily: bool,
    pub props: Vec<i64>,
    pub auto_running: bool,
    pub vip: bool,
    pub guide_level: i64,
    pub medal_bits: i64,
    pub first: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdventureRewards {
    pub items: Vec<AdventureItem>,
}
