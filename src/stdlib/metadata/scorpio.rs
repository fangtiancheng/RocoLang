use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("scorpio", "first_buy_direct", return_type: "ScorpioFirstInfo", "直接购买一阶奖励。", params: [], returns: "购买后的天蝎宫一阶状态。", examples: ["let info = scorpio::first_buy_direct();"]),
        super::stdlib_doc!("scorpio", "first_evolve", return_type: "ScorpioFirstInfo", "执行一阶进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "进化后的一阶状态。", examples: ["let info = scorpio::first_evolve(100, 0);"]),
        super::stdlib_doc!("scorpio", "first_level_up", return_type: "ScorpioFirstInfo", "提升一阶宠物等级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "提升后的一阶状态。", examples: ["let info = scorpio::first_level_up(100, 0);"]),
        super::stdlib_doc!("scorpio", "first_query", return_type: "ScorpioFirstInfo", "查询一阶状态。", params: [], returns: "一阶进度和奖励。", examples: ["let info = scorpio::first_query();"]),
        super::stdlib_doc!("scorpio", "first_query_bag", return_type: "ScorpioFirstInfo", "查询一阶可用宠物。", params: [], returns: "可用宠物和一阶状态。", examples: ["let info = scorpio::first_query_bag();"]),
        super::stdlib_doc!("scorpio", "first_submit_game", return_type: "ScorpioFirstInfo", "提交一阶小游戏。", params: ["score" => "小游戏得分。"], returns: "提交后的一阶状态。", examples: ["let info = scorpio::first_submit_game(100);"]),
        super::stdlib_doc!("scorpio", "second_buy_challenge_count", return_type: "ScorpioSecondInfo", "购买二阶挑战次数。", params: [], returns: "购买后的二阶状态。", examples: ["let info = scorpio::second_buy_challenge_count();"]),
        super::stdlib_doc!("scorpio", "second_buy_direct", return_type: "ScorpioSecondInfo", "直接购买二阶奖励。", params: [], returns: "购买后的二阶状态。", examples: ["let info = scorpio::second_buy_direct();"]),
        super::stdlib_doc!("scorpio", "second_evolve", return_type: "ScorpioSecondInfo", "执行二阶进化。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "进化后的二阶状态。", examples: ["let info = scorpio::second_evolve(100, 0);"]),
        super::stdlib_doc!("scorpio", "second_level_up", return_type: "ScorpioSecondInfo", "提升二阶宠物等级。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "宠物捕获时间。"], returns: "提升后的二阶状态。", examples: ["let info = scorpio::second_level_up(100, 0);"]),
        super::stdlib_doc!("scorpio", "second_query", return_type: "ScorpioSecondInfo", "查询二阶状态。", params: [], returns: "二阶进度和奖励。", examples: ["let info = scorpio::second_query();"]),
        super::stdlib_doc!("scorpio", "second_query_bag", return_type: "ScorpioSecondInfo", "查询二阶可用宠物。", params: [], returns: "可用宠物和二阶状态。", examples: ["let info = scorpio::second_query_bag();"]),
        super::stdlib_doc!("scorpio", "second_settle", return_type: "ScorpioSecondInfo", "提交二阶结算。", params: ["reward_source" => "奖励来源。"], returns: "结算后的二阶状态。", examples: ["let info = scorpio::second_settle(0);"]),
        super::stdlib_doc!("scorpio", "third_buy_guess_count", return_type: "ScorpioThirdInfo", "购买三阶猜测次数。", params: ["count" => "购买数量。"], returns: "购买后的三阶状态。", examples: ["let info = scorpio::third_buy_guess_count(1);"]),
        super::stdlib_doc!("scorpio", "third_buy_red_sand", return_type: "ScorpioThirdInfo", "购买三阶红沙。", params: ["count" => "购买数量。"], returns: "购买后的三阶状态。", examples: ["let info = scorpio::third_buy_red_sand(1);"]),
        super::stdlib_doc!("scorpio", "third_exchange_pet", return_type: "ScorpioThirdInfo", "兑换三阶宠物。", params: [], returns: "兑换后的三阶状态。", examples: ["let info = scorpio::third_exchange_pet();"]),
        super::stdlib_doc!("scorpio", "third_query", return_type: "ScorpioThirdInfo", "查询三阶状态。", params: [], returns: "三阶进度和奖励。", examples: ["let info = scorpio::third_query();"]),
        super::stdlib_doc!("scorpio", "third_submit_game", return_type: "ScorpioThirdInfo", "提交三阶小游戏。", params: ["success" => "小游戏是否成功。"], returns: "提交后的三阶状态。", examples: ["let info = scorpio::third_submit_game(true);"]),
    ]
}
