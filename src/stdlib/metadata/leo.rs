use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("leo", "first_buy_tail", return_type: "LeoFirstInfo", "购买狮子宫的 first_buy_tail 操作。", params: ["count" => "数量。"], returns: "返回 LeoFirstInfo。", examples: ["let result = leo::first_buy_tail(0);"]),
        super::stdlib_doc!("leo", "first_buy_wish", return_type: "LeoFirstInfo", "购买狮子宫的 first_buy_wish 操作。", params: [], returns: "返回 LeoFirstInfo。", examples: ["let result = leo::first_buy_wish();"]),
        super::stdlib_doc!("leo", "first_exchange_item", return_type: "LeoFirstExchangeInfo", "兑换狮子宫的 first_exchange_item 操作。", params: ["exchange_position" => "兑换项位置。"], returns: "返回 LeoFirstExchangeInfo。", examples: ["let result = leo::first_exchange_item(0);"]),
        super::stdlib_doc!("leo", "first_exchange_pet", return_type: "LeoFirstInfo", "兑换狮子宫的 first_exchange_pet 操作。", params: [], returns: "返回 LeoFirstInfo。", examples: ["let result = leo::first_exchange_pet();"]),
        super::stdlib_doc!("leo", "first_query_status", return_type: "LeoFirstStatusInfo", "查询狮子宫的 first_query_status 操作。", params: [], returns: "返回 LeoFirstStatusInfo。", examples: ["let result = leo::first_query_status();"]),
        super::stdlib_doc!("leo", "second_buy_full_level", return_type: "LeoSecondInfo", "购买狮子宫的 second_buy_full_level 操作。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_buy_full_level(0, 0);"]),
        super::stdlib_doc!("leo", "second_query", return_type: "LeoSecondInfo", "查询狮子宫的 second_query 操作。", params: [], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_query();"]),
        super::stdlib_doc!("leo", "second_query_spirit", return_type: "LeoSecondInfo", "查询狮子宫的 second_query_spirit 操作。", params: [], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_query_spirit();"]),
        super::stdlib_doc!("leo", "second_settle_combat", return_type: "LeoSecondInfo", "处理狮子宫二阶战斗结算。", params: ["hunt_index" => "猎取目标索引。"], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_settle_combat(0);"]),
        super::stdlib_doc!("leo", "second_submit_onekey", return_type: "LeoSecondInfo", "提交狮子宫的 second_submit_onekey 操作。", params: [], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_submit_onekey();"]),
        super::stdlib_doc!("leo", "second_submit_spirit", return_type: "LeoSecondInfo", "提交狮子宫的 second_submit_spirit 操作。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "返回 LeoSecondInfo。", examples: ["let result = leo::second_submit_spirit(0, 0);"]),
        super::stdlib_doc!("leo", "third_evolve", return_type: "LeoThirdInfo", "提升狮子宫的 third_evolve 操作。", params: ["catch_time" => "宠物捕获时间。"], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_evolve(0);"]),
        super::stdlib_doc!("leo", "third_full_level", return_type: "LeoThirdInfo", "提升狮子宫的 third_full_level 操作。", params: [], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_full_level();"]),
        super::stdlib_doc!("leo", "third_get_reward", return_type: "LeoThirdInfo", "领取狮子宫的 third_get_reward 操作。", params: [], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_get_reward();"]),
        super::stdlib_doc!("leo", "third_light_star", return_type: "LeoThirdInfo", "点亮狮子宫三阶星位。", params: ["star_index" => "星位索引。"], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_light_star(0);"]),
        super::stdlib_doc!("leo", "third_query", return_type: "LeoThirdInfo", "查询狮子宫的 third_query 操作。", params: [], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_query();"]),
        super::stdlib_doc!("leo", "third_query_bag", return_type: "LeoThirdInfo", "查询狮子宫的 third_query_bag 操作。", params: [], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_query_bag();"]),
        super::stdlib_doc!("leo", "third_submit_combat", return_type: "LeoThirdInfo", "提交狮子宫三阶挑战结果。", params: ["challenge_index" => "挑战索引。", "win" => "是否获胜。"], returns: "返回 LeoThirdInfo。", examples: ["let result = leo::third_submit_combat(0, false);"]),
    ]
}
