use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        ManorGroundInfo,
        ground_id,
        ground_status,
        seed,
        plant_status,
        current_time,
        total_time,
        total_produce,
        left_produce,
        has_grass,
        has_insect,
        has_fruit,
        season,
        left_row_times
    );
    register_getters!(
        engine,
        ManorInfo,
        qq_uin,
        manor_level,
        manor_exp,
        gold_mass_num,
        gold_money_num,
        guide_type,
        pet_status,
        scarecrow_exp,
        scarecrow_level,
        scarecrow_id,
        home_id,
        parasol_id,
        beautify_id,
        billboard_id,
        scarecrow_ever_play,
        scarecrow_next_exp,
        scarecrow_gift_gotten,
        proficiency_a,
        proficiency_a_exp,
        proficiency_a_exp_pre,
        proficiency_a_exp_next,
        proficiency_b,
        proficiency_b_exp,
        proficiency_b_exp_pre,
        proficiency_b_exp_next,
        proficiency_c,
        proficiency_c_exp,
        proficiency_c_exp_pre,
        proficiency_c_exp_next,
        steal_state
    );
    engine.register_get("gift_status_a", |value: &mut ManorInfo| {
        to_array(&value.gift_status_a)
    });
    engine.register_get("gift_status_b", |value: &mut ManorInfo| {
        to_array(&value.gift_status_b)
    });
    engine.register_get("gift_status_c", |value: &mut ManorInfo| {
        to_array(&value.gift_status_c)
    });
    engine.register_get("grounds", |value: &mut ManorInfo| to_array(&value.grounds));
    register_getters!(engine, ManorItemCount, item_id, item_count);
    register_getters!(
        engine,
        ManorPlantStatus,
        uin,
        has_fruit,
        has_insect,
        has_grass
    );
    register_getters!(engine, ManorRewardInfo, item_id, count);
    register_getters!(engine, ManorSowResult, exp);
    engine.register_get("ground", |value: &mut ManorSowResult| value.ground.clone());
    register_getters!(engine, ManorReclaimResult, ground_id, result, exp);
    engine.register_get("rewards", |value: &mut ManorReclaimResult| {
        to_array(&value.rewards)
    });
    register_getters!(
        engine,
        ManorReapResult,
        qq_uin,
        seed_id,
        result,
        exp,
        fruit_num,
        event_id
    );
    engine.register_get("ground", |value: &mut ManorReapResult| value.ground.clone());
    engine.register_get("rewards", |value: &mut ManorReapResult| {
        to_array(&value.rewards)
    });
    engine.register_get("ground", |value: &mut ManorUprootResult| {
        value.ground.clone()
    });
    register_getters!(engine, ManorWeedResult, qq_uin, exp);
    engine.register_get("ground", |value: &mut ManorWeedResult| value.ground.clone());
    register_getters!(
        engine,
        ManorFertilizerResult,
        can_fertilizer,
        deduce_time_in_second,
        fertilizer,
        uin
    );
    engine.register_get("ground", |value: &mut ManorFertilizerResult| {
        value.ground.clone()
    });
    register_getters!(
        engine,
        ManorStrawmanPlayResult,
        qq_uin,
        magic_id,
        money,
        ground_id
    );
    engine.register_get("rewards", |value: &mut ManorStrawmanPlayResult| {
        to_array(&value.rewards)
    });
    register_getters!(engine, ManorStrawmanRewardResult, total_exp);
    register_getters!(
        engine,
        ManorCocoTreeStatus,
        growth_value,
        level,
        can_pick_fruit,
        watered,
        time_past
    );
    register_getters!(engine, ManorCocoTreeReward, item_id, item_count);
    engine.register_get("status", |value: &mut ManorCocoTreeFeedResult| {
        value.status.clone()
    });
    engine.register_get("rewards", |value: &mut ManorCocoTreeFeedResult| {
        to_array(&value.rewards)
    });
    register_getters!(engine, ManorFriendCocoTreeStatus, feed_type, level);
    register_getters!(engine, ManorFriendCocoTreeFeedResult, manor_exp);
    register_getters!(engine, ManorFriendSummary, uin, score, plant_score);
    register_getters!(
        engine,
        ManorFriendDetail,
        uin,
        vip_level,
        uin_code,
        version,
        roco_nickname,
        qq_nickname,
        qq_icon_url
    );
}
