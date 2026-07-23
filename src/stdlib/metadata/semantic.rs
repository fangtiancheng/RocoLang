use super::StdlibFunctionRegistration;

pub(super) fn module_display_name(module: &str) -> Option<&'static str> {
    Some(match module {
        "adventure" => "冒险",
        "alchemy_furnace" => "炼丹炉",
        "aquarius" => "水瓶宫",
        "aries" => "白羊宫",
        "cancer" => "巨蟹宫",
        "capricorn" => "摩羯宫",
        "combat" => "战斗",
        "combat_result" => "战斗结果枚举",
        "combat_status" => "战斗状态枚举",
        "dark_city" => "暗黑城",
        "diamond_tear" => "钻石之泪",
        "four_seasons" => "四时邀约",
        "friend" => "好友",
        "game" => "游戏运行时",
        "gemini" => "双子宫",
        "home" => "家园",
        "ice_crystal" => "冰晶活动",
        "incubative_machine" => "孵化机",
        "jump_machine" => "弹弹机",
        "ladder" => "天梯赛",
        "leo" => "狮子宫",
        "libra" => "天秤宫",
        "lookup" => "静态数据查询",
        "magic_pioneer" => "魔法先锋队",
        "manor" => "农场",
        "memory" => "远程状态存储",
        "mountain_sea" => "山海秘境",
        "multi_evolution" => "多元进化",
        "mystery_fusion" => "神秘融合",
        "news" => "新闻与活动状态",
        "news_times" => "新闻次数枚举",
        "personality" => "宠物性格枚举",
        "pet_egg" => "宠物培育",
        "pet_training" => "宠物训练",
        "pisces" => "双鱼宫",
        "play_guide" => "玩法引导",
        "profile" => "账号资料",
        "remote_state" => "远程状态",
        "role" => "角色",
        "sagittarius" => "射手宫",
        "scene" => "场景",
        "scorpio" => "天蝎宫",
        "sentinel_intelligence" => "哨兵情报",
        "session" => "会话状态",
        "spirit" => "宠物",
        "spirit_book" => "宠物图鉴",
        "star_tower" => "星辰塔",
        "summon" => "召唤",
        "system" => "系统",
        "task" => "任务",
        "taurus" => "金牛宫",
        "three_starters" => "三主宠活动",
        "treasure_realm" => "珍宝秘境",
        "type_ladder" => "系别天梯赛",
        "unicorn" => "独角兽活动",
        "virgo" => "处女宫",
        "weather" => "天气枚举",
        _ => return None,
    })
}

pub(super) fn function_description(registration: StdlibFunctionRegistration) -> String {
    let module = module_display_name(registration.module).unwrap_or(registration.module);
    let phase = phase_description(registration.name);
    let action = action_description(registration.name);
    match phase {
        Some(phase) => format!("{action}{module}{phase}数据或状态。"),
        None => format!("{action}{module}数据或状态。"),
    }
}

