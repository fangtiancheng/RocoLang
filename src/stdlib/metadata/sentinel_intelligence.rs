use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("sentinel_intelligence", "evolve_spirit", return_type: "SentinelIntelligenceInfo", "执行哨兵情报活动宠物进化。", params: ["index" => "候选宠物索引。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的活动状态。", examples: ["let info = sentinel_intelligence::evolve_spirit(0, 0);"]),
        super::stdlib_doc!("sentinel_intelligence", "exchange_item", return_type: "SentinelIntelligenceInfo", "兑换哨兵情报活动物品。", params: ["index" => "兑换项索引。"], returns: "返回兑换后的活动状态。", examples: ["let info = sentinel_intelligence::exchange_item(0);"]),
        super::stdlib_doc!("sentinel_intelligence", "exchange_spirit", return_type: "SentinelIntelligenceInfo", "兑换哨兵情报活动宠物。", params: ["index" => "兑换项索引。"], returns: "返回兑换后的活动状态。", examples: ["let info = sentinel_intelligence::exchange_spirit(0);"]),
        super::stdlib_doc!("sentinel_intelligence", "get_prize", return_type: "SentinelIntelligenceInfo", "领取指定首领的哨兵情报活动奖励。", params: ["boss_index" => "首领索引。"], returns: "返回领取后的活动状态。", examples: ["let info = sentinel_intelligence::get_prize(0);"]),
        super::stdlib_doc!("sentinel_intelligence", "query", return_type: "SentinelIntelligenceInfo", "查询哨兵情报活动状态。", params: [], returns: "返回活动进度和奖励信息。", examples: ["let info = sentinel_intelligence::query();"]),
        super::stdlib_doc!("sentinel_intelligence", "query_all", return_type: "SentinelIntelligenceInfo", "查询哨兵情报活动全部状态。", params: [], returns: "返回活动完整状态。", examples: ["let info = sentinel_intelligence::query_all();"]),
        super::stdlib_doc!("sentinel_intelligence", "query_bag", return_type: "SentinelIntelligenceInfo", "查询哨兵情报活动可进化宠物。", params: ["evolve_spirit_id" => "进化目标宠物 ID。"], returns: "返回可进化宠物和活动状态。", examples: ["let info = sentinel_intelligence::query_bag(100);"]),
        super::stdlib_doc!("sentinel_intelligence", "refresh_boss", return_type: "SentinelIntelligenceInfo", "刷新哨兵情报活动首领。", params: [], returns: "返回刷新后的活动状态。", examples: ["let info = sentinel_intelligence::refresh_boss();"]),
        super::stdlib_doc!("sentinel_intelligence", "refresh_exchange", return_type: "SentinelIntelligenceInfo", "刷新哨兵情报活动兑换列表。", params: [], returns: "返回刷新后的活动状态。", examples: ["let info = sentinel_intelligence::refresh_exchange();"]),
        super::stdlib_doc!("sentinel_intelligence", "refresh_mission", return_type: "SentinelIntelligenceInfo", "刷新哨兵情报活动任务。", params: [], returns: "返回刷新后的活动状态。", examples: ["let info = sentinel_intelligence::refresh_mission();"]),
        super::stdlib_doc!("sentinel_intelligence", "settle_fight", return_type: "SentinelIntelligenceInfo", "提交哨兵情报活动战斗结算。", params: [], returns: "返回结算后的活动状态。", examples: ["let info = sentinel_intelligence::settle_fight();"]),
        super::stdlib_doc!("sentinel_intelligence", "start_fight", return_type: "SentinelIntelligenceInfo", "开始指定首领的哨兵情报活动战斗。", params: ["boss_index" => "首领索引。"], returns: "返回开始战斗后的活动状态。", examples: ["let info = sentinel_intelligence::start_fight(0);"]),
    ]
}
