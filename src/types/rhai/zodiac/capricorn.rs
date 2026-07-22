use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        CapricornPalaceNoteItem,
        item_index,
        item_id,
        count,
        need
    );
    register_getters!(engine, CapricornPalaceNotesInfo, can_summon);
    engine.register_get("items", |value: &mut CapricornPalaceNotesInfo| {
        to_array(&value.items)
    });
    register_getters!(engine, CapricornTeamPlayer, uin, nick);
    register_getters!(engine, CapricornTeamSnapshot, ticks);
    engine.register_get("players", |value: &mut CapricornTeamSnapshot| {
        to_array(&value.players)
    });
    register_getters!(engine, CapricornInviteListInfo, result_code, message, ticks);
    engine.register_get("players", |value: &mut CapricornInviteListInfo| {
        to_array(&value.players)
    });
    register_getters!(
        engine,
        CapricornTeamOperationInfo,
        result_code,
        message,
        team,
    );
    register_getters!(
        engine,
        CapricornSecondTask,
        task_type,
        data1,
        data2,
        step,
        current,
    );
    register_getters!(
        engine,
        CapricornBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        CapricornStarPalaceInfo,
        result_code,
        message,
        request_context,
    );
    register_getters!(
        engine,
        CapricornSecondInfo,
        result_code,
        message,
        request_context,
        finish,
        current,
        position,
        second_task,
    );
    engine.register_get("bag_candidates", |value: &mut CapricornSecondInfo| {
        to_array(&value.bag_candidates)
    });
    register_getters!(
        engine,
        CapricornThirdInfo,
        result_code,
        message,
        request_context,
        finish,
        current,
        remain,
        price,
        limit,
        progress_percent,
        reward_num,
        tips,
    );
    engine.register_get("bag_candidates", |value: &mut CapricornThirdInfo| {
        to_array(&value.bag_candidates)
    });
}
