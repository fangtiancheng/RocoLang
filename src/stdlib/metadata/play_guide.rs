use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("play_guide", "diamond_task_claim_reward", return_type: "DiamondTaskInfo", "领取钻石任务奖励。", params: ["index" => "任务索引。"], returns: "返回领取后的钻石任务状态。", examples: ["let info = play_guide::diamond_task_claim_reward(0);"]),
        super::stdlib_doc!("play_guide", "diamond_task_query", return_type: "DiamondTaskInfo", "查询钻石任务状态。", params: [], returns: "返回钻石任务进度和奖励。", examples: ["let info = play_guide::diamond_task_query();"]),
        super::stdlib_doc!("play_guide", "qq_game_hall_gift", return_type: "QqGameHallGiftInfo", "领取 QQ 游戏大厅礼包。", params: [], returns: "返回礼包领取状态和奖励。", examples: ["let info = play_guide::qq_game_hall_gift();"]),
        super::stdlib_doc!("play_guide", "week_task_claim_task", return_type: "WeekTaskInfo", "领取周任务奖励。", params: ["index" => "周任务索引。"], returns: "返回领取后的周任务状态。", examples: ["let info = play_guide::week_task_claim_task(0);"]),
        super::stdlib_doc!("play_guide", "week_task_exchange", return_type: "WeekTaskInfo", "使用周任务资源兑换奖励。", params: ["exchange_type" => "兑换类型。", "index" => "兑换项索引。"], returns: "返回兑换后的周任务状态。", examples: ["let info = play_guide::week_task_exchange(0, 0);"]),
        super::stdlib_doc!("play_guide", "week_task_query", return_type: "WeekTaskInfo", "查询周任务状态。", params: [], returns: "返回周任务进度、资源和奖励。", examples: ["let info = play_guide::week_task_query();"]),
    ]
}
