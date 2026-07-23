use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("three_starters", "fire_buy", return_type: "FiresWillInfo", "购买三主宠活动火系奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的火系活动状态。", examples: ["let info = three_starters::fire_buy(0);"]),
        super::stdlib_doc!("three_starters", "fire_get_gift", return_type: "FiresWillInfo", "领取三主宠活动火系奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回领取后的火系活动状态。", examples: ["let info = three_starters::fire_get_gift(0);"]),
        super::stdlib_doc!("three_starters", "fire_over", return_type: "FiresWillInfo", "提交三主宠活动火系完成操作。", params: ["index" => "目标索引。"], returns: "返回提交后的火系活动状态。", examples: ["let info = three_starters::fire_over(0);"]),
        super::stdlib_doc!("three_starters", "fire_query", return_type: "FiresWillInfo", "查询三主宠活动火系状态。", params: [], returns: "返回火系进度和奖励。", examples: ["let info = three_starters::fire_query();"]),
        super::stdlib_doc!("three_starters", "fire_query_bag", return_type: "FiresWillInfo", "查询三主宠活动火系可用宠物。", params: [], returns: "返回可用宠物和火系状态。", examples: ["let info = three_starters::fire_query_bag();"]),
        super::stdlib_doc!("three_starters", "fire_start", return_type: "FiresWillInfo", "开始三主宠活动火系任务。", params: ["index" => "任务索引。"], returns: "返回开始后的火系活动状态。", examples: ["let info = three_starters::fire_start(0);"]),
        super::stdlib_doc!("three_starters", "fire_submit_battle", return_type: "FiresWillInfo", "提交三主宠活动火系战斗结果。", params: ["index" => "战斗索引。"], returns: "返回提交后的火系活动状态。", examples: ["let info = three_starters::fire_submit_battle(0);"]),
        super::stdlib_doc!("three_starters", "fire_submit_direct", return_type: "FiresWillInfo", "直接提交三主宠活动火系任务。", params: ["index" => "任务索引。"], returns: "返回提交后的火系活动状态。", examples: ["let info = three_starters::fire_submit_direct(0);"]),
        super::stdlib_doc!("three_starters", "sun_buy", return_type: "BatheSunInfo", "购买三主宠活动阳光奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的阳光活动状态。", examples: ["let info = three_starters::sun_buy(0);"]),
        super::stdlib_doc!("three_starters", "sun_get_gift", return_type: "BatheSunInfo", "领取三主宠活动阳光奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回领取后的阳光活动状态。", examples: ["let info = three_starters::sun_get_gift(0);"]),
        super::stdlib_doc!("three_starters", "sun_query", return_type: "BatheSunInfo", "查询三主宠活动阳光状态。", params: [], returns: "返回阳光进度和奖励。", examples: ["let info = three_starters::sun_query();"]),
        super::stdlib_doc!("three_starters", "sun_query_bag", return_type: "BatheSunInfo", "查询三主宠活动阳光可用宠物。", params: [], returns: "返回可用宠物和阳光状态。", examples: ["let info = three_starters::sun_query_bag();"]),
        super::stdlib_doc!("three_starters", "sun_settle_fight", return_type: "BatheSunInfo", "提交三主宠活动阳光战斗结算。", params: [], returns: "返回结算后的阳光活动状态。", examples: ["let info = three_starters::sun_settle_fight();"]),
        super::stdlib_doc!("three_starters", "sun_start_collect", return_type: "BatheSunInfo", "开始三主宠活动阳光收集。", params: [], returns: "返回开始后的阳光活动状态。", examples: ["let info = three_starters::sun_start_collect();"]),
        super::stdlib_doc!("three_starters", "sun_start_fight", return_type: "BatheSunInfo", "开始三主宠活动阳光战斗。", params: [], returns: "返回开始后的阳光活动状态。", examples: ["let info = three_starters::sun_start_fight();"]),
        super::stdlib_doc!("three_starters", "sun_submit", return_type: "BatheSunInfo", "提交三主宠活动阳光任务。", params: [], returns: "返回提交后的阳光活动状态。", examples: ["let info = three_starters::sun_submit();"]),
        super::stdlib_doc!("three_starters", "water_buy", return_type: "WaterSourceInfo", "购买三主宠活动水系奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回购买后的水系活动状态。", examples: ["let info = three_starters::water_buy(0);"]),
        super::stdlib_doc!("three_starters", "water_get_gift", return_type: "WaterSourceInfo", "领取三主宠活动水系奖励。", params: ["catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回领取后的水系活动状态。", examples: ["let info = three_starters::water_get_gift(0);"]),
        super::stdlib_doc!("three_starters", "water_query", return_type: "WaterSourceInfo", "查询三主宠活动水系状态。", params: [], returns: "返回水系进度和奖励。", examples: ["let info = three_starters::water_query();"]),
        super::stdlib_doc!("three_starters", "water_query_bag", return_type: "WaterSourceInfo", "查询三主宠活动水系可用宠物。", params: [], returns: "返回可用宠物和水系状态。", examples: ["let info = three_starters::water_query_bag();"]),
        super::stdlib_doc!("three_starters", "water_settle_fight", return_type: "WaterSourceInfo", "提交三主宠活动水系战斗结算。", params: [], returns: "返回结算后的水系活动状态。", examples: ["let info = three_starters::water_settle_fight();"]),
        super::stdlib_doc!("three_starters", "water_submit", return_type: "WaterSourceInfo", "提交三主宠活动水系任务。", params: [], returns: "返回提交后的水系活动状态。", examples: ["let info = three_starters::water_submit();"]),
    ]
}
