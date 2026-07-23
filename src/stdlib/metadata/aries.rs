use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("aries", "first_buy_direct", return_type: "AriesFirstInfo", "直接购买白羊宫一阶奖励。", params: [], returns: "返回购买后的白羊宫一阶状态。", examples: ["let info = aries::first_buy_direct();"]),
        super::stdlib_doc!("aries", "first_buy_times", return_type: "AriesFirstInfo", "购买白羊宫一阶次数。", params: [], returns: "返回购买后的白羊宫一阶状态。", examples: ["let info = aries::first_buy_times();"]),
        super::stdlib_doc!("aries", "first_dice", return_type: "AriesFirstInfo", "进行白羊宫一阶骰子操作。", params: [], returns: "返回操作后的白羊宫一阶状态。", examples: ["let info = aries::first_dice();"]),
        super::stdlib_doc!("aries", "first_evolve", return_type: "AriesFirstInfo", "执行白羊宫一阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的白羊宫一阶状态。", examples: ["let info = aries::first_evolve(100, 0);"]),
        super::stdlib_doc!("aries", "first_get_gold", return_type: "AriesFirstInfo", "领取白羊宫一阶金币奖励。", params: [], returns: "返回领取后的白羊宫一阶状态。", examples: ["let info = aries::first_get_gold();"]),
        super::stdlib_doc!("aries", "first_level_up", return_type: "AriesFirstInfo", "提升白羊宫一阶宠物等级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回提升后的白羊宫一阶状态。", examples: ["let info = aries::first_level_up(100, 0);"]),
        super::stdlib_doc!("aries", "first_query", return_type: "AriesFirstInfo", "查询白羊宫一阶活动状态。", params: [], returns: "返回白羊宫一阶进度和奖励。", examples: ["let info = aries::first_query();"]),
        super::stdlib_doc!("aries", "first_query_bag", return_type: "AriesFirstInfo", "查询白羊宫一阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = aries::first_query_bag();"]),
        super::stdlib_doc!("aries", "first_settle_battle", return_type: "AriesFirstInfo", "提交白羊宫一阶战斗结算。", params: ["battle_type" => "战斗类型。"], returns: "返回结算后的白羊宫一阶状态。", examples: ["let info = aries::first_settle_battle(0);"]),
        super::stdlib_doc!("aries", "first_start", return_type: "AriesFirstInfo", "开始白羊宫一阶活动。", params: [], returns: "返回开始后的白羊宫一阶状态。", examples: ["let info = aries::first_start();"]),
        super::stdlib_doc!("aries", "second_buy_direct", return_type: "AriesSecondInfo", "直接购买白羊宫二阶奖励。", params: [], returns: "返回购买后的白羊宫二阶状态。", examples: ["let info = aries::second_buy_direct();"]),
        super::stdlib_doc!("aries", "second_buy_level", return_type: "AriesSecondInfo", "购买白羊宫二阶等级奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的白羊宫二阶状态。", examples: ["let info = aries::second_buy_level(100, 0);"]),
        super::stdlib_doc!("aries", "second_evolve", return_type: "AriesSecondInfo", "执行白羊宫二阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的白羊宫二阶状态。", examples: ["let info = aries::second_evolve(100, 0);"]),
        super::stdlib_doc!("aries", "second_query", return_type: "AriesSecondInfo", "查询白羊宫二阶活动状态。", params: [], returns: "返回白羊宫二阶进度和奖励。", examples: ["let info = aries::second_query();"]),
        super::stdlib_doc!("aries", "second_query_bag", return_type: "AriesSecondInfo", "查询白羊宫二阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = aries::second_query_bag();"]),
        super::stdlib_doc!("aries", "second_submit_game", return_type: "AriesSecondInfo", "提交白羊宫二阶小游戏结果。", params: ["power" => "小游戏得分或力量值。"], returns: "返回提交后的白羊宫二阶状态。", examples: ["let info = aries::second_submit_game(100);"]),
        super::stdlib_doc!("aries", "third_buy_tail", return_type: "AriesThirdInfo", "购买白羊宫三阶尾部奖励。", params: ["count" => "购买数量。"], returns: "返回购买后的白羊宫三阶状态。", examples: ["let info = aries::third_buy_tail(1);"]),
        super::stdlib_doc!("aries", "third_buy_wish", return_type: "AriesThirdInfo", "购买白羊宫三阶祈愿。", params: [], returns: "返回购买后的白羊宫三阶状态。", examples: ["let info = aries::third_buy_wish();"]),
        super::stdlib_doc!("aries", "third_exchange_item", return_type: "AriesThirdExchangeInfo", "兑换白羊宫三阶物品。", params: ["exchange_position" => "兑换项位置。"], returns: "返回兑换后的白羊宫三阶兑换结果。", examples: ["let info = aries::third_exchange_item(0);"]),
        super::stdlib_doc!("aries", "third_exchange_pet", return_type: "AriesThirdInfo", "兑换白羊宫三阶宠物。", params: [], returns: "返回兑换后的白羊宫三阶状态。", examples: ["let info = aries::third_exchange_pet();"]),
        super::stdlib_doc!("aries", "third_query_status", return_type: "AriesThirdStatusInfo", "查询白羊宫三阶任务状态。", params: [], returns: "返回白羊宫三阶任务状态。", examples: ["let info = aries::third_query_status();"]),
    ]
}
