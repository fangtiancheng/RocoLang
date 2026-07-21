use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

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
