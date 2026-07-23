use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("libra", "first_advance", return_type: "LibraFirstInfo", "推进天秤宫一阶任务。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "天秤宫一阶状态。", examples: ["let info = libra::first_advance(100, 0);"]),
        super::stdlib_doc!("libra", "first_get_gift", return_type: "LibraFirstInfo", "领取天秤宫一阶奖励。", params: [], returns: "领取后的天秤宫一阶状态。", examples: ["let info = libra::first_get_gift();"]),
        super::stdlib_doc!("libra", "first_notify_full_level", return_type: "LibraFirstInfo", "上报一阶宠物满级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "上报后的天秤宫一阶状态。", examples: ["let info = libra::first_notify_full_level(100, 0);"]),
        super::stdlib_doc!("libra", "first_query", return_type: "LibraFirstInfo", "查询天秤宫一阶状态。", params: [], returns: "一阶进度和奖励。", examples: ["let info = libra::first_query();"]),
        super::stdlib_doc!("libra", "first_query_bag", return_type: "LibraFirstInfo", "查询一阶可用宠物。", params: [], returns: "可用宠物和活动状态。", examples: ["let info = libra::first_query_bag();"]),
        super::stdlib_doc!("libra", "first_settle_fight", return_type: "LibraFirstInfo", "提交一阶战斗结算。", params: ["prop_index" => "战斗道具索引。"], returns: "结算后的状态。", examples: ["let info = libra::first_settle_fight(0);"]),
        super::stdlib_doc!("libra", "first_submit_game", return_type: "LibraFirstInfo", "提交一阶小游戏结果。", params: [], returns: "提交后的状态。", examples: ["let info = libra::first_submit_game();"]),
        super::stdlib_doc!("libra", "second_awaken", return_type: "LibraSecondInfo", "执行二阶觉醒。", params: [], returns: "觉醒后的二阶状态。", examples: ["let info = libra::second_awaken();"]),
        super::stdlib_doc!("libra", "second_buy_challenge_count", return_type: "LibraSecondInfo", "购买二阶挑战次数。", params: [], returns: "购买后的二阶状态。", examples: ["let info = libra::second_buy_challenge_count();"]),
        super::stdlib_doc!("libra", "second_evolution", return_type: "LibraSecondInfo", "执行二阶进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "进化后的二阶状态。", examples: ["let info = libra::second_evolution(100, 0);"]),
        super::stdlib_doc!("libra", "second_full_level", return_type: "LibraSecondInfo", "执行二阶满级操作。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "升级后的二阶状态。", examples: ["let info = libra::second_full_level(100, 0);"]),
        super::stdlib_doc!("libra", "second_query", return_type: "LibraSecondInfo", "查询二阶状态。", params: [], returns: "二阶进度和奖励。", examples: ["let info = libra::second_query();"]),
        super::stdlib_doc!("libra", "second_query_bag", return_type: "LibraSecondInfo", "查询二阶可用宠物。", params: [], returns: "可用宠物和二阶状态。", examples: ["let info = libra::second_query_bag();"]),
        super::stdlib_doc!("libra", "second_settle_fight", return_type: "LibraSecondInfo", "提交二阶 NPC 战斗结算。", params: ["npc_index" => "NPC 索引。"], returns: "结算后的二阶状态。", examples: ["let info = libra::second_settle_fight(0);"]),
        super::stdlib_doc!("libra", "third_buy_tail", return_type: "LibraThirdInfo", "购买三阶尾部奖励。", params: ["count" => "购买数量。"], returns: "购买后的三阶状态。", examples: ["let info = libra::third_buy_tail(1);"]),
        super::stdlib_doc!("libra", "third_buy_wish", return_type: "LibraThirdInfo", "购买三阶祈愿。", params: [], returns: "购买后的三阶状态。", examples: ["let info = libra::third_buy_wish();"]),
        super::stdlib_doc!("libra", "third_exchange_item", return_type: "LibraThirdExchangeInfo", "兑换三阶物品。", params: ["exchange_position" => "兑换项位置。"], returns: "三阶兑换结果。", examples: ["let info = libra::third_exchange_item(0);"]),
        super::stdlib_doc!("libra", "third_exchange_pet", return_type: "LibraThirdInfo", "兑换三阶宠物。", params: [], returns: "兑换后的三阶状态。", examples: ["let info = libra::third_exchange_pet();"]),
        super::stdlib_doc!("libra", "third_query_status", return_type: "LibraThirdStatusInfo", "查询三阶任务状态。", params: [], returns: "三阶任务状态。", examples: ["let info = libra::third_query_status();"]),
    ]
}
