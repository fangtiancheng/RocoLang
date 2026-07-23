use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("four_seasons", "claim_spirit_reward", return_type: "FourSeasonsInfo", "领取四时邀约宠物奖励。", params: ["index" => "奖励索引。"], returns: "返回领取后的四时邀约状态。", examples: ["let info = four_seasons::claim_spirit_reward(0);"]),
        super::stdlib_doc!("four_seasons", "exchange_shop_item", return_type: "FourSeasonsInfo", "兑换四时邀约商店物品。", params: ["slot" => "商店槽位。"], returns: "返回兑换后的四时邀约状态。", examples: ["let info = four_seasons::exchange_shop_item(0);"]),
        super::stdlib_doc!("four_seasons", "query", return_type: "FourSeasonsInfo", "查询四时邀约活动状态。", params: [], returns: "返回活动进度、商店和奖励信息。", examples: ["let info = four_seasons::query();"]),
        super::stdlib_doc!("four_seasons", "roll_dice", return_type: "FourSeasonsInfo", "进行四时邀约骰子操作。", params: ["vip" => "是否使用 VIP 骰子能力。"], returns: "返回操作后的四时邀约状态。", examples: ["let info = four_seasons::roll_dice(false);"]),
        super::stdlib_doc!("four_seasons", "submit_event", return_type: "FourSeasonsInfo", "提交四时邀约事件结果。", params: [], returns: "返回提交后的四时邀约状态。", examples: ["let info = four_seasons::submit_event();"]),
        super::stdlib_doc!("four_seasons", "submit_minigame", return_type: "FourSeasonsInfo", "提交四时邀约小游戏结果。", params: [], returns: "返回提交后的四时邀约状态。", examples: ["let info = four_seasons::submit_minigame();"]),
        super::stdlib_doc!("four_seasons", "upgrade_box", return_type: "FourSeasonsInfo", "升级四时邀约指定宝箱。", params: ["position" => "宝箱位置。"], returns: "返回升级后的四时邀约状态。", examples: ["let info = four_seasons::upgrade_box(0);"]),
        super::stdlib_doc!("four_seasons", "use_appointed_step_tool", return_type: "FourSeasonsInfo", "使用四时邀约指定步数道具。", params: ["step" => "指定步数。"], returns: "返回使用道具后的四时邀约状态。", examples: ["let info = four_seasons::use_appointed_step_tool(1);"]),
        super::stdlib_doc!("four_seasons", "use_tool", return_type: "FourSeasonsInfo", "使用四时邀约活动道具。", params: ["tool" => "道具 ID。"], returns: "返回使用道具后的四时邀约状态。", examples: ["let info = four_seasons::use_tool(1001);"]),
    ]
}
