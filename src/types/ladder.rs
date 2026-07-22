use serde::{Deserialize, Serialize};

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
