use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub seal_count: i64,
    pub success: i64,
    pub attrs: Vec<i64>,
    pub bosses: Vec<MountainSeaBossInfo>,
    pub souls: Vec<MountainSeaSoulInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaBossInfo {
    pub index: i64,
    pub boss_type: i64,
    pub fight_id: i64,
    pub name: String,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaSoulInfo {
    pub soul_type: i64,
    pub boss_type: i64,
    pub name: String,
    pub count: i64,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        MountainSeaBossInfo,
        index,
        boss_type,
        fight_id,
        name,
        status,
    );
    register_getters!(
        engine,
        MountainSeaSoulInfo,
        soul_type,
        boss_type,
        name,
        count
    );
    register_getters!(
        engine,
        MountainSeaInfo,
        result_code,
        message,
        fight_id,
        seal_count,
        success,
    );
    engine.register_get("attrs", |value: &mut MountainSeaInfo| {
        to_array(&value.attrs)
    });
    engine.register_get("bosses", |value: &mut MountainSeaInfo| {
        to_array(&value.bosses)
    });
    engine.register_get("souls", |value: &mut MountainSeaInfo| {
        to_array(&value.souls)
    });
}
