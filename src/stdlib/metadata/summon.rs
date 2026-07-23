use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("summon", "cancel_wish", return_type: "SummonInfo", "取消召唤祈愿。", params: ["pool_version" => "召唤池版本。"], returns: "返回取消祈愿后的召唤状态。", examples: ["let info = summon::cancel_wish(1);"]),
        super::stdlib_doc!("summon", "draw", return_type: "SummonInfo", "从指定召唤池抽取奖励。", params: ["pool_version" => "召唤池版本。", "draw_count" => "抽取次数。"], returns: "返回抽取后的召唤状态和奖励。", examples: ["let info = summon::draw(1, 10);"]),
        super::stdlib_doc!("summon", "exchange", return_type: "SummonInfo", "使用召唤资源兑换奖励。", params: ["exchange_kind" => "兑换类型。", "pool_version" => "召唤池版本。", "item_index" => "兑换项索引。", "count" => "兑换数量。"], returns: "返回兑换后的召唤状态。", examples: ["let info = summon::exchange(0, 1, 0, 1);"]),
        super::stdlib_doc!("summon", "query", return_type: "SummonInfo", "查询召唤活动状态。", params: [], returns: "返回召唤池、资源和奖励信息。", examples: ["let info = summon::query();"]),
        super::stdlib_doc!("summon", "query_data", return_type: "SummonInfo", "查询召唤活动详细数据。", params: [], returns: "返回召唤活动详细状态。", examples: ["let info = summon::query_data();"]),
        super::stdlib_doc!("summon", "query_record", return_type: "SummonInfo", "查询召唤历史记录。", params: [], returns: "返回召唤记录和活动状态。", examples: ["let info = summon::query_record();"]),
        super::stdlib_doc!("summon", "set_wish", return_type: "SummonInfo", "设置召唤祈愿目标。", params: ["pool_version" => "召唤池版本。", "wish_index" => "祈愿项索引。"], returns: "返回设置后的召唤状态。", examples: ["let info = summon::set_wish(1, 0);"]),
    ]
}
