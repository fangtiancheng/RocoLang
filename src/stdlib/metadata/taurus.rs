use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("taurus", "first_buy_item", return_type: "TaurusFirstInfo", "购买金牛宫一阶物品。", params: ["item_index" => "物品索引。", "count" => "购买数量。"], returns: "返回购买后的金牛宫一阶状态。", examples: ["let info = taurus::first_buy_item(0, 1);"]),
        super::stdlib_doc!("taurus", "first_buy_pet", return_type: "TaurusFirstInfo", "购买金牛宫一阶宠物。", params: [], returns: "返回购买后的金牛宫一阶状态。", examples: ["let info = taurus::first_buy_pet();"]),
        super::stdlib_doc!("taurus", "first_get_ding", return_type: "TaurusFirstInfo", "领取金牛宫一阶鼎奖励。", params: [], returns: "返回领取后的金牛宫一阶状态。", examples: ["let info = taurus::first_get_ding();"]),
        super::stdlib_doc!("taurus", "first_get_glue", return_type: "TaurusFirstInfo", "领取金牛宫一阶胶水奖励。", params: [], returns: "返回领取后的金牛宫一阶状态。", examples: ["let info = taurus::first_get_glue();"]),
        super::stdlib_doc!("taurus", "first_get_leather", return_type: "TaurusFirstInfo", "领取金牛宫一阶皮革奖励。", params: [], returns: "返回领取后的金牛宫一阶状态。", examples: ["let info = taurus::first_get_leather();"]),
        super::stdlib_doc!("taurus", "first_get_nail", return_type: "TaurusFirstInfo", "领取金牛宫一阶钉子奖励。", params: [], returns: "返回领取后的金牛宫一阶状态。", examples: ["let info = taurus::first_get_nail();"]),
        super::stdlib_doc!("taurus", "first_mix", return_type: "TaurusFirstInfo", "合成金牛宫一阶指定部件。", params: ["part_index" => "部件索引。"], returns: "返回合成后的金牛宫一阶状态。", examples: ["let info = taurus::first_mix(0);"]),
        super::stdlib_doc!("taurus", "first_query", return_type: "TaurusFirstInfo", "查询金牛宫一阶活动状态。", params: [], returns: "返回金牛宫一阶进度和奖励。", examples: ["let info = taurus::first_query();"]),
        super::stdlib_doc!("taurus", "second_buy_game", return_type: "TaurusSecondInfo", "购买金牛宫二阶小游戏次数。", params: ["game_index" => "小游戏索引。"], returns: "返回购买后的金牛宫二阶状态。", examples: ["let info = taurus::second_buy_game(0);"]),
        super::stdlib_doc!("taurus", "second_buy_level", return_type: "TaurusSecondInfo", "购买金牛宫二阶等级奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的金牛宫二阶状态。", examples: ["let info = taurus::second_buy_level(100, 0);"]),
        super::stdlib_doc!("taurus", "second_evolve", return_type: "TaurusSecondInfo", "执行金牛宫二阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的金牛宫二阶状态。", examples: ["let info = taurus::second_evolve(100, 0);"]),
        super::stdlib_doc!("taurus", "second_query", return_type: "TaurusSecondInfo", "查询金牛宫二阶活动状态。", params: [], returns: "返回金牛宫二阶进度和奖励。", examples: ["let info = taurus::second_query();"]),
        super::stdlib_doc!("taurus", "second_query_bag", return_type: "TaurusSecondInfo", "查询金牛宫二阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = taurus::second_query_bag();"]),
        super::stdlib_doc!("taurus", "second_submit_game", return_type: "TaurusSecondInfo", "提交金牛宫二阶小游戏结果。", params: ["game_index" => "小游戏索引。", "score" => "小游戏得分。"], returns: "返回提交后的金牛宫二阶状态。", examples: ["let info = taurus::second_submit_game(0, 100);"]),
        super::stdlib_doc!("taurus", "third_buy_level", return_type: "TaurusThirdInfo", "购买金牛宫三阶等级奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的金牛宫三阶状态。", examples: ["let info = taurus::third_buy_level(100, 0);"]),
        super::stdlib_doc!("taurus", "third_buy_score", return_type: "TaurusThirdInfo", "购买金牛宫三阶积分。", params: ["kind" => "积分类型。"], returns: "返回购买后的金牛宫三阶状态。", examples: ["let info = taurus::third_buy_score(0);"]),
        super::stdlib_doc!("taurus", "third_evolve", return_type: "TaurusThirdInfo", "执行金牛宫三阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的金牛宫三阶状态。", examples: ["let info = taurus::third_evolve(100, 0);"]),
        super::stdlib_doc!("taurus", "third_get_task", return_type: "TaurusThirdInfo", "领取金牛宫三阶任务。", params: [], returns: "返回领取后的金牛宫三阶状态。", examples: ["let info = taurus::third_get_task();"]),
        super::stdlib_doc!("taurus", "third_query", return_type: "TaurusThirdInfo", "查询金牛宫三阶活动状态。", params: [], returns: "返回金牛宫三阶进度和奖励。", examples: ["let info = taurus::third_query();"]),
        super::stdlib_doc!("taurus", "third_query_bag", return_type: "TaurusThirdInfo", "查询金牛宫三阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = taurus::third_query_bag();"]),
        super::stdlib_doc!("taurus", "third_settle_npc_fight", return_type: "TaurusThirdInfo", "提交金牛宫三阶 NPC 战斗结算。", params: [], returns: "返回结算后的金牛宫三阶状态。", examples: ["let info = taurus::third_settle_npc_fight();"]),
    ]
}
