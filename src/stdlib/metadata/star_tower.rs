use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("star_tower", "full_level", return_type: "StarTowerInfo", "使用指定宠物完成星辰塔满级操作。", params: ["spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回操作后的星辰塔状态。", examples: ["let info = star_tower::full_level(100, 0);"]),
        super::stdlib_doc!("star_tower", "get_floor_award", return_type: "StarTowerInfo", "领取星辰塔指定楼层奖励。", params: ["storey_index" => "楼层索引。"], returns: "返回领取后的星辰塔状态。", examples: ["let info = star_tower::get_floor_award(1);"]),
        super::stdlib_doc!("star_tower", "get_top_reward", return_type: "StarTowerInfo", "领取星辰塔顶层奖励。", params: ["reward_index" => "奖励索引。"], returns: "返回领取后的星辰塔状态。", examples: ["let info = star_tower::get_top_reward(0);"]),
        super::stdlib_doc!("star_tower", "query", return_type: "StarTowerInfo", "查询星辰塔状态。", params: [], returns: "返回楼层、首领、奖励和兑换信息。", examples: ["let info = star_tower::query();"]),
        super::stdlib_doc!("star_tower", "query_bag", return_type: "StarTowerInfo", "查询星辰塔相关背包宠物。", params: [], returns: "返回宠物候选和星辰塔状态。", examples: ["let info = star_tower::query_bag();"]),
        super::stdlib_doc!("star_tower", "quick_fight", return_type: "StarTowerInfo", "快速挑战星辰塔楼层。", params: ["storey" => "目标楼层。", "storey1" => "附加楼层参数。", "sell" => "是否自动出售奖励。"], returns: "返回快速挑战后的星辰塔状态。", examples: ["let info = star_tower::quick_fight(1, 1, false);"]),
        super::stdlib_doc!("star_tower", "settle_floor_fight", return_type: "StarTowerInfo", "提交星辰塔楼层战斗结算。", params: ["storey_index" => "楼层索引。", "node_index" => "Boss 节点索引。"], returns: "返回结算后的星辰塔状态。", examples: ["let info = star_tower::settle_floor_fight(1, 0);"]),
        super::stdlib_doc!("star_tower", "settle_top_boss_fight", return_type: "StarTowerInfo", "提交星辰塔顶层首领战斗结算。", params: [], returns: "返回结算后的星辰塔状态。", examples: ["let info = star_tower::settle_top_boss_fight();"]),
        super::stdlib_doc!("star_tower", "toggle_auto_sell", return_type: "StarTowerInfo", "切换星辰塔奖励自动出售设置。", params: [], returns: "返回更新后的星辰塔设置。", examples: ["let info = star_tower::toggle_auto_sell();"]),
    ]
}
