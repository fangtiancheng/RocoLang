use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeFurnitureChange {
    pub item_id: i64,
    pub change: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeFurniture {
    pub item_id: i64,
    pub group_id: i64,
    pub x: i64,
    pub y: i64,
    pub expire_time: i64,
    pub interacted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeOverview {
    pub area_id: i64,
    pub left_x: i64,
    pub right_x: i64,
    pub energy: i64,
    pub uin: i64,
    pub level: i64,
    pub exp: i64,
    pub next_exp: i64,
    pub remaining_build_count: i64,
    pub guide_step: i64,
    pub coins: i64,
    pub goods: Vec<HomeFurnitureChange>,
    pub furniture: Vec<HomeFurniture>,
    pub star_factory_uin: i64,
    pub star_factory_nickname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeFriendSummary {
    pub uin: i64,
    pub home_exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeTrainingSkill {
    pub skill_id: i64,
    pub pp: i64,
    pub max_pp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeTrainingSpirit {
    pub spirit_id: i64,
    pub level: i64,
    pub next_level_exp: i64,
    pub mettle: i64,
    pub sex: i64,
    pub catch_time: i64,
    pub caught_place: i64,
    pub put_time: i64,
    pub love: i64,
    pub max_hp: i64,
    pub stats: Vec<i64>,
    pub talents: Vec<i64>,
    pub efforts: Vec<i64>,
    pub skills: Vec<HomeTrainingSkill>,
    pub offline_exp: i64,
    pub offline_time_units: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeTrainingSpiritReport {
    pub owner_uin: i64,
    pub spirit: HomeTrainingSpirit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeTakeTrainingSpiritResult {
    pub destination: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeCoachSpiritList {
    pub exp: i64,
    pub limit: i64,
    pub refreshed: bool,
    pub spirit_ids: Vec<i64>,
}
