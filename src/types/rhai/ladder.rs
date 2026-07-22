use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_optional_getters!(engine, RocoOptionalTypeLadderRankUser);
    register_getters!(
        engine,
        LadderSpiritInfo,
        pet_id,
        pet_level,
        now_hp,
        full_hp,
        skin
    );
    engine.register_get("equipment_ids", |value: &mut LadderSpiritInfo| {
        to_array(&value.equipment_ids)
    });
    register_getters!(engine, LadderQuestInfo, status, id, give_up);
    register_getters!(
        engine,
        LadderFightRecord,
        win,
        score,
        round,
        my_point,
        other_point,
        fight_type
    );
    engine.register_get("my_spirits", |value: &mut LadderFightRecord| {
        to_array(&value.my_spirits)
    });
    engine.register_get("other_spirits", |value: &mut LadderFightRecord| {
        to_array(&value.other_spirits)
    });
    register_getters!(
        engine,
        LadderInfo,
        win_nums,
        win_point,
        spirit_info_flag,
        left_time,
        rank_level,
        left_play_times,
        left_reward_times,
        season_reward_flag,
        fight_days,
        next_win_point,
        show_achievement,
        season,
        all_nums,
        left_play_times_df,
        win_point_df,
        win_nums_df,
        all_nums_df
    );
    engine.register_get("spirits", |value: &mut LadderInfo| to_array(&value.spirits));
    engine.register_get("backup_spirits", |value: &mut LadderInfo| {
        to_array(&value.backup_spirits)
    });
    engine.register_get("day_quests", |value: &mut LadderInfo| {
        to_array(&value.day_quests)
    });
    engine.register_get("achievement_list", |value: &mut LadderInfo| {
        to_array(&value.achievement_list)
    });
    engine.register_get("ban_list", |value: &mut LadderInfo| {
        to_array(&value.ban_list)
    });
    engine.register_get("records", |value: &mut LadderInfo| to_array(&value.records));
    register_getters!(
        engine,
        LadderRankUser,
        uin,
        name,
        win_nums,
        win_point,
        rank_num,
        achievement_num,
        show_achievement,
        rank_level
    );
    engine.register_get("medals", |value: &mut LadderRankUser| {
        to_array(&value.medals)
    });
    register_getters!(engine, LadderRankInfo, rank_level, rank_change);
    engine.register_get("users", |value: &mut LadderRankInfo| to_array(&value.users));
    register_getters!(engine, TypeLadderRank, rank, small_rank, star);
    register_getters!(
        engine,
        TypeLadderSpiritInfo,
        spirit_id,
        level,
        current_hp,
        max_hp,
        attribute,
        eligibility,
        eligibility_code,
        skin
    );
    register_getters!(engine, TypeLadderFightRecord, win, round);
    engine.register_get("my_spirits", |value: &mut TypeLadderFightRecord| {
        to_array(&value.my_spirits)
    });
    engine.register_get("opponent_spirits", |value: &mut TypeLadderFightRecord| {
        to_array(&value.opponent_spirits)
    });
    register_getters!(
        engine,
        TypeLadderInfo,
        season,
        win_count,
        battle_count,
        left_play_times,
        proxy,
        grade,
        current_rank,
        max_rank,
        season_reward_available,
        season_reward_flag
    );
    engine.register_get("allowed_attributes", |value: &mut TypeLadderInfo| {
        to_array(&value.allowed_attributes)
    });
    engine.register_get("banned_spirit_ids", |value: &mut TypeLadderInfo| {
        to_array(&value.banned_spirit_ids)
    });
    engine.register_get("spirits", |value: &mut TypeLadderInfo| {
        to_array(&value.spirits)
    });
    engine.register_get("records", |value: &mut TypeLadderInfo| {
        to_array(&value.records)
    });
    register_getters!(
        engine,
        TypeLadderRankUser,
        uin,
        name,
        win_count,
        battle_count,
        rank_num,
        score
    );
    engine.register_get("my_info", |value: &mut TypeLadderRankInfo| {
        value.my_info.clone()
    });
    engine.register_get("users", |value: &mut TypeLadderRankInfo| {
        to_array(&value.users)
    });
    register_config_rhai_getters(engine);
}

fn register_config_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, LadderQuestConfigEntry, id, diff, description);
    register_getters!(engine, LadderSpiritCostEntry, spirit_id, cost);
    register_getters!(engine, LadderMatchConfig, error);
    engine.register_get("match_rewards", |value: &mut LadderMatchConfig| {
        to_array(&value.match_rewards)
    });
    engine.register_get("win_rewards", |value: &mut LadderMatchConfig| {
        to_array(&value.win_rewards)
    });
    engine.register_get("season_rewards", |value: &mut LadderMatchConfig| {
        to_array(&value.season_rewards)
    });
    engine.register_get("task0_descriptions", |value: &mut LadderMatchConfig| {
        to_array(&value.task0_descriptions)
    });
    engine.register_get("task1_descriptions", |value: &mut LadderMatchConfig| {
        to_array(&value.task1_descriptions)
    });
    engine.register_get("spirit_costs", |value: &mut LadderMatchConfig| {
        to_array(&value.spirit_costs)
    });
    engine.register_get("limit_spirits", |value: &mut LadderMatchConfig| {
        to_array(&value.limit_spirits)
    });
}
