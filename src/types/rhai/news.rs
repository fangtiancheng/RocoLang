use super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
