use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        SentinelBossInfo,
        index,
        spirit_id,
        difficulty,
        status,
        max_intelligence,
        intelligence,
    );
    register_getters!(
        engine,
        SentinelExchangeInfo,
        index,
        item_id,
        need_bounty,
        status
    );
    register_getters!(
        engine,
        SentinelSpiritExchangeInfo,
        index,
        spirit_id,
        need_intelligence,
        evolve_spirit_id,
        status,
    );
    register_getters!(
        engine,
        SentinelIntelligenceInfo,
        result_code,
        message,
        fight_id,
        added_bounty,
        refresh_count,
        exchange_refresh_count,
        mission_type,
        fight_times,
        bounty,
        intelligence_count,
    );
    engine.register_get("mission_values", |value: &mut SentinelIntelligenceInfo| {
        to_array(&value.mission_values)
    });
    engine.register_get("bosses", |value: &mut SentinelIntelligenceInfo| {
        to_array(&value.bosses)
    });
    engine.register_get("exchanges", |value: &mut SentinelIntelligenceInfo| {
        to_array(&value.exchanges)
    });
    engine.register_get("spirits", |value: &mut SentinelIntelligenceInfo| {
        to_array(&value.spirits)
    });
}
