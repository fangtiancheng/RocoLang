use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
