use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("cancer", "mend_shape_buy", return_type: "CancerMendShapeInfo", "购买巨蟹宫锻形奖励。", params: ["buy_type" => "购买类型。"], returns: "返回购买后的锻形状态。", examples: ["let info = cancer::mend_shape_buy(0);"]),
        super::stdlib_doc!("cancer", "mend_shape_buy_full", return_type: "CancerMendShapeInfo", "使用指定宠物完成巨蟹宫锻形购买。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的锻形状态。", examples: ["let info = cancer::mend_shape_buy_full(100, 0);"]),
        super::stdlib_doc!("cancer", "mend_shape_query", return_type: "CancerMendShapeInfo", "查询巨蟹宫锻形状态。", params: [], returns: "返回锻形进度和奖励。", examples: ["let info = cancer::mend_shape_query();"]),
        super::stdlib_doc!("cancer", "mend_shape_query_bag", return_type: "CancerMendShapeBagInfo", "查询巨蟹宫锻形可用宠物。", params: [], returns: "返回可用宠物列表。", examples: ["let info = cancer::mend_shape_query_bag();"]),
        super::stdlib_doc!("cancer", "mend_shape_upgrade", return_type: "CancerMendShapeInfo", "执行巨蟹宫锻形升级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回升级后的锻形状态。", examples: ["let info = cancer::mend_shape_upgrade(100, 0);"]),
        super::stdlib_doc!("cancer", "mend_shape_upgrade_to_100", return_type: "CancerMendShapeInfo", "将巨蟹宫锻形宠物升级至 100 级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回升级后的锻形状态。", examples: ["let info = cancer::mend_shape_upgrade_to_100(100, 0);"]),
        super::stdlib_doc!("cancer", "sharp_scorpion_buy_tail", return_type: "CancerSharpScorpionInfo", "购买巨蟹宫利蝎活动尾部奖励。", params: ["num" => "购买数量。"], returns: "返回购买后的利蝎状态。", examples: ["let info = cancer::sharp_scorpion_buy_tail(1);"]),
        super::stdlib_doc!("cancer", "sharp_scorpion_buy_wish", return_type: "CancerSharpScorpionInfo", "购买巨蟹宫利蝎活动祈愿。", params: [], returns: "返回购买后的利蝎状态。", examples: ["let info = cancer::sharp_scorpion_buy_wish();"]),
        super::stdlib_doc!("cancer", "sharp_scorpion_exc_pet", return_type: "CancerSharpScorpionInfo", "兑换巨蟹宫利蝎活动宠物。", params: [], returns: "返回兑换后的利蝎状态。", examples: ["let info = cancer::sharp_scorpion_exc_pet();"]),
        super::stdlib_doc!("cancer", "sharp_scorpion_exchange_item", return_type: "CancerSharpScorpionInfo", "兑换巨蟹宫利蝎活动物品。", params: ["exc_pos" => "兑换项位置。"], returns: "返回兑换后的利蝎状态。", examples: ["let info = cancer::sharp_scorpion_exchange_item(0);"]),
        super::stdlib_doc!("cancer", "sharp_scorpion_query", return_type: "CancerSharpScorpionInfo", "查询巨蟹宫利蝎活动状态。", params: [], returns: "返回利蝎活动进度和奖励。", examples: ["let info = cancer::sharp_scorpion_query();"]),
        super::stdlib_doc!("cancer", "unseal_memories_bag_query", return_type: "CancerUnsealMemoriesBagInfo", "查询巨蟹宫解封记忆可用宠物。", params: [], returns: "返回可用宠物列表。", examples: ["let info = cancer::unseal_memories_bag_query();"]),
        super::stdlib_doc!("cancer", "unseal_memories_buy", return_type: "CancerUnsealMemoriesInfo", "购买巨蟹宫解封记忆奖励。", params: ["buy_type" => "购买类型。"], returns: "返回购买后的解封记忆状态。", examples: ["let info = cancer::unseal_memories_buy(0);"]),
        super::stdlib_doc!("cancer", "unseal_memories_commit", return_type: "CancerUnsealMemoriesInfo", "提交巨蟹宫解封记忆选择。", params: ["choice" => "选择项。"], returns: "返回提交后的解封记忆状态。", examples: ["let info = cancer::unseal_memories_commit(0);"]),
        super::stdlib_doc!("cancer", "unseal_memories_put_full", return_type: "CancerUnsealMemoriesBagInfo", "提交指定宠物参与巨蟹宫解封记忆。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回提交后的候选宠物状态。", examples: ["let info = cancer::unseal_memories_put_full(100, 0);"]),
        super::stdlib_doc!("cancer", "unseal_memories_query", return_type: "CancerUnsealMemoriesInfo", "查询巨蟹宫解封记忆状态。", params: [], returns: "返回解封进度和奖励。", examples: ["let info = cancer::unseal_memories_query();"]),
        super::stdlib_doc!("cancer", "unseal_memories_start_game", return_type: "CancerUnsealMemoriesInfo", "开始巨蟹宫解封记忆小游戏。", params: [], returns: "返回开始后的解封记忆状态。", examples: ["let info = cancer::unseal_memories_start_game();"]),
    ]
}