pub(super) fn parameter_description(name: &str) -> String {
    if let Some(description) = exact_parameter_description(name) {
        return description.to_string();
    }
    if let Some(subject) = name.strip_suffix("_id") {
        return format!("{} ID。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_uin") {
        return format!("{}的 UIN。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_index") {
        return format!("{}索引。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_position") {
        return format!("{}位置。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_type") {
        return format!("{}类型。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_count") {
        return format!("{}数量。", subject_display_name(subject));
    }
    if let Some(subject) = name.strip_suffix("_time") {
        return format!("{}时间戳。", subject_display_name(subject));
    }
    format!("{}。", subject_display_name(name))
}

pub(super) fn return_description(name: &str, return_type: Option<&str>) -> String {
    match return_type {
        Some("()") => "操作完成后不返回数据。".to_string(),
        Some("bool") => "返回操作是否成功。".to_string(),
        Some("int") => primitive_return_description(name, "整数"),
        Some("string") => primitive_return_description(name, "文本"),
        Some(return_type) => format!("返回 {return_type}；字段语义见对应类型文档。"),
        None if is_mutation(name) => "返回服务端对本次操作的处理结果。".to_string(),
        None => "返回本次查询得到的数据。".to_string(),
    }
}

fn phase_description(name: &str) -> Option<&'static str> {
    if name.starts_with("first_") {
        Some("第一阶段")
    } else if name.starts_with("second_") {
        Some("第二阶段")
    } else if name.starts_with("third_") {
        Some("第三阶段")
    } else if name.starts_with("monkey_cultivation_") {
        Some("灵猴修炼")
    } else if name.starts_with("monkey_evo_") {
        Some("灵猴进化")
    } else if name.starts_with("raging_fire_") {
        Some("烈火试炼")
    } else {
        None
    }
}

fn action_description(name: &str) -> &'static str {
    if contains_any(
        name,
        &["query", "get_info", "get_status", "snapshot", "list"],
    ) {
        "查询"
    } else if contains_any(name, &["claim", "get_gift", "get_reward", "take_reward"]) {
        "领取"
    } else if contains_any(name, &["buy", "purchase"]) {
        "购买"
    } else if contains_any(name, &["exchange", "_exc_", "exc_pet"]) {
        "兑换"
    } else if contains_any(name, &["evolve", "upgrade", "level_up", "grow_up"]) {
        "提升"
    } else if contains_any(name, &["settle", "report_fight", "report_battle"]) {
        "提交战斗结算并更新"
    } else if contains_any(name, &["start", "begin", "open"]) {
        "开始"
    } else if contains_any(name, &["submit", "commit", "apply", "report", "set_"]) {
        "提交并更新"
    } else if contains_any(name, &["recover", "restore", "heal"]) {
        "恢复"
    } else if contains_any(
        name,
        &["cancel", "terminate", "give_up", "leave", "disband"],
    ) {
        "取消或结束"
    } else if contains_any(name, &["invite", "accept", "refuse"]) {
        "处理邀请并更新"
    } else if contains_any(name, &["remove", "delete", "uproot"]) {
        "移除"
    } else if contains_any(name, &["add", "increase"]) {
        "增加"
    } else if contains_any(name, &["random", "draw", "dice", "refresh"]) {
        "随机刷新"
    } else if contains_any(name, &["feed", "teach", "train", "cultivate"]) {
        "培养并更新"
    } else {
        "执行并更新"
    }
}

fn is_mutation(name: &str) -> bool {
    !contains_any(
        name,
        &["query", "get_", "can_", "is_", "has_", "lookup", "name"],
    )
}

fn primitive_return_description(name: &str, kind: &str) -> String {
    if name.starts_with("is_") || name.starts_with("can_") || name.starts_with("has_") {
        format!("返回判定结果对应的{kind}值。")
    } else {
        format!("返回接口计算或查询得到的{kind}值。")
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn exact_parameter_description(name: &str) -> Option<&'static str> {
    Some(match name {
        "catch_time" | "spirit_catch_time" | "equipment_catch_time" => {
            "实例捕获时间，用于与 ID 联合定位具体实例。"
        }
        "count" | "num" | "number" => "本次操作数量。",
        "enabled" => "是否启用该功能。",
        "key" => "状态或配置键。",
        "kind" => "业务类别。",
        "position" | "slot" => "背包、队伍或槽位位置。",
        "target" => "操作目标。",
        "uin" => "玩家 UIN。",
        "value" => "要写入或提交的值。",
        "values" => "要批量写入或提交的值。",
        "which" => "目标槽位编号。",
        "ms" => "等待时长，单位为毫秒。",
        "timeout_ms" => "超时时长，单位为毫秒。",
        "target_ms" => "目标时间戳，单位为毫秒。",
        "min_inclusive" => "随机区间下界，包含该值。",
        "max_inclusive" => "随机区间上界，包含该值。",
        "message" => "要输出或记录的文本。",
        "accept" => "是否接受请求。",
        "success" | "win" => "本次操作或战斗是否成功。",
        "refresh" => "是否强制刷新数据。",
        "vip" | "vip_boost" => "是否使用 VIP 能力或加成。",
        _ => return None,
    })
}

fn subject_display_name(subject: &str) -> String {
    subject
        .split('_')
        .map(|token| match token {
            "area" => "区域",
            "answer" => "答案",
            "bag" => "背包",
            "battle" => "战斗",
            "blood" => "血脉",
            "book" => "图鉴",
            "boss" => "首领",
            "buy" => "购买",
            "challenge" => "挑战",
            "combat" => "战斗",
            "condition" => "条件",
            "egg" => "宠物蛋",
            "equipment" => "装备",
            "exchange" | "exc" => "兑换",
            "fight" => "战斗",
            "friend" => "好友",
            "game" => "游戏",
            "ground" => "土地",
            "item" => "物品",
            "level" => "等级",
            "magic" => "魔法",
            "male" => "雄性宠物",
            "female" => "雌性宠物",
            "npc" => "NPC",
            "page" => "页码",
            "pet" | "spirit" => "宠物",
            "reward" => "奖励",
            "scene" => "场景",
            "skill" => "技能",
            "stage" => "阶段",
            "star" => "星级",
            "story" => "故事线",
            "task" => "任务",
            "training" => "训练",
            "type" => "类型",
            "weather" => "天气",
            "wish" => "祈愿",
            other => other,
        })
        .collect::<Vec<_>>()
        .join(" ")
}
