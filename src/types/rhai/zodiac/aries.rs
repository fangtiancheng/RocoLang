use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, AriesField, name, value);
    register_getters!(engine, AriesCounter, name, current, limit);
    register_getters!(engine, AriesReward, reward_index, reward_type, reward_count);
    register_getters!(
        engine,
        AriesBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );

    macro_rules! register_aries_info {
        ($ty:ty, $name:literal) => {
            engine.register_type_with_name::<$ty>($name);
            register_getters!(engine, $ty, result_code, message, request_context);
            engine.register_get("fields", |value: &mut $ty| to_array(&value.fields));
            engine.register_get("counters", |value: &mut $ty| to_array(&value.counters));
            engine.register_get("rewards", |value: &mut $ty| to_array(&value.rewards));
            engine.register_get("bag_candidates", |value: &mut $ty| {
                to_array(&value.bag_candidates)
            });
        };
    }
    register_aries_info!(AriesFirstInfo, "AriesFirstInfo");
    register_aries_info!(AriesSecondInfo, "AriesSecondInfo");
    register_aries_info!(AriesThirdInfo, "AriesThirdInfo");
    register_getters!(
        engine,
        AriesThirdStatusInfo,
        result_code,
        message,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
        boss_left_hp,
        left_fight_count,
    );
    register_getters!(
        engine,
        AriesThirdExchangeInfo,
        result_code,
        message,
        item,
        light_num,
        tail_num,
        exchange_count0,
        exchange_count1,
    );
}
