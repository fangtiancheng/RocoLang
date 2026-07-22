use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        UserInfo,
        uin,
        id,
        nick_name,
        level,
        is_vip,
        vip_level,
        vip_expiring_days,
        vip_lulu,
        trainer_level,
        trainer_exp
    );
    register_getters!(
        engine,
        ServerTimeInfo,
        stamp,
        full_year,
        month,
        date,
        hours,
        minutes,
        seconds,
        day,
        day_of_year
    );
    register_getters!(engine, ServerTimeResult, ok, code, message, error);
    register_error_info_getters!(engine, ServerTimeResult);
    engine.register_get("result", |value: &mut ServerTimeResult| {
        value.result.clone()
    });
}
