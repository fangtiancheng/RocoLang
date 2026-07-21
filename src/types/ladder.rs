use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderInfo {
    pub win_nums: i64,
    pub win_point: i64,
    pub spirits: Vec<LadderSpiritInfo>,
    pub spirit_info_flag: i64,
    pub backup_spirits: Vec<LadderSpiritInfo>,
    pub left_time: i64,
    pub rank_level: i64,
    pub left_play_times: i64,
    pub left_reward_times: i64,
    pub season_reward_flag: i64,
    pub fight_days: i64,
    pub next_win_point: i64,
    pub day_quests: Vec<LadderQuestInfo>,
    pub show_achievement: i64,
    pub achievement_list: Vec<i64>,
    pub ban_list: Vec<i64>,
    pub records: Vec<LadderFightRecord>,
    pub season: i64,
    pub all_nums: i64,
    pub left_play_times_df: i64,
    pub win_point_df: i64,
    pub win_nums_df: i64,
    pub all_nums_df: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderSpiritInfo {
    pub pet_id: i64,
    pub pet_level: i64,
    pub equipment_ids: Vec<i64>,
    pub now_hp: i64,
    pub full_hp: i64,
    pub skin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderQuestInfo {
    pub status: i64,
    pub id: i64,
    pub give_up: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderFightRecord {
    pub win: i64,
    pub score: i64,
    pub round: i64,
    pub my_point: i64,
    pub other_point: i64,
    pub my_spirits: Vec<i64>,
    pub other_spirits: Vec<i64>,
    pub fight_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderRankUser {
    pub uin: i64,
    pub name: String,
    pub win_nums: i64,
    pub win_point: i64,
    pub rank_num: i64,
    pub medals: Vec<i64>,
    pub achievement_num: i64,
    pub show_achievement: i64,
    pub rank_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderRankInfo {
    pub users: Vec<LadderRankUser>,
    pub rank_level: i64,
    pub rank_change: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeLadderRank {
    pub rank: i64,
    pub small_rank: i64,
    pub star: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderSpiritInfo {
    pub spirit_id: i64,
    pub level: i64,
    pub current_hp: i64,
    pub max_hp: i64,
    pub attribute: i64,
    pub eligibility: String,
    pub eligibility_code: i64,
    pub skin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderFightRecord {
    pub win: i64,
    pub round: i64,
    pub my_spirits: Vec<i64>,
    pub opponent_spirits: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderInfo {
    pub season: i64,
    pub win_count: i64,
    pub battle_count: i64,
    pub left_play_times: i64,
    pub proxy: i64,
    pub grade: i64,
    pub current_rank: TypeLadderRank,
    pub max_rank: TypeLadderRank,
    pub allowed_attributes: Vec<i64>,
    pub banned_spirit_ids: Vec<i64>,
    pub spirits: Vec<TypeLadderSpiritInfo>,
    pub records: Vec<TypeLadderFightRecord>,
    pub season_reward_available: bool,
    pub season_reward_flag: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeLadderRankUser {
    pub uin: i64,
    pub name: String,
    pub win_count: i64,
    pub battle_count: i64,
    pub rank_num: i64,
    pub score: TypeLadderRank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalTypeLadderRankUser {
    Missing,
    Present { value: TypeLadderRankUser },
}

impl RocoOptionalTypeLadderRankUser {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: TypeLadderRankUser) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn value(&self) -> Option<TypeLadderRankUser> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value.clone()),
        }
    }
}

impl From<Option<TypeLadderRankUser>> for RocoOptionalTypeLadderRankUser {
    fn from(value: Option<TypeLadderRankUser>) -> Self {
        value.map(Self::present).unwrap_or(Self::Missing)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLadderRankInfo {
    pub my_info: RocoOptionalTypeLadderRankUser,
    pub users: Vec<TypeLadderRankUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderQuestConfigEntry {
    pub id: i64,
    pub diff: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderSpiritCostEntry {
    pub spirit_id: i64,
    pub cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderMatchConfig {
    pub match_rewards: Vec<String>,
    pub win_rewards: Vec<String>,
    pub season_rewards: Vec<String>,
    pub task0_descriptions: Vec<LadderQuestConfigEntry>,
    pub task1_descriptions: Vec<LadderQuestConfigEntry>,
    pub spirit_costs: Vec<LadderSpiritCostEntry>,
    pub limit_spirits: Vec<i64>,
    pub error: String,
}

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
}

pub(super) fn register_config_rhai_getters(engine: &mut Engine) {
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
