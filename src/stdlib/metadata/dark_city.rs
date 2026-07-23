use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("dark_city", "expedition_query", return_type: "DarkCityExpeditionInfo", "查询暗黑城远征状态。", params: [], returns: "返回远征进度和奖励状态。", examples: ["let info = dark_city::expedition_query();"]),
        super::stdlib_doc!("dark_city", "expedition_set_vip_pass", return_type: "DarkCityExpeditionInfo", "设置暗黑城远征是否使用 VIP 通行。", params: ["enabled" => "是否启用 VIP 通行。"], returns: "返回更新后的远征状态。", examples: ["let info = dark_city::expedition_set_vip_pass(true);"]),
        super::stdlib_doc!("dark_city", "expedition_settle_fight", return_type: "DarkCityExpeditionInfo", "提交暗黑城远征战斗结算。", params: [], returns: "返回结算后的远征状态。", examples: ["let info = dark_city::expedition_settle_fight();"]),
        super::stdlib_doc!("dark_city", "expedition_start_fight", return_type: "DarkCityExpeditionInfo", "开始暗黑城远征战斗。", params: ["vip_boost" => "是否使用 VIP 战斗加成。"], returns: "返回开始战斗后的远征状态。", examples: ["let info = dark_city::expedition_start_fight(false);"]),
        super::stdlib_doc!("dark_city", "reputation_exchange", return_type: "DarkCityReputationInfo", "使用暗黑城声望兑换奖励。", params: ["index" => "兑换项索引。", "count" => "兑换数量。"], returns: "返回兑换后的声望状态。", examples: ["let info = dark_city::reputation_exchange(0, 1);"]),
        super::stdlib_doc!("dark_city", "reputation_query_exchange", return_type: "DarkCityReputationInfo", "查询暗黑城声望兑换列表。", params: [], returns: "返回声望和兑换项信息。", examples: ["let info = dark_city::reputation_query_exchange();"]),
    ]
}
