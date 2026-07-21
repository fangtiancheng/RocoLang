use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        DarkCityExpeditionInfo,
        result_code,
        message,
        fight_id,
        fight_index,
        vip,
        vip_pass_enabled,
        schedule,
        schedule_name,
        added_reputation,
    );
    register_getters!(engine, DarkCityExchangeItem, index, item_id, cost);
    register_getters!(
        engine,
        DarkCityReputationInfo,
        result_code,
        message,
        reputation
    );
    engine.register_get("exchanges", |value: &mut DarkCityReputationInfo| {
        to_array(&value.exchanges)
    });
}
