use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, HomeFurnitureChange, item_id, change, total);
    register_getters!(
        engine,
        HomeFurniture,
        item_id,
        group_id,
        x,
        y,
        expire_time,
        interacted
    );
    register_getters!(
        engine,
        HomeOverview,
        area_id,
        left_x,
        right_x,
        energy,
        uin,
        level,
        exp,
        next_exp,
        remaining_build_count,
        guide_step,
        coins,
        star_factory_uin,
        star_factory_nickname
    );
    engine.register_get("goods", |value: &mut HomeOverview| to_array(&value.goods));
    engine.register_get("furniture", |value: &mut HomeOverview| {
        to_array(&value.furniture)
    });
    register_getters!(engine, HomeFriendSummary, uin, home_exp);
    register_getters!(engine, HomeTrainingSkill, skill_id, pp, max_pp);
    register_getters!(
        engine,
        HomeTrainingSpirit,
        spirit_id,
        level,
        next_level_exp,
        mettle,
        sex,
        catch_time,
        caught_place,
        put_time,
        love,
        max_hp,
        offline_exp,
        offline_time_units
    );
    engine.register_get("stats", |value: &mut HomeTrainingSpirit| {
        to_array(&value.stats)
    });
    engine.register_get("talents", |value: &mut HomeTrainingSpirit| {
        to_array(&value.talents)
    });
    engine.register_get("efforts", |value: &mut HomeTrainingSpirit| {
        to_array(&value.efforts)
    });
    engine.register_get("skills", |value: &mut HomeTrainingSpirit| {
        to_array(&value.skills)
    });
    register_getters!(engine, HomeTrainingSpiritReport, owner_uin);
    engine.register_get("spirit", |value: &mut HomeTrainingSpiritReport| {
        value.spirit.clone()
    });
    register_getters!(engine, HomeTakeTrainingSpiritResult, destination);
    register_getters!(engine, HomeCoachSpiritList, exp, limit, refreshed);
    engine.register_get("spirit_ids", |value: &mut HomeCoachSpiritList| {
        to_array(&value.spirit_ids)
    });
}
