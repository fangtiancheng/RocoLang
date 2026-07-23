use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("diamond_tear", "buy_pet", return_type: "DiamondTearInfo", "购买钻石之泪活动宠物。", params: [], returns: "返回购买后的活动状态。", examples: ["let info = diamond_tear::buy_pet();"]),
        super::stdlib_doc!("diamond_tear", "claim_diamond", return_type: "DiamondTearInfo", "领取钻石之泪活动钻石奖励。", params: [], returns: "返回领取后的活动状态。", examples: ["let info = diamond_tear::claim_diamond();"]),
        super::stdlib_doc!("diamond_tear", "freeze", return_type: "DiamondTearInfo", "执行钻石之泪活动冻结操作。", params: [], returns: "返回操作后的活动状态。", examples: ["let info = diamond_tear::freeze();"]),
        super::stdlib_doc!("diamond_tear", "query", return_type: "DiamondTearInfo", "查询钻石之泪活动状态。", params: [], returns: "返回活动状态。", examples: ["let info = diamond_tear::query();"]),
    ]
}
