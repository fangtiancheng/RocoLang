use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("alchemy_furnace", "monkey_cultivation_get_gift", return_type: "MonkeyCultivationInfo", "领取灵猴修炼奖励。", params: [], returns: "返回领取后的灵猴修炼状态。", examples: ["let info = alchemy_furnace::monkey_cultivation_get_gift();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_cultivation_query", return_type: "MonkeyCultivationInfo", "查询灵猴修炼进度和可领取奖励。", params: [], returns: "返回灵猴修炼状态。", examples: ["let info = alchemy_furnace::monkey_cultivation_query();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_cultivation_submit_default", return_type: "MonkeyCultivationInfo", "提交默认材料完成一次灵猴修炼。", params: [], returns: "返回提交后的灵猴修炼状态。", examples: ["let info = alchemy_furnace::monkey_cultivation_submit_default();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_cultivation_submit_pills", return_type: "MonkeyCultivationInfo", "按选定的药剂组合提交灵猴修炼。", params: ["dragon_tiger" => "是否使用龙虎丹。", "cultivate_origin" => "是否使用培元丹。", "nine_turn" => "是否使用九转丹。"], returns: "返回提交后的灵猴修炼状态。", examples: ["let info = alchemy_furnace::monkey_cultivation_submit_pills(true, false, false);"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_evolve", return_type: "MonkeyEvoInfo", "使用指定捕获时间的宠物进行灵猴进化。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_evolve(0);"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_get_gift", return_type: "MonkeyEvoInfo", "领取灵猴进化奖励。", params: [], returns: "返回领取后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_get_gift();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_give_up", return_type: "MonkeyEvoInfo", "放弃当前灵猴进化任务。", params: [], returns: "返回放弃后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_give_up();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_query", return_type: "MonkeyEvoInfo", "查询灵猴进化任务状态。", params: [], returns: "返回灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_query();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_query_bag", return_type: "MonkeyEvoInfo", "查询灵猴进化可用背包宠物。", params: [], returns: "返回灵猴进化状态及可用宠物。", examples: ["let info = alchemy_furnace::monkey_evo_query_bag();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_report_fight", return_type: "MonkeyEvoInfo", "上报灵猴进化战斗结果。", params: ["fight_type" => "战斗类型。"], returns: "返回战斗上报后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_report_fight(1);"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_submit_default", return_type: "MonkeyEvoInfo", "提交默认方案推进灵猴进化。", params: [], returns: "返回提交后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_submit_default();"]),
        super::stdlib_doc!("alchemy_furnace", "monkey_evo_submit_pills", return_type: "MonkeyEvoInfo", "按选定的药剂组合推进灵猴进化。", params: ["dragon_tiger" => "是否使用龙虎丹。", "cultivate_origin" => "是否使用培元丹。", "nine_turn" => "是否使用九转丹。"], returns: "返回提交后的灵猴进化状态。", examples: ["let info = alchemy_furnace::monkey_evo_submit_pills(false, true, false);"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_buy", return_type: "RagingFireInfo", "购买烈火试炼中的宠物或奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的烈火试炼状态。", examples: ["let info = alchemy_furnace::raging_fire_buy(100, 0);"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_get_gift", return_type: "RagingFireInfo", "领取烈火试炼奖励。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回领取后的烈火试炼状态。", examples: ["let info = alchemy_furnace::raging_fire_get_gift(100, 0);"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_query", return_type: "RagingFireInfo", "查询烈火试炼进度。", params: [], returns: "返回烈火试炼状态。", examples: ["let info = alchemy_furnace::raging_fire_query();"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_query_bag", return_type: "RagingFireInfo", "查询烈火试炼可用背包宠物。", params: [], returns: "返回烈火试炼状态及可用宠物。", examples: ["let info = alchemy_furnace::raging_fire_query_bag();"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_report_fight", return_type: "RagingFireInfo", "上报烈火试炼战斗结果。", params: ["target" => "本次战斗目标。"], returns: "返回战斗上报后的烈火试炼状态。", examples: ["let info = alchemy_furnace::raging_fire_report_fight(1);"]),
        super::stdlib_doc!("alchemy_furnace", "raging_fire_submit_stone", return_type: "RagingFireInfo", "提交指定数量的试炼石。", params: ["count" => "提交的试炼石数量。"], returns: "返回提交后的烈火试炼状态。", examples: ["let info = alchemy_furnace::raging_fire_submit_stone(1);"]),
    ]
}
