use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("treasure_realm", "boost_by_item", return_type: "TreasureRealmInfo", "使用物品加速珍宝秘境进度。", params: ["index" => "物品或加速项索引。"], returns: "返回加速后的珍宝秘境状态。", examples: ["let info = treasure_realm::boost_by_item(0);"]),
        super::stdlib_doc!("treasure_realm", "boost_by_vip", return_type: "TreasureRealmInfo", "使用 VIP 能力加速珍宝秘境进度。", params: [], returns: "返回加速后的珍宝秘境状态。", examples: ["let info = treasure_realm::boost_by_vip();"]),
        super::stdlib_doc!("treasure_realm", "buy", return_type: "TreasureRealmInfo", "购买珍宝秘境指定奖励。", params: ["index" => "奖励索引。"], returns: "返回购买后的珍宝秘境状态。", examples: ["let info = treasure_realm::buy(0);"]),
        super::stdlib_doc!("treasure_realm", "get_gift", return_type: "TreasureRealmInfo", "领取珍宝秘境礼包。", params: ["index" => "礼包索引。"], returns: "返回领取后的珍宝秘境状态。", examples: ["let info = treasure_realm::get_gift(0);"]),
        super::stdlib_doc!("treasure_realm", "query", return_type: "TreasureRealmInfo", "查询珍宝秘境状态。", params: [], returns: "返回进度、奖励和加速信息。", examples: ["let info = treasure_realm::query();"]),
        super::stdlib_doc!("treasure_realm", "start_battle", return_type: "TreasureRealmInfo", "开始珍宝秘境战斗。", params: [], returns: "返回开始战斗后的珍宝秘境状态。", examples: ["let info = treasure_realm::start_battle();"]),
        super::stdlib_doc!("treasure_realm", "submit_battle", return_type: "TreasureRealmInfo", "提交珍宝秘境战斗结果。", params: [], returns: "返回提交后的珍宝秘境状态。", examples: ["let info = treasure_realm::submit_battle();"]),
    ]
}
