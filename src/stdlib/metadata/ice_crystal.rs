use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("ice_crystal", "charge_with_item", return_type: "IceCrystalInfo", "使用物品为冰晶活动充能。", params: ["item" => "充能使用的物品 ID。"], returns: "返回充能后的冰晶活动状态。", examples: ["let info = ice_crystal::charge_with_item(1001);"]),
        super::stdlib_doc!("ice_crystal", "condense_crystal", return_type: "IceCrystalInfo", "凝聚指定冰晶。", params: ["crystal" => "冰晶类型或 ID。"], returns: "返回凝聚后的冰晶活动状态。", examples: ["let info = ice_crystal::condense_crystal(1);"]),
        super::stdlib_doc!("ice_crystal", "evolve", return_type: "IceCrystalInfo", "执行冰晶活动指定目标的进化。", params: ["target" => "进化目标。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的冰晶活动状态。", examples: ["let info = ice_crystal::evolve(100, 0);"]),
        super::stdlib_doc!("ice_crystal", "one_key_evolve", return_type: "IceCrystalInfo", "一键执行冰晶活动指定目标的进化。", params: ["target" => "进化目标。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回一键进化后的冰晶活动状态。", examples: ["let info = ice_crystal::one_key_evolve(100, 0);"]),
        super::stdlib_doc!("ice_crystal", "query", return_type: "IceCrystalInfo", "查询冰晶活动状态。", params: [], returns: "返回冰晶数量、进化进度和奖励状态。", examples: ["let info = ice_crystal::query();"]),
        super::stdlib_doc!("ice_crystal", "query_evolution_bag", return_type: "IceCrystalInfo", "查询冰晶活动可进化的宠物。", params: ["target" => "进化目标。"], returns: "返回可进化宠物和活动状态。", examples: ["let info = ice_crystal::query_evolution_bag(100);"]),
        super::stdlib_doc!("ice_crystal", "report_battle_win", return_type: "IceCrystalInfo", "上报冰晶活动战斗胜利。", params: [], returns: "返回上报后的冰晶活动状态。", examples: ["let info = ice_crystal::report_battle_win();"]),
    ]
}
