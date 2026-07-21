use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureRealmInfo {
    pub result_code: i64,
    pub message: String,
    pub battle: i64,
    pub battle_id: i64,
    pub schedule: i64,
    pub possible: i64,
    pub time: i64,
    pub got_box: bool,
    pub item_counts: Vec<i64>,
    pub commits: Vec<i64>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        TreasureRealmInfo,
        result_code,
        message,
        battle,
        battle_id,
        schedule,
        possible,
        time,
        got_box,
    );
    engine.register_get("item_counts", |value: &mut TreasureRealmInfo| {
        to_array(&value.item_counts)
    });
    engine.register_get("commits", |value: &mut TreasureRealmInfo| {
        to_array(&value.commits)
    });
}
