use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "RocoRequestContext" => (
            "结构化请求上下文。",
            vec![
                field("raw", "string", "原始请求上下文。"),
                field("domain", "string", "第一个点号前的业务域。"),
                field(
                    "action",
                    "string",
                    "第一个点号后的动作名；旧单段上下文为空。",
                ),
            ],
        ),
        "RocoRewardKind" => (
            "结构化奖励类型。",
            vec![field(
                "code",
                "string",
                "稳定脚本代码，例如 item、money、assignable_exp、spirit_equipment。",
            )],
        ),
        "RocoOptionalI64" => (
            "结构化可选整数。",
            vec![
                field("present", "bool", "是否存在服务器返回值。"),
                field("value", "int | ()", "服务器返回值；缺失时为 ()。"),
            ],
        ),
        "RocoDisplayItem" => (
            "展示用奖励物品。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("item_count", "int", "数量。"),
                field("item_type", "int", "物品类型。"),
            ],
        ),
        "RocoOptionalDisplayItem" => (
            "结构化可选展示物品。",
            vec![
                field("present", "bool", "是否存在展示物品。"),
                field("value", "RocoDisplayItem | ()", "展示物品；缺失时为 ()。"),
            ],
        ),
        "RocoOptionalIceCrystalBattleInfo" => (
            "结构化可选冰晶战斗信息。",
            vec![
                field("present", "bool", "是否存在当前战斗。"),
                field(
                    "value",
                    "IceCrystalBattleInfo | ()",
                    "当前战斗；缺失时为 ()。",
                ),
            ],
        ),
        "RocoOptionalCapricornSecondTask" => (
            "结构化可选摩羯宫二阶任务。",
            vec![
                field("present", "bool", "是否存在二阶任务。"),
                field(
                    "value",
                    "CapricornSecondTask | ()",
                    "二阶任务；缺失时为 ()。",
                ),
            ],
        ),
        "RocoOptionalStarTowerTop" => (
            "结构化可选星辰塔顶层信息。",
            vec![
                field("present", "bool", "是否存在顶层信息。"),
                field("value", "StarTowerTop | ()", "顶层信息；缺失时为 ()。"),
            ],
        ),
        "RocoOptionalCapricornTeamSnapshot" => (
            "结构化可选摩羯宫队伍快照。",
            vec![
                field("present", "bool", "是否存在队伍快照。"),
                field(
                    "value",
                    "CapricornTeamSnapshot | ()",
                    "队伍快照；缺失时为 ()。",
                ),
            ],
        ),
        "RocoOptionalTypeLadderRankUser" => (
            "结构化可选系别天梯个人排行信息。",
            vec![
                field("present", "bool", "是否存在个人排行信息。"),
                field(
                    "value",
                    "TypeLadderRankUser | ()",
                    "个人排行信息；缺失时为 ()。",
                ),
            ],
        ),
        _ => return None,
    })
}
