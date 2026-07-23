use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "HomeFurnitureChange" => (
            "家园物品数量变化。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("change", "int", "本次变化量。"),
                field("total", "int", "变化后的总数量。"),
            ],
        ),
        "HomeFurniture" => (
            "家园中摆放的家具。",
            vec![
                field("item_id", "int", "家具物品 ID。"),
                field("group_id", "int", "家具实例组 ID。"),
                field("x", "int", "横坐标。"),
                field("y", "int", "纵坐标。"),
                field("expire_time", "int", "过期时间。"),
                field("interacted", "bool", "是否已经互动。"),
            ],
        ),
        "HomeOverview" => (
            "家园区域概览。",
            vec![
                field("area_id", "int", "区域 ID。"),
                field("left_x", "int", "区域左边界。"),
                field("right_x", "int", "区域右边界。"),
                field("energy", "int", "家园能量。"),
                field("uin", "int", "家园主人 UIN。"),
                field("level", "int", "家园等级。"),
                field("exp", "int", "当前家园经验。"),
                field("next_exp", "int", "下一级所需经验。"),
                field("remaining_build_count", "int", "剩余建造次数。"),
                field("guide_step", "int", "引导步骤。"),
                field("coins", "int", "家园硬币。"),
                field("goods", "HomeFurnitureChange[]", "家园物品列表。"),
                field("furniture", "HomeFurniture[]", "已摆放家具列表。"),
                field("star_factory_uin", "int", "星工场好友 UIN。"),
                field("star_factory_nickname", "string", "星工场好友昵称。"),
            ],
        ),
        "HomeFriendSummary" => (
            "家园好友摘要。",
            vec![
                field("uin", "int", "好友 UIN。"),
                field("home_exp", "int", "好友家园经验。"),
            ],
        ),
        "HomeTrainingSkill" => (
            "家园锻炼宠物技能。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "当前 PP。"),
                field("max_pp", "int", "最大 PP。"),
            ],
        ),
        "HomeTrainingSpirit" => (
            "家园锻炼宠物详情。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("level", "int", "等级。"),
                field("next_level_exp", "int", "下一级所需经验。"),
                field("mettle", "int", "性格。"),
                field("sex", "int", "性别。"),
                field("catch_time", "int", "捕获时间。"),
                field("caught_place", "int", "捕获地点。"),
                field("put_time", "int", "放入家园的时间。"),
                field("love", "int", "亲密度。"),
                field("max_hp", "int", "最大生命值。"),
                field("stats", "int[]", "六项能力值。"),
                field("talents", "int[]", "六项天赋值。"),
                field("efforts", "int[]", "六项努力值。"),
                field("skills", "HomeTrainingSkill[]", "技能列表。"),
                field("offline_exp", "int", "离线锻炼经验。"),
                field("offline_time_units", "int", "离线锻炼时间单位。"),
            ],
        ),
        "HomeTrainingSpiritReport" => (
            "家园锻炼宠物报告。",
            vec![
                field("owner_uin", "int", "宠物所属角色 UIN。"),
                field("spirit", "HomeTrainingSpirit", "锻炼宠物详情。"),
            ],
        ),
        "HomeTakeTrainingSpiritResult" => (
            "收回家园锻炼宠物的结果。",
            vec![field("destination", "int", "宠物收回后的目标位置。")],
        ),
        "HomeCoachSpiritList" => (
            "家园教练切磋宠物列表。",
            vec![
                field("exp", "int", "当前教练经验。"),
                field("limit", "int", "服务端限制值。"),
                field("refreshed", "bool", "本次是否完成刷新。"),
                field("spirit_ids", "int[]", "可切磋宠物 ID 列表。"),
            ],
        ),
        "ManorGroundInfo" => (
            "Manor ground information.",
            vec![
                field("ground_id", "int", "Ground ID."),
                field("ground_status", "int", "Ground status."),
                field("seed", "int", "Seed ID."),
                field("plant_status", "int", "Plant status."),
                field("current_time", "int", "Current growth time."),
                field("total_time", "int", "Total growth time."),
                field("total_produce", "int", "Total produce count."),
                field("left_produce", "int", "Remaining produce count."),
                field("has_grass", "bool", "Whether grass is present."),
                field("has_insect", "bool", "Whether insects are present."),
                field("has_fruit", "bool", "Whether fruit can be harvested."),
                field("season", "int", "Season ID."),
                field("left_row_times", "int", "Remaining row action count."),
            ],
        ),
        "ManorPlantStatus" => (
            "农场种植状态。",
            vec![
                field("uin", "int", "状态所属角色 UIN。"),
                field("has_fruit", "bool", "是否有可收获果实。"),
                field("has_insect", "bool", "是否有虫害。"),
                field("has_grass", "bool", "是否有杂草。"),
            ],
        ),
        "ManorReclaimResult" => (
            "土地开垦结果。",
            vec![
                field("ground_id", "int", "土地 ID。"),
                field("result", "int", "服务端结果值。"),
                field("exp", "int", "获得经验。"),
                field("rewards", "ManorRewardInfo[]", "奖励列表。"),
            ],
        ),
        "ManorRewardInfo" => (
            "农场奖励物品。",
            vec![
                field("item_id", "int", "奖励物品 ID。"),
                field("count", "int", "奖励数量。"),
            ],
        ),
        "ManorSowResult" => (
            "播种结果。",
            vec![
                field("exp", "int", "获得经验。"),
                field("ground", "ManorGroundInfo", "播种后的土地状态。"),
            ],
        ),
        "ManorReapResult" => (
            "收获结果。",
            vec![
                field("qq_uin", "int", "角色 UIN。"),
                field("seed_id", "int", "种子 ID。"),
                field("result", "int", "服务端结果值。"),
                field("exp", "int", "获得经验。"),
                field("fruit_num", "int", "收获数量。"),
                field("ground", "ManorGroundInfo", "收获后的土地状态。"),
                field("event_id", "int", "事件 ID。"),
                field("rewards", "ManorRewardInfo[]", "奖励列表。"),
            ],
        ),
        "ManorUprootResult" => (
            "铲除结果。",
            vec![field("ground", "ManorGroundInfo", "铲除后的土地状态。")],
        ),
        "ManorWeedResult" => (
            "除草结果。",
            vec![
                field("qq_uin", "int", "角色 UIN。"),
                field("exp", "int", "获得经验。"),
                field("ground", "ManorGroundInfo", "除草后的土地状态。"),
            ],
        ),
        "ManorFertilizerResult" => (
            "施肥结果。",
            vec![
                field("can_fertilizer", "bool", "是否可以继续施肥。"),
                field("deduce_time_in_second", "int", "减少的生长时间，单位为秒。"),
                field("fertilizer", "int", "肥料数量或类型。"),
                field("uin", "int", "角色 UIN。"),
                field("ground", "ManorGroundInfo", "施肥后的土地状态。"),
            ],
        ),
        "ManorStrawmanPlayResult" => (
            "稻草人游戏结算。",
            vec![
                field("qq_uin", "int", "角色 UIN。"),
                field("magic_id", "int", "魔法 ID。"),
                field("money", "int", "洛克贝变化。"),
                field("ground_id", "int", "关联土地 ID。"),
                field("rewards", "ManorRewardInfo[]", "奖励列表。"),
            ],
        ),
        "ManorStrawmanRewardResult" => (
            "稻草人奖励领取结果。",
            vec![field("total_exp", "int", "领取后的总经验。")],
        ),
        "ManorCocoTreeStatus" => (
            "可可树状态。",
            vec![
                field("growth_value", "int", "成长值。"),
                field("level", "int", "等级。"),
                field("can_pick_fruit", "bool", "是否可采摘。"),
                field("watered", "bool", "是否已浇水。"),
                field("time_past", "int", "已过去时间。"),
            ],
        ),
        "ManorCocoTreeReward" => (
            "可可树操作奖励。",
            vec![
                field("item_id", "int", "奖励物品 ID。"),
                field("item_count", "int", "奖励数量。"),
            ],
        ),
        "ManorCocoTreeFeedResult" => (
            "可可树操作结果。",
            vec![
                field("status", "ManorCocoTreeStatus", "操作后的状态。"),
                field("rewards", "ManorCocoTreeReward[]", "操作奖励。"),
            ],
        ),
        "ManorFriendCocoTreeStatus" => (
            "好友可可树状态。",
            vec![
                field("feed_type", "int", "可执行的照料类型。"),
                field("level", "int", "可可树等级。"),
            ],
        ),
        "ManorFriendCocoTreeFeedResult" => (
            "好友可可树照料结果。",
            vec![field("manor_exp", "int", "获得的农场经验。")],
        ),
        "ManorFriendSummary" => (
            "农场好友简表。",
            vec![
                field("uin", "int", "好友 UIN。"),
                field("score", "int", "好友积分。"),
                field("plant_score", "int", "种植积分。"),
            ],
        ),
        "ManorItemCount" => (
            "Manor seed information.",
            vec![
                field("item_id", "int", "Seed item ID."),
                field("item_count", "int", "Seed count."),
            ],
        ),
        _ => return None,
    })
}
