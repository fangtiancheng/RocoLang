use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "manor",
            "get_ground_info",
            "manor::get_ground_info() -> ManorGroundInfo",
            "查询家园农场土地信息。",
            params: [],
            returns: "土地信息。",
            examples: ["let ground = manor::get_ground_info();"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_seed_bag",
            "manor::get_seed_bag() -> ManorItemCount[]",
            "查询家园农场种子背包。",
            params: [],
            returns: "种子列表。",
            examples: ["let seeds = manor::get_seed_bag();"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_plant_status",
            "manor::get_plant_status() -> ManorPlantStatus[]",
            "查询自己农场的成熟、虫害和杂草状态。",
            params: [],
            returns: "农场状态列表。",
            examples: ["let statuses = manor::get_plant_status();"]
        ),
        super::stdlib_doc!(
            "manor",
            "reclaim",
            "manor::reclaim(ground_id: int) -> ManorReclaimResult",
            "开垦指定土地。",
            params: ["ground_id" => "土地 ID。"],
            returns: "开垦结果。",
            examples: ["let result = manor::reclaim(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "sow",
            "manor::sow(seed_id: int, ground_id: int) -> ManorSowResult",
            "在指定土地播种。",
            params: ["seed_id" => "种子 ID。", "ground_id" => "土地 ID。"],
            returns: "播种后的经验和土地状态。",
            examples: ["manor::sow(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "reap",
            "manor::reap(ground_id: int) -> ManorReapResult",
            "收获指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "收获结果、土地状态和奖励。",
            examples: ["manor::reap(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "uproot",
            "manor::uproot(ground_id: int) -> ManorUprootResult",
            "铲除指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "铲除后的土地状态。",
            examples: ["manor::uproot(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "weed",
            "manor::weed(ground_id: int, weed_type: int) -> ManorWeedResult",
            "清理指定土地杂草。",
            params: ["ground_id" => "土地 ID。", "weed_type" => "杂草类型。"],
            returns: "除草后的经验和土地状态。",
            examples: ["manor::weed(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "use_fertilizer",
            "manor::use_fertilizer(ground_id: int, fertilizer_item_id: int) -> ManorFertilizerResult",
            "对指定土地使用肥料。",
            params: ["ground_id" => "土地 ID。", "fertilizer_item_id" => "肥料道具 ID。"],
            returns: "施肥结果和土地状态。",
            examples: ["manor::use_fertilizer(1, 300000);"]
        ),
        super::stdlib_doc!(
            "manor",
            "submit_strawman_result",
            "manor::submit_strawman_result(game_result: int) -> ManorStrawmanPlayResult",
            "提交一次稻草人游戏结果。",
            params: ["game_result" => "游戏结果：0 失败，1 成功。"],
            returns: "本次游戏结算。",
            examples: ["let result = manor::submit_strawman_result(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "claim_strawman_reward",
            "manor::claim_strawman_reward() -> ManorStrawmanRewardResult",
            "领取稻草人经验奖励。",
            params: [],
            returns: "领取后的稻草人总经验。",
            examples: ["let result = manor::claim_strawman_reward();"]
        ),
        super::stdlib_doc!(
            "manor",
            "claim_strawman_gift",
            "manor::claim_strawman_gift() -> ManorRewardInfo[]",
            "领取稻草人玩具奖励。",
            params: [],
            returns: "奖励列表。",
            examples: ["let rewards = manor::claim_strawman_gift();"]
        ),
        super::stdlib_doc!(
            "manor",
            "query_coco_tree",
            "manor::query_coco_tree() -> ManorCocoTreeStatus",
            "查询自己的可可树状态。",
            params: [],
            returns: "可可树状态。",
            examples: ["let tree = manor::query_coco_tree();"]
        ),
        super::stdlib_doc!(
            "manor",
            "apply_coco_tree_feed",
            "manor::apply_coco_tree_feed(feed_type: int) -> ManorCocoTreeFeedResult",
            "对自己的可可树执行浇水或采摘操作。",
            params: ["feed_type" => "操作类型：0 浇水，1 采摘。"],
            returns: "操作后的状态和奖励。",
            examples: ["let result = manor::apply_coco_tree_feed(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "query_friend_coco_tree",
            "manor::query_friend_coco_tree(friend_uin: int) -> ManorFriendCocoTreeStatus",
            "查询好友的可可树状态。",
            params: ["friend_uin" => "好友 UIN。"],
            returns: "好友可可树状态。",
            examples: ["let tree = manor::query_friend_coco_tree(123456);"]
        ),
        super::stdlib_doc!(
            "manor",
            "apply_friend_coco_tree_feed",
            "manor::apply_friend_coco_tree_feed(friend_uin: int) -> ManorFriendCocoTreeFeedResult",
            "给好友的可可树浇水。",
            params: ["friend_uin" => "好友 UIN。"],
            returns: "获得的农场经验。",
            examples: ["let result = manor::apply_friend_coco_tree_feed(123456);"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_friend_list",
            "manor::get_friend_list(version: int) -> ManorFriendSummary[]",
            "查询农场好友简表。",
            params: ["version" => "好友列表版本号。"],
            returns: "好友农场积分简表。",
            examples: ["let friends = manor::get_friend_list(0);"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_friend_details",
            "manor::get_friend_details(friend_uins: int[]) -> ManorFriendDetail[]",
            "批量查询好友详细资料。",
            params: ["friend_uins" => "好友 UIN 列表。"],
            returns: "好友详细资料列表。",
            examples: ["let friends = manor::get_friend_details([123456]);"]
        ),
    ]
}
