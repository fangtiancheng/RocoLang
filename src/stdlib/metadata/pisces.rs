use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("pisces", "first_buy", return_type: "PiscesFirstInfo", "购买双鱼宫一阶奖励。", params: [], returns: "返回购买后的双鱼宫一阶状态。", examples: ["let info = pisces::first_buy();"]),
        super::stdlib_doc!("pisces", "first_exchange", return_type: "PiscesFirstInfo", "兑换双鱼宫一阶奖励。", params: [], returns: "返回兑换后的双鱼宫一阶状态。", examples: ["let info = pisces::first_exchange();"]),
        super::stdlib_doc!("pisces", "first_get_gift", return_type: "PiscesFirstInfo", "领取双鱼宫一阶奖励。", params: [], returns: "返回领取后的双鱼宫一阶状态。", examples: ["let info = pisces::first_get_gift();"]),
        super::stdlib_doc!("pisces", "first_query", return_type: "PiscesFirstInfo", "查询双鱼宫一阶活动状态。", params: [], returns: "返回双鱼宫一阶进度和奖励。", examples: ["let info = pisces::first_query();"]),
        super::stdlib_doc!("pisces", "first_submit", return_type: "PiscesFirstInfo", "提交双鱼宫一阶活动操作。", params: [], returns: "返回提交后的双鱼宫一阶状态。", examples: ["let info = pisces::first_submit();"]),
        super::stdlib_doc!("pisces", "second_evolution", return_type: "PiscesSecondInfo", "执行双鱼宫二阶宠物进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的双鱼宫二阶状态。", examples: ["let info = pisces::second_evolution(100, 0);"]),
        super::stdlib_doc!("pisces", "second_get_gift", return_type: "PiscesSecondInfo", "领取双鱼宫二阶宠物奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回领取后的双鱼宫二阶状态。", examples: ["let info = pisces::second_get_gift(100, 0);"]),
        super::stdlib_doc!("pisces", "second_query", return_type: "PiscesSecondInfo", "查询双鱼宫二阶活动状态。", params: [], returns: "返回双鱼宫二阶进度和奖励。", examples: ["let info = pisces::second_query();"]),
        super::stdlib_doc!("pisces", "second_repair", return_type: "PiscesSecondInfo", "修复双鱼宫二阶指定项目。", params: ["repair_index" => "修复项索引。"], returns: "返回修复后的双鱼宫二阶状态。", examples: ["let info = pisces::second_repair(0);"]),
        super::stdlib_doc!("pisces", "second_settle_fight", return_type: "PiscesSecondInfo", "提交双鱼宫二阶战斗结算。", params: ["fight_index" => "战斗索引。"], returns: "返回结算后的双鱼宫二阶状态。", examples: ["let info = pisces::second_settle_fight(0);"]),
        super::stdlib_doc!("pisces", "second_submit", return_type: "PiscesSecondInfo", "提交指定宠物参与双鱼宫二阶活动。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回提交后的双鱼宫二阶状态。", examples: ["let info = pisces::second_submit(100, 0);"]),
        super::stdlib_doc!("pisces", "second_submit_without_spirit", return_type: "PiscesSecondInfo", "提交双鱼宫二阶无宠物方案。", params: [], returns: "返回提交后的双鱼宫二阶状态。", examples: ["let info = pisces::second_submit_without_spirit();"]),
        super::stdlib_doc!("pisces", "second_view", return_type: "PiscesSecondInfo", "查看双鱼宫二阶指定类型信息。", params: ["kind" => "信息类型。"], returns: "返回指定类型的双鱼宫二阶状态。", examples: ["let info = pisces::second_view(0);"]),
        super::stdlib_doc!("pisces", "third_buy", return_type: "PiscesThirdInfo", "购买双鱼宫三阶奖励或资格。", params: ["kind" => "购买类型。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的双鱼宫三阶状态。", examples: ["let info = pisces::third_buy(0, 100, 0);"]),
        super::stdlib_doc!("pisces", "third_complete", return_type: "PiscesThirdInfo", "完成双鱼宫三阶指定等级目标。", params: ["level_index" => "等级目标索引。"], returns: "返回完成后的双鱼宫三阶状态。", examples: ["let info = pisces::third_complete(0);"]),
        super::stdlib_doc!("pisces", "third_full_level", return_type: "PiscesThirdInfo", "将双鱼宫三阶宠物升级至目标等级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回升级后的双鱼宫三阶状态。", examples: ["let info = pisces::third_full_level(100, 0);"]),
        super::stdlib_doc!("pisces", "third_get_item", return_type: "PiscesThirdInfo", "领取双鱼宫三阶奖励物品。", params: ["reward_index" => "奖励索引。"], returns: "返回领取后的双鱼宫三阶状态。", examples: ["let info = pisces::third_get_item(0);"]),
        super::stdlib_doc!("pisces", "third_query", return_type: "PiscesThirdInfo", "查询双鱼宫三阶活动状态。", params: [], returns: "返回双鱼宫三阶进度和奖励。", examples: ["let info = pisces::third_query();"]),
        super::stdlib_doc!("pisces", "third_query_bag", return_type: "PiscesThirdInfo", "查询双鱼宫三阶指定类型宠物。", params: ["kind" => "宠物查询类型。"], returns: "返回可用宠物和活动状态。", examples: ["let info = pisces::third_query_bag(0);"]),
        super::stdlib_doc!("pisces", "third_settle_fight", return_type: "PiscesThirdInfo", "提交双鱼宫三阶首领战斗结算。", params: ["boss_index" => "首领索引。"], returns: "返回结算后的双鱼宫三阶状态。", examples: ["let info = pisces::third_settle_fight(0);"]),
        super::stdlib_doc!("pisces", "third_up", return_type: "PiscesThirdInfo", "提升双鱼宫三阶宠物等级或进度。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回提升后的双鱼宫三阶状态。", examples: ["let info = pisces::third_up(100, 0);"]),
    ]
}
