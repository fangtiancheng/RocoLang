use super::*;

/// Generic task socket APIs.
pub trait RocoTaskStdLib: Send {
    fn task_query_info_list(&mut self) -> Result<TaskInfoList> {
        unsupported("task::query_info_list")
    }

    fn task_accept(&mut self, _task_id: i64) -> Result<TaskInfoList> {
        unsupported("task::accept")
    }

    fn task_complete(&mut self, _task_id: i64) -> Result<CompleteTaskResult> {
        unsupported("task::complete")
    }

    fn task_get_achievement_list(&mut self) -> Result<TaskAchievementList> {
        unsupported("task::get_achievement_list")
    }

    fn task_check_achievement_finish(&mut self, _story_id: i64) -> Result<TaskAchievementList> {
        unsupported("task::check_achievement_finish")
    }

    fn task_query_magic_growup_info(&mut self) -> Result<MagicGrowupInfo> {
        unsupported("task::query_magic_growup_info")
    }

    fn task_condition_query_status_raw(&mut self, _task_id: i64) -> Result<TaskConditionStatusRaw> {
        unsupported("task::condition_query_status_raw")
    }

    fn task_condition_apply_complete(&mut self, _npc_id: i64) -> Result<TaskConditionApplyResult> {
        unsupported("task::condition_apply_complete")
    }

    fn task_condition_query_status(&mut self, _task_id: i64) -> Result<TaskConditionStatus> {
        unsupported("task::condition_query_status")
    }
}
