use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("aquarius", "first_add_challenge_count", return_type: "AquariusFirstInfo", "增加水瓶宫一阶挑战次数。", params: [], returns: "返回更新后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_add_challenge_count();"]),
        super::stdlib_doc!("aquarius", "first_buy_direct", return_type: "AquariusFirstInfo", "直接购买水瓶宫一阶奖励。", params: ["item_or_spirit_id" => "购买物品或宠物 ID。", "count" => "购买数量。"], returns: "返回购买后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_buy_direct(100, 1);"]),
        super::stdlib_doc!("aquarius", "first_buy_evolve_access", return_type: "AquariusFirstInfo", "购买水瓶宫一阶进化资格。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_buy_evolve_access(100, 0);"]),
        super::stdlib_doc!("aquarius", "first_buy_star_num", return_type: "AquariusFirstInfo", "购买水瓶宫一阶星数。", params: [], returns: "返回购买后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_buy_star_num();"]),
        super::stdlib_doc!("aquarius", "first_evolve", return_type: "AquariusFirstInfo", "执行水瓶宫一阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_evolve(100, 0);"]),
        super::stdlib_doc!("aquarius", "first_query", return_type: "AquariusFirstInfo", "查询水瓶宫一阶活动状态。", params: [], returns: "返回水瓶宫一阶进度和奖励。", examples: ["let info = aquarius::first_query();"]),
        super::stdlib_doc!("aquarius", "first_query_bag", return_type: "AquariusFirstInfo", "查询水瓶宫一阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = aquarius::first_query_bag();"]),
        super::stdlib_doc!("aquarius", "first_settle_combat", return_type: "AquariusFirstInfo", "提交水瓶宫一阶首领战斗结算。", params: ["boss_index" => "首领索引。"], returns: "返回结算后的水瓶宫一阶状态。", examples: ["let info = aquarius::first_settle_combat(0);"]),
        super::stdlib_doc!("aquarius", "second_buy_spirit", return_type: "AquariusSecondInfo", "购买水瓶宫二阶宠物。", params: [], returns: "返回购买后的水瓶宫二阶状态。", examples: ["let info = aquarius::second_buy_spirit();"]),
        super::stdlib_doc!("aquarius", "second_buy_tail", return_type: "AquariusSecondInfo", "购买水瓶宫二阶尾部奖励。", params: ["count" => "购买数量。"], returns: "返回购买后的水瓶宫二阶状态。", examples: ["let info = aquarius::second_buy_tail(1);"]),
        super::stdlib_doc!("aquarius", "second_buy_wish", return_type: "AquariusSecondInfo", "购买水瓶宫二阶祈愿。", params: [], returns: "返回购买后的水瓶宫二阶状态。", examples: ["let info = aquarius::second_buy_wish();"]),
        super::stdlib_doc!("aquarius", "second_combat_again", return_type: "AquariusSecondInfo", "重新挑战水瓶宫二阶战斗。", params: [], returns: "返回重新挑战后的水瓶宫二阶状态。", examples: ["let info = aquarius::second_combat_again();"]),
        super::stdlib_doc!("aquarius", "second_exchange_item", return_type: "AquariusSecondExchangeInfo", "兑换水瓶宫二阶物品。", params: ["exchange_position" => "兑换项位置。"], returns: "返回兑换后的水瓶宫二阶兑换结果。", examples: ["let info = aquarius::second_exchange_item(0);"]),
        super::stdlib_doc!("aquarius", "second_exchange_pet", return_type: "AquariusSecondInfo", "兑换水瓶宫二阶宠物。", params: [], returns: "返回兑换后的水瓶宫二阶状态。", examples: ["let info = aquarius::second_exchange_pet();"]),
        super::stdlib_doc!("aquarius", "second_query_diamond", return_type: "AquariusSecondInfo", "查询水瓶宫二阶钻石资源。", params: [], returns: "返回水瓶宫二阶资源状态。", examples: ["let info = aquarius::second_query_diamond();"]),
        super::stdlib_doc!("aquarius", "second_query_status", return_type: "AquariusSecondStatusInfo", "查询水瓶宫二阶任务状态。", params: [], returns: "返回水瓶宫二阶任务状态。", examples: ["let info = aquarius::second_query_status();"]),
        super::stdlib_doc!("aquarius", "third_buy_evolve", return_type: "AquariusThirdInfo", "购买水瓶宫三阶进化资格。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的水瓶宫三阶状态。", examples: ["let info = aquarius::third_buy_evolve(100, 0);"]),
        super::stdlib_doc!("aquarius", "third_buy_level", return_type: "AquariusThirdInfo", "购买水瓶宫三阶等级奖励。", params: [], returns: "返回购买后的水瓶宫三阶状态。", examples: ["let info = aquarius::third_buy_level();"]),
        super::stdlib_doc!("aquarius", "third_evolve", return_type: "AquariusThirdInfo", "执行水瓶宫三阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的水瓶宫三阶状态。", examples: ["let info = aquarius::third_evolve(100, 0);"]),
        super::stdlib_doc!("aquarius", "third_query", return_type: "AquariusThirdInfo", "查询水瓶宫三阶活动状态。", params: [], returns: "返回水瓶宫三阶进度和奖励。", examples: ["let info = aquarius::third_query();"]),
        super::stdlib_doc!("aquarius", "third_query_bag", return_type: "AquariusThirdInfo", "查询水瓶宫三阶指定类型宠物。", params: ["bag_type" => "背包查询类型。"], returns: "返回可用宠物和活动状态。", examples: ["let info = aquarius::third_query_bag(0);"]),
        super::stdlib_doc!("aquarius", "third_random", return_type: "AquariusThirdInfo", "执行水瓶宫三阶随机操作。", params: [], returns: "返回随机操作后的水瓶宫三阶状态。", examples: ["let info = aquarius::third_random();"]),
        super::stdlib_doc!("aquarius", "third_settle_combat", return_type: "AquariusThirdInfo", "提交水瓶宫三阶战斗结算。", params: [], returns: "返回结算后的水瓶宫三阶状态。", examples: ["let info = aquarius::third_settle_combat();"]),
    ]
}
