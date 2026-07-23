use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("multi_evolution", "fire_claim_reward", return_type: "MultiEvolutionInfo", "领取火系多元进化奖励。", params: [], returns: "返回领取后的多元进化状态。", examples: ["let info = multi_evolution::fire_claim_reward();"]),
        super::stdlib_doc!("multi_evolution", "fire_evolve", return_type: "MultiEvolutionInfo", "执行火系多元进化。", params: ["slot" => "进化槽位。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "item_count" => "消耗物品数量。", "fire_score" => "火系进化积分。"], returns: "返回进化后的多元进化状态。", examples: ["let info = multi_evolution::fire_evolve(1, 100, 0, 1, 0);"]),
        super::stdlib_doc!("multi_evolution", "fire_query_booster_item_count", return_type: "MultiEvolutionInfo", "查询火系多元进化加速物品数量。", params: [], returns: "返回加速物品数量和活动状态。", examples: ["let info = multi_evolution::fire_query_booster_item_count();"]),
        super::stdlib_doc!("multi_evolution", "fire_query_candidates", return_type: "MultiEvolutionInfo", "查询火系多元进化候选宠物。", params: ["slot" => "进化槽位。"], returns: "返回候选宠物和活动状态。", examples: ["let info = multi_evolution::fire_query_candidates(1);"]),
        super::stdlib_doc!("multi_evolution", "fire_query_reward_available", return_type: "MultiEvolutionInfo", "查询火系多元进化奖励是否可领取。", params: [], returns: "返回奖励状态和活动信息。", examples: ["let info = multi_evolution::fire_query_reward_available();"]),
        super::stdlib_doc!("multi_evolution", "grass_first_evolve", return_type: "MultiEvolutionInfo", "执行草系多元进化第一阶段。", params: ["slot" => "进化槽位。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "sunlight" => "投入的阳光数量。"], returns: "返回进化后的多元进化状态。", examples: ["let info = multi_evolution::grass_first_evolve(1, 100, 0, 1);"]),
        super::stdlib_doc!("multi_evolution", "grass_query_candidates", return_type: "MultiEvolutionInfo", "查询草系多元进化候选宠物。", params: ["slot" => "进化槽位。"], returns: "返回候选宠物和活动状态。", examples: ["let info = multi_evolution::grass_query_candidates(1);"]),
        super::stdlib_doc!("multi_evolution", "grass_second_evolve", return_type: "MultiEvolutionInfo", "执行草系多元进化第二阶段。", params: ["slot" => "进化槽位。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "sunlight" => "投入的阳光数量。"], returns: "返回进化后的多元进化状态。", examples: ["let info = multi_evolution::grass_second_evolve(1, 100, 0, 1);"]),
        super::stdlib_doc!("multi_evolution", "water_evolve", return_type: "MultiEvolutionInfo", "执行水系多元进化。", params: ["slot" => "进化槽位。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的多元进化状态。", examples: ["let info = multi_evolution::water_evolve(1, 100, 0);"]),
        super::stdlib_doc!("multi_evolution", "water_query_candidates", return_type: "MultiEvolutionInfo", "查询水系多元进化候选宠物。", params: ["slot" => "进化槽位。"], returns: "返回候选宠物和活动状态。", examples: ["let info = multi_evolution::water_query_candidates(1);"]),
    ]
}
