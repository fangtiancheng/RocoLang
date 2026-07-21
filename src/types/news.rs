use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReport {
    pub id: i64,
    pub report_type: i64,
    pub begin_time: i64,
    pub end_time: i64,
    pub act_begin_time: Vec<i64>,
    pub act_end_time: Vec<i64>,
    pub name_image_url: String,
    pub app_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsTimesReportsResult {
    pub reports: Vec<NewsTimesReport>,
    pub player_status_today: Vec<i64>,
    pub player_status_forever: Vec<i64>,
    pub gift_gotten: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsActiveItem {
    pub id: i64,
    pub scene_id: i64,
    pub npc_x: i64,
    pub npc_y: i64,
    pub time: String,
    pub content: String,
    pub auto_start: bool,
    pub script_url: String,
    pub app_url: String,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        NewsTimesReport,
        id,
        report_type,
        begin_time,
        end_time,
        name_image_url,
        app_url
    );
    engine.register_get("act_begin_time", |value: &mut NewsTimesReport| {
        to_array(&value.act_begin_time)
    });
    engine.register_get("act_end_time", |value: &mut NewsTimesReport| {
        to_array(&value.act_end_time)
    });
    register_getters!(engine, NewsTimesReportsResult, gift_gotten);
    engine.register_get("reports", |value: &mut NewsTimesReportsResult| {
        to_array(&value.reports)
    });
    engine.register_get(
        "player_status_today",
        |value: &mut NewsTimesReportsResult| to_array(&value.player_status_today),
    );
    engine.register_get(
        "player_status_forever",
        |value: &mut NewsTimesReportsResult| to_array(&value.player_status_forever),
    );
    register_getters!(
        engine,
        NewsActiveItem,
        id,
        scene_id,
        npc_x,
        npc_y,
        time,
        content,
        auto_start,
        script_url,
        app_url
    );
}
