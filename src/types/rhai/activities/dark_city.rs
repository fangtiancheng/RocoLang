use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
