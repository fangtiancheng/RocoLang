use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonInfo {
    pub result_code: i64,
    pub message: String,
    pub vip: i64,
    pub magic: i64,
    pub count: i64,
    pub show: i64,
    pub pools: Vec<SummonPoolState>,
    pub config_pools: Vec<SummonPoolConfig>,
    pub exchange_groups: Vec<SummonExchangeGroup>,
    pub rewards: Vec<SummonRewardItem>,
    pub records: Vec<SummonRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolState {
    pub version: i64,
    pub token_item_id: i64,
    pub token_count: i64,
    pub today_draw_count: i64,
    pub wish_index: i64,
    pub succeeded: bool,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolConfig {
    pub version: i64,
    pub title: String,
    pub vip_limit: i64,
    pub end_time: i64,
    pub daily_max: i64,
    pub token_item_id: i64,
    pub recommend: String,
    pub info: String,
    pub reward_text: String,
    pub rewards: Vec<SummonRewardItem>,
    pub wish_candidates: Vec<SummonRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeGroup {
    pub kind: String,
    pub items: Vec<SummonExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonExchangeItem {
    pub index: i64,
    pub reward: SummonRewardItem,
    pub cost: SummonRewardItem,
    pub need: i64,
    pub max: i64,
    pub day_max: i64,
    pub times: i64,
    pub day_times: i64,
    pub add: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummonRewardItem {
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonRecord {
    pub pool_version: i64,
    pub title: String,
    pub id: i64,
    pub item_type: i64,
    pub count: i64,
    pub year: i64,
    pub month: i64,
    pub day: i64,
}
