use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "task",
            "query_info_list",
            return_type: "TaskInfoList",
            "查询当前可见的主线和普通任务列表。",
            params: [],
            returns: "返回任务列表和服务端结果信息。",
            examples: ["let result = task::query_info_list();"]
        ),
        super::stdlib_doc!(
            "task",
            "accept",
            return_type: "TaskInfoList",
            "接受指定任务。",
            params: ["task_id" => "任务 ID。"],
            returns: "返回接受任务后的任务列表。",
            examples: ["let result = task::accept(1001);"]
        ),
        super::stdlib_doc!(
            "task",
            "complete",
            return_type: "CompleteTaskResult",
            "提交指定任务完成操作并领取奖励。",
            params: ["task_id" => "任务 ID。"],
            returns: "返回属性、物品和任务状态变化。",
            examples: ["let result = task::complete(1001);"]
        ),
        super::stdlib_doc!(
            "task",
            "report_progress",
            return_type: "TaskProgressResult",
            "上报指定任务的进度。",
            params: ["task_index" => "任务索引。"],
            returns: "返回进度上报结果。",
            examples: ["let result = task::report_progress(1);"]
        ),
        super::stdlib_doc!(
            "task",
            "get_achievement_list",
            return_type: "TaskAchievementList",
            "查询已完成的成就列表。",
            params: [],
            returns: "返回成就列表。",
            examples: ["let result = task::get_achievement_list();"]
        ),
        super::stdlib_doc!(
            "task",
            "check_achievement_finish",
            return_type: "TaskAchievementList",
            "检查指定故事线成就是否完成。",
            params: ["story_id" => "故事线 ID。"],
            returns: "返回检查后的成就列表。",
            examples: ["let result = task::check_achievement_finish(1001);"]
        ),
        super::stdlib_doc!(
            "task",
            "query_magic_growup_info",
            return_type: "MagicGrowupInfo",
            "查询魔法成长任务信息。",
            params: [],
            returns: "返回成长进度、能量和宠物等级信息。",
            examples: ["let result = task::query_magic_growup_info();"]
        ),
        super::stdlib_doc!(
            "task",
            "condition_query_status",
            return_type: "TaskConditionStatus",
            "查询任务条件当前进度。",
            params: ["task_id" => "任务 ID。"],
            returns: "返回任务条件列表及其当前值。",
            examples: ["let result = task::condition_query_status(1001);"]
        ),
        super::stdlib_doc!(
            "task",
            "condition_apply_complete",
            return_type: "TaskConditionApplyResult",
            "提交 NPC 任务条件完成操作。",
            params: ["npc_id" => "NPC ID。"],
            returns: "返回条件提交结果和变更物品。",
            examples: ["let result = task::condition_apply_complete(2001);"]
        ),
    ]
}
