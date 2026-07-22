use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "AmendNatureCandidate" => (
            "可修改性格的宠物候选项。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("catch_time", "int", "捕获时间。"),
                field("level", "int", "等级。"),
                field("personality", "int", "当前性格编号。"),
                field("personality_name", "string", "当前性格名称。"),
                field("need_money", "int", "所需洛克贝。"),
            ],
        ),
        "BattleSpiritResult" => (
            "战斗结束后的单只宠物结算。",
            vec![
                field("position", "int", "背包号位。"),
                field("exp", "int", "获得经验。"),
                field("level_delta", "int", "等级变化。"),
                field("level", "int", "结算后等级。"),
                field("next_exp", "int", "下一级所需经验。"),
                field("effort", "int", "获得努力值。"),
                field("new_skill_ids", "int[]", "新学会的技能 ID。"),
                field("evolve_spirit_id", "int", "进化后的宠物 ID。"),
            ],
        ),
        "BattleCapturedSpirit" => (
            "战斗中捕获的宠物信息。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("level", "int", "等级。"),
                field("disposition", "int", "性格编号。"),
                field("property_list", "int[]", "能力值列表。"),
                field("flair_list", "int[]", "天赋列表。"),
            ],
        ),
        "BloodGiftOption" => (
            "血脉天赋选项。",
            vec![
                field("blood_index", "int", "血脉索引。"),
                field("talent_type", "int", "天赋类型。"),
                field("talent_name", "string", "天赋名称。"),
                field("talent_description", "string", "天赋说明。"),
                field("awakened", "bool", "是否已觉醒。"),
                field(
                    "required_items",
                    "BloodGiftItemRequirement[]",
                    "觉醒所需物品。",
                ),
            ],
        ),
        "BloodGiftItemRequirement" => (
            "血脉觉醒所需物品。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "当前数量。"),
                field("need", "int", "所需数量。"),
            ],
        ),
        "LadderQuestConfigEntry" => (
            "天梯任务配置项。",
            vec![
                field("id", "int", "任务 ID。"),
                field("diff", "int", "任务差值。"),
                field("description", "string", "任务说明。"),
            ],
        ),
        "LadderSpiritCostEntry" => (
            "天梯宠物消耗配置项。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("cost", "int", "消耗值。"),
            ],
        ),
        "MiniGameRewardItem" => (
            "小游戏奖励物品。",
            vec![
                field("id", "int", "物品 ID。"),
                field("count", "int", "数量。"),
                field("item_type", "int", "物品类型。"),
            ],
        ),
        "MiniGameExtraField" => (
            "小游戏额外返回字段。",
            vec![
                field("key", "string", "字段名。"),
                field("value", "int", "字段值。"),
            ],
        ),
        "NewsTimesReport" => (
            "新闻活动时间条目。",
            vec![
                field("id", "int", "活动 ID。"),
                field("report_type", "int", "活动类型。"),
                field("begin_time", "int", "开始时间。"),
                field("end_time", "int", "结束时间。"),
                field("act_begin_time", "int[]", "分段开始时间。"),
                field("act_end_time", "int[]", "分段结束时间。"),
                field("name_image_url", "string", "名称图片地址。"),
                field("app_url", "string", "活动地址。"),
            ],
        ),
        "SpiritBookGroup" => (
            "宠物图鉴分组。",
            vec![
                field("template_id", "int", "分组模板 ID。"),
                field("spirits", "SpiritBookEntry[]", "分组内宠物。"),
            ],
        ),
        _ => return None,
    })
}
