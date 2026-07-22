use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_optional_getters!(engine, RocoOptionalStarTowerTop);
    engine.register_get("star", |value: &mut RocoOptionalStarTowerTop| {
        value.value().map(|top| top.star).unwrap_or_default()
    });
    engine.register_get("fight_id", |value: &mut RocoOptionalStarTowerTop| {
        value.value().map(|top| top.fight_id).unwrap_or_default()
    });
    engine.register_get("tokens", |value: &mut RocoOptionalStarTowerTop| {
        value
            .value()
            .map(|top| to_array(&top.tokens))
            .unwrap_or_default()
    });
    engine.register_get("exchanges", |value: &mut RocoOptionalStarTowerTop| {
        value
            .value()
            .map(|top| to_array(&top.exchanges))
            .unwrap_or_default()
    });
    register_getters!(
        engine,
        StarTowerNode,
        node_index,
        star,
        spirit_id,
        fight_id,
        item_id,
        reward,
        equip_id
    );
    register_getters!(
        engine,
        StarTowerStorey,
        storey_index,
        first,
        can_quick_fight
    );
    engine.register_get("nodes", |value: &mut StarTowerStorey| {
        to_array(&value.nodes)
    });
    engine.register_get("exchange_items", |value: &mut StarTowerStorey| {
        to_array(&value.exchange_items)
    });
    register_getters!(
        engine,
        StarTowerExchangeItem,
        index,
        item_id,
        item_name,
        spirit_id,
        spirit_name,
        owned,
        required
    );
    register_getters!(
        engine,
        StarTowerTop,
        star,
        refresh,
        fight_desc,
        task_desc,
        fight_id
    );
    engine.register_get("tokens", |value: &mut StarTowerTop| to_array(&value.tokens));
    engine.register_get("exchanges", |value: &mut StarTowerTop| {
        to_array(&value.exchanges)
    });
    engine.register_get("missions", |value: &mut StarTowerTop| {
        to_array(&value.missions)
    });
    engine.register_get("rewards", |value: &mut StarTowerTop| {
        to_array(&value.rewards)
    });
    register_getters!(engine, StarTowerTopMission, index, description, completed);
    register_getters!(
        engine,
        StarTowerTopReward,
        index,
        threshold,
        name,
        amount,
        state,
        claimed,
        claimable
    );
    register_getters!(
        engine,
        StarTowerInfo,
        result_code,
        message,
        mop,
        boss_id,
        countdown,
        auto_sell,
        money
    );
    engine.register_get("has_top", |value: &mut StarTowerInfo| {
        value.top.is_present()
    });
    engine.register_get("top", |value: &mut StarTowerInfo| value.top.clone());
    engine.register_get("clips", |value: &mut StarTowerInfo| to_array(&value.clips));
    engine.register_get("storeys", |value: &mut StarTowerInfo| {
        to_array(&value.storeys)
    });
}
