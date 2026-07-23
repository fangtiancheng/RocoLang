use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("mountain_sea", "enter_boss", return_type: "MountainSeaInfo", "进入山海秘境指定首领战斗。", params: ["boss_index" => "首领索引。"], returns: "返回进入战斗后的山海秘境状态。", examples: ["let info = mountain_sea::enter_boss(0);"]),
        super::stdlib_doc!("mountain_sea", "open", return_type: "MountainSeaInfo", "开启山海秘境玩法。", params: [], returns: "返回开启后的山海秘境状态。", examples: ["let info = mountain_sea::open();"]),
        super::stdlib_doc!("mountain_sea", "query", return_type: "MountainSeaInfo", "查询山海秘境状态。", params: [], returns: "返回秘境进度、首领和奖励信息。", examples: ["let info = mountain_sea::query();"]),
        super::stdlib_doc!("mountain_sea", "settle_fight", return_type: "MountainSeaInfo", "提交山海秘境战斗结算。", params: [], returns: "返回结算后的山海秘境状态。", examples: ["let info = mountain_sea::settle_fight();"]),
        super::stdlib_doc!("mountain_sea", "summon", return_type: "MountainSeaInfo", "在山海秘境召唤指定魂魄。", params: ["page_index" => "魂魄列表页码。", "soul_type" => "魂魄类型。", "soul_count" => "召唤数量。"], returns: "返回召唤后的山海秘境状态。", examples: ["let info = mountain_sea::summon(1, 0, 1);"]),
    ]
}
