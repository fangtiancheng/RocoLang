use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("gemini", "first_buy_evolve_access", return_type: "GeminiFirstInfo", "购买双子宫一阶进化资格。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的双子宫一阶状态。", examples: ["let info = gemini::first_buy_evolve_access(0);"]),
        super::stdlib_doc!("gemini", "first_buy_ingredient", return_type: "GeminiFirstInfo", "购买双子宫一阶培养材料。", params: [], returns: "返回购买后的双子宫一阶状态。", examples: ["let info = gemini::first_buy_ingredient();"]),
        super::stdlib_doc!("gemini", "first_query", return_type: "GeminiFirstInfo", "查询双子宫一阶活动状态。", params: [], returns: "返回双子宫一阶进度和奖励。", examples: ["let info = gemini::first_query();"]),
        super::stdlib_doc!("gemini", "first_query_bag", return_type: "GeminiFirstInfo", "查询双子宫一阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = gemini::first_query_bag();"]),
        super::stdlib_doc!("gemini", "first_upgrade", return_type: "GeminiFirstInfo", "执行双子宫一阶宠物升级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回升级后的双子宫一阶状态。", examples: ["let info = gemini::first_upgrade(100, 0);"]),
        super::stdlib_doc!("gemini", "first_upgrade_to_100", return_type: "GeminiFirstInfo", "将双子宫一阶宠物升级至 100 级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回升级后的双子宫一阶状态。", examples: ["let info = gemini::first_upgrade_to_100(100, 0);"]),
        super::stdlib_doc!("gemini", "second_add_score", return_type: "GeminiSecondInfo", "增加双子宫二阶活动积分。", params: ["kind" => "积分类型。", "score" => "增加的积分。"], returns: "返回更新后的双子宫二阶状态。", examples: ["let info = gemini::second_add_score(0, 1);"]),
        super::stdlib_doc!("gemini", "second_buy", return_type: "GeminiSecondInfo", "购买双子宫二阶奖励。", params: [], returns: "返回购买后的双子宫二阶状态。", examples: ["let info = gemini::second_buy();"]),
        super::stdlib_doc!("gemini", "second_get_gift", return_type: "GeminiSecondInfo", "领取双子宫二阶奖励。", params: [], returns: "返回领取后的双子宫二阶状态。", examples: ["let info = gemini::second_get_gift();"]),
        super::stdlib_doc!("gemini", "second_query", return_type: "GeminiSecondInfo", "查询双子宫二阶活动状态。", params: [], returns: "返回双子宫二阶积分和奖励。", examples: ["let info = gemini::second_query();"]),
        super::stdlib_doc!("gemini", "second_submit", return_type: "GeminiSecondInfo", "提交双子宫二阶积分操作。", params: ["kind" => "积分类型。"], returns: "返回提交后的双子宫二阶状态。", examples: ["let info = gemini::second_submit(0);"]),
        super::stdlib_doc!("gemini", "third_buy_challenge_count", return_type: "GeminiThirdInfo", "购买双子宫三阶挑战次数。", params: [], returns: "返回购买后的双子宫三阶状态。", examples: ["let info = gemini::third_buy_challenge_count();"]),
        super::stdlib_doc!("gemini", "third_buy_level", return_type: "GeminiThirdInfo", "购买双子宫三阶等级奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的双子宫三阶状态。", examples: ["let info = gemini::third_buy_level(100, 0);"]),
        super::stdlib_doc!("gemini", "third_buy_score", return_type: "GeminiThirdInfo", "购买双子宫三阶积分。", params: [], returns: "返回购买后的双子宫三阶状态。", examples: ["let info = gemini::third_buy_score();"]),
        super::stdlib_doc!("gemini", "third_buy_score_by_index", return_type: "GeminiThirdInfo", "按位置购买双子宫三阶积分。", params: ["side" => "积分侧别。", "index" => "积分项索引。", "score" => "购买的积分数量。"], returns: "返回购买后的双子宫三阶状态。", examples: ["let info = gemini::third_buy_score_by_index(0, 0, 1);"]),
        super::stdlib_doc!("gemini", "third_query", return_type: "GeminiThirdInfo", "查询双子宫三阶活动状态。", params: [], returns: "返回双子宫三阶进度和奖励。", examples: ["let info = gemini::third_query();"]),
        super::stdlib_doc!("gemini", "third_query_bag", return_type: "GeminiThirdInfo", "查询双子宫三阶可用宠物。", params: [], returns: "返回可用宠物和活动状态。", examples: ["let info = gemini::third_query_bag();"]),
        super::stdlib_doc!("gemini", "third_settle_combat", return_type: "GeminiThirdInfo", "提交双子宫三阶战斗结算。", params: ["side" => "战斗侧别。", "index" => "战斗索引。"], returns: "返回结算后的双子宫三阶状态。", examples: ["let info = gemini::third_settle_combat(0, 0);"]),
        super::stdlib_doc!("gemini", "third_submit", return_type: "GeminiThirdInfo", "提交双子宫三阶宠物。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回提交后的双子宫三阶状态。", examples: ["let info = gemini::third_submit(100, 0);"]),
        super::stdlib_doc!("gemini", "third_submit_without_spirit", return_type: "GeminiThirdInfo", "提交双子宫三阶无宠物方案。", params: [], returns: "返回提交后的双子宫三阶状态。", examples: ["let info = gemini::third_submit_without_spirit();"]),
    ]
}
