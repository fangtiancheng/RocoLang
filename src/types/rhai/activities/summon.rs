use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, SummonRewardItem, id, item_type, count);
    register_getters!(
        engine,
        SummonPoolState,
        version,
        token_item_id,
        token_count,
        today_draw_count,
        wish_index,
        succeeded,
        end_time,
    );
    register_getters!(
        engine,
        SummonPoolConfig,
        version,
        title,
        vip_limit,
        end_time,
        daily_max,
        token_item_id,
        recommend,
        info,
        reward_text,
    );
    engine.register_get("rewards", |value: &mut SummonPoolConfig| {
        to_array(&value.rewards)
    });
    engine.register_get("wish_candidates", |value: &mut SummonPoolConfig| {
        to_array(&value.wish_candidates)
    });
    register_getters!(
        engine,
        SummonExchangeItem,
        index,
        reward,
        cost,
        need,
        max,
        day_max,
        times,
        day_times,
        add,
    );
    register_getters!(engine, SummonExchangeGroup, kind);
    engine.register_get("items", |value: &mut SummonExchangeGroup| {
        to_array(&value.items)
    });
    register_getters!(
        engine,
        SummonRecord,
        pool_version,
        title,
        id,
        item_type,
        count,
        year,
        month,
        day,
    );
    register_getters!(
        engine,
        SummonInfo,
        result_code,
        message,
        vip,
        magic,
        count,
        show,
    );
    engine.register_get("pools", |value: &mut SummonInfo| to_array(&value.pools));
    engine.register_get("config_pools", |value: &mut SummonInfo| {
        to_array(&value.config_pools)
    });
    engine.register_get("exchange_groups", |value: &mut SummonInfo| {
        to_array(&value.exchange_groups)
    });
    engine.register_get("rewards", |value: &mut SummonInfo| to_array(&value.rewards));
    engine.register_get("records", |value: &mut SummonInfo| to_array(&value.records));
}
