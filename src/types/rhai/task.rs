use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, TaskProgressResult, result_code, message);
    register_getters!(
        engine,
        TaskInfo,
        story_id,
        task_id,
        status,
        task_type,
        task_type_sub,
        theme_id
    );
    register_getters!(engine, TaskInfoList, result_code, message);
    engine.register_get("tasks", |value: &mut TaskInfoList| to_array(&value.tasks));
    register_getters!(engine, TaskItemChanged, item_id, item_count);
    register_getters!(
        engine,
        CompleteTaskResult,
        result_code,
        message,
        money_add,
        exp_add,
        honor_add,
        power_add,
        intellect_add,
        charm_add,
        story_id
    );
    engine.register_get("changed_items", |value: &mut CompleteTaskResult| {
        to_array(&value.changed_items)
    });
    engine.register_get("tasks", |value: &mut CompleteTaskResult| {
        to_array(&value.tasks)
    });
    register_getters!(engine, TaskAchievement, theme_id, finish_time, story_id);
    register_getters!(engine, TaskAchievementList, result_code, message);
    engine.register_get("achievements", |value: &mut TaskAchievementList| {
        to_array(&value.achievements)
    });
    register_getters!(
        engine,
        MagicGrowupInfo,
        ui_ret,
        rating_title,
        progress,
        energy
    );
    engine.register_get("spirit_levels", |value: &mut MagicGrowupInfo| {
        to_array(&value.spirit_levels)
    });
    engine.register_get("honor_times", |value: &mut MagicGrowupInfo| {
        to_array(&value.honor_times)
    });
    register_getters!(engine, TaskConditionProgress, state, now_value, max_value);
    register_getters!(engine, TaskConditionStatus, result_code, message, task_id);
    engine.register_get("conditions", |value: &mut TaskConditionStatus| {
        to_array(&value.conditions)
    });
    register_getters!(
        engine,
        TaskConditionApplyResult,
        result_code,
        message,
        npc_id
    );
    engine.register_get("changed_items", |value: &mut TaskConditionApplyResult| {
        to_array(&value.changed_items)
    });
}
