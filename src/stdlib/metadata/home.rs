use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "home",
            "get_overview",
            "home::get_overview(area_id: int) -> HomeOverview",
            "查询当前角色指定区域的家园概览。",
            params: ["area_id" => "家园区域 ID。"],
            returns: "家园等级、经验、能量、家具和星工场信息。",
            examples: ["let overview = home::get_overview(1);"]
        ),
        super::stdlib_doc!(
            "home",
            "get_friend_list",
            "home::get_friend_list() -> HomeFriendSummary[]",
            "查询家园好友及其家园经验。",
            params: [],
            returns: "家园好友摘要列表。",
            examples: ["let friends = home::get_friend_list();"]
        ),
        super::stdlib_doc!(
            "home",
            "get_training_spirits",
            "home::get_training_spirits() -> HomeTrainingSpirit[]",
            "查询当前角色放在家园锻炼的宠物。",
            params: [],
            returns: "家园锻炼宠物列表。",
            examples: ["let spirits = home::get_training_spirits();"]
        ),
        super::stdlib_doc!(
            "home",
            "get_training_spirit_report",
            "home::get_training_spirit_report(spirit_id: int, catch_time: int) -> HomeTrainingSpiritReport",
            "查询一只家园锻炼宠物的详情与报告。",
            params: [
                "spirit_id" => "宠物 ID。",
                "catch_time" => "宠物捕获时间。"
            ],
            returns: "家园锻炼宠物报告。",
            examples: ["let report = home::get_training_spirit_report(30, catch_time);"]
        ),
        super::stdlib_doc!(
            "home",
            "take_training_spirit",
            "home::take_training_spirit(spirit_id: int, catch_time: int) -> HomeTakeTrainingSpiritResult",
            "收回一只正在家园锻炼的宠物。",
            params: [
                "spirit_id" => "宠物 ID。",
                "catch_time" => "宠物捕获时间。"
            ],
            returns: "宠物收回后的目标位置。",
            examples: ["let result = home::take_training_spirit(30, catch_time);"]
        ),
        super::stdlib_doc!(
            "home",
            "query_coach_spirits",
            "home::query_coach_spirits(refresh: bool) -> HomeCoachSpiritList",
            "查询或刷新家园教练切磋宠物列表。",
            params: ["refresh" => "是否消耗刷新机会刷新列表。"],
            returns: "教练切磋经验、限制和宠物 ID 列表。",
            examples: ["let list = home::query_coach_spirits(false);"]
        ),
    ]
}
