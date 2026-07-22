use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelIntelligenceInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub added_bounty: i64,
    pub refresh_count: i64,
    pub exchange_refresh_count: i64,
    pub mission_type: i64,
    pub mission_values: Vec<i64>,
    pub fight_times: i64,
    pub bounty: i64,
    pub intelligence_count: i64,
    pub bosses: Vec<SentinelBossInfo>,
    pub exchanges: Vec<SentinelExchangeInfo>,
    pub spirits: Vec<SentinelSpiritExchangeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelBossInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub difficulty: i64,
    pub status: i64,
    pub max_intelligence: i64,
    pub intelligence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelExchangeInfo {
    pub index: i64,
    pub item_id: i64,
    pub need_bounty: i64,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelSpiritExchangeInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub need_intelligence: i64,
    pub evolve_spirit_id: i64,
    pub status: i64,
}
