use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "manor",
            "get_ground_info",
            return_type: "ManorGroundInfo",
            "查询家园农场土地信息。",
            params: [],
            returns: "土地信息。",
            examples: ["let ground = manor::get_ground_info();"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_seed_bag",
            return_type: "ManorItemCount[]",
            "查询家园农场种子背包。",
            params: [],
            returns: "种子列表。",
            examples: ["let seeds = manor::get_seed_bag();"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_plant_status",
            return_type: "ManorPlantStatus[]",
            "查询自己农场的成熟、虫害和杂草状态。",
            params: [],
            returns: "农场状态列表。",
            examples: ["let statuses = manor::get_plant_status();"]
        ),
        super::stdlib_doc!(
            "manor",
            "reclaim",
            return_type: "ManorReclaimResult",
            "开垦指定土地。",
            params: ["ground_id" => "土地 ID。"],
            returns: "开垦结果。",
            examples: ["let result = manor::reclaim(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "sow",
            return_type: "ManorSowResult",
            "在指定土地播种。",
            params: ["seed_id" => "种子 ID。", "ground_id" => "土地 ID。"],
            returns: "播种后的经验和土地状态。",
            examples: ["manor::sow(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "reap",
            return_type: "ManorReapResult",
            "收获指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "收获结果、土地状态和奖励。",
            examples: ["manor::reap(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "uproot",
            return_type: "ManorUprootResult",
            "铲除指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "铲除后的土地状态。",
            examples: ["manor::uproot(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "weed",
            return_type: "ManorWeedResult",
            "清理指定土地杂草。",
            params: ["ground_id" => "土地 ID。", "weed_type" => "杂草类型。"],
            returns: "除草后的经验和土地状态。",
            examples: ["manor::weed(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "use_fertilizer",
            return_type: "ManorFertilizerResult",
            "对指定土地使用肥料。",
            params: ["ground_id" => "土地 ID。", "fertilizer_item_id" => "肥料道具 ID。"],
            returns: "施肥结果和土地状态。",
            examples: ["manor::use_fertilizer(1, 300000);"]
        ),
        super::stdlib_doc!(
            "manor",
            "submit_strawman_result",
            return_type: "ManorStrawmanPlayResult",
            "提交一次稻草人游戏结果。",
            params: ["game_result" => "游戏结果：0 失败，1 成功。"],
            returns: "本次游戏结算。",
            examples: ["let result = manor::submit_strawman_result(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "claim_strawman_reward",
            return_type: "ManorStrawmanRewardResult",
            "领取稻草人经验奖励。",
            params: [],
            returns: "领取后的稻草人总经验。",
            examples: ["let result = manor::claim_strawman_reward();"]
        ),
        super::stdlib_doc!(
            "manor",
            "claim_strawman_gift",
            return_type: "ManorRewardInfo[]",
            "领取稻草人玩具奖励。",
            params: [],
            returns: "奖励列表。",
            examples: ["let rewards = manor::claim_strawman_gift();"]
        ),
        super::stdlib_doc!(
            "manor",
            "query_coco_tree",
            return_type: "ManorCocoTreeStatus",
            "查询自己的可可树状态。",
            params: [],
            returns: "可可树状态。",
            examples: ["let tree = manor::query_coco_tree();"]
        ),
        super::stdlib_doc!(
            "manor",
            "apply_coco_tree_feed",
            return_type: "ManorCocoTreeFeedResult",
            "对自己的可可树执行浇水或采摘操作。",
            params: ["feed_type" => "操作类型：0 浇水，1 采摘。"],
            returns: "操作后的状态和奖励。",
            examples: ["let result = manor::apply_coco_tree_feed(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "query_friend_coco_tree",
            return_type: "ManorFriendCocoTreeStatus",
            "查询好友的可可树状态。",
            params: ["friend_uin" => "好友 UIN。"],
            returns: "好友可可树状态。",
            examples: ["let tree = manor::query_friend_coco_tree(123456);"]
        ),
        super::stdlib_doc!(
            "manor",
            "apply_friend_coco_tree_feed",
            return_type: "ManorFriendCocoTreeFeedResult",
            "给好友的可可树浇水。",
            params: ["friend_uin" => "好友 UIN。"],
            returns: "获得的农场经验。",
            examples: ["let result = manor::apply_friend_coco_tree_feed(123456);"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_friend_list",
            return_type: "ManorFriendSummary[]",
            "查询农场好友简表。",
            params: ["version" => "好友列表版本号。"],
            returns: "好友农场积分简表。",
            examples: ["let friends = manor::get_friend_list(0);"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_friend_details",
            return_type: "ManorFriendDetail[]",
            "批量查询好友详细资料。",
            params: ["friend_uins" => "好友 UIN 列表。"],
            returns: "好友详细资料列表。",
            examples: ["let friends = manor::get_friend_details([123456]);"]
        ),
    ]
}
