use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityExpeditionInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub fight_index: i64,
    pub vip: bool,
    pub vip_pass_enabled: bool,
    pub schedule: i64,
    pub schedule_name: String,
    pub added_reputation: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityReputationInfo {
    pub result_code: i64,
    pub message: String,
    pub reputation: i64,
    pub exchanges: Vec<DarkCityExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkCityExchangeItem {
    pub index: i64,
    pub item_id: i64,
    pub cost: i64,
}
