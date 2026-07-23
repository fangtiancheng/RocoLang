use super::{bag_candidate_fields, exchange_display_item_fields, field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "ThreeStartersField" => (
            "三主宠活动的键值字段。",
            vec![
                field("name", "string", "字段名称。"),
                field("value", "string", "字段值。"),
            ],
        ),
        "ThreeStartersCounter" => (
            "三主宠活动的计数器。",
            vec![
                field("name", "string", "计数器名称。"),
                field("current", "int", "当前计数。"),
                field("limit", "int", "计数上限。"),
            ],
        ),
        "FourSeasonsShopRewardInfo" => (
            "四季活动商店奖励。",
            vec![
                field("reward_id", "int", "奖励 ID。"),
                field("reward_kind", "int", "奖励类型。"),
                field("count", "int", "奖励数量。"),
            ],
        ),
        "FourSeasonsMonthlySpiritRewardInfo" => (
            "四季活动月度宠物奖励。",
            vec![
                field("month", "int", "月份。"),
                field("reward_index", "int", "奖励索引。"),
                field("spirit_id", "int", "宠物 ID。"),
                field("ticket_cost", "int", "兑换所需票数。"),
            ],
        ),
        "IceCrystalBattleInfo" => (
            "冰晶活动当前战斗信息。",
            vec![
                field("battle_index", "int", "战斗索引。"),
                field("fight_id", "int", "战斗 ID。"),
            ],
        ),
        "MultiEvolutionCandidate" => (
            "多元进化候选宠物。",
            vec![
                field("candidate_index", "int", "候选项索引。"),
                field("spirit_id", "int", "宠物 ID。"),
                field("catch_time", "int", "捕获时间。"),
                field("condition_code", "int", "进化条件代码。"),
                field("condition_name", "string", "进化条件名称。"),
            ],
        ),
        "CapricornTeamSnapshot" => (
            "摩羯宫队伍快照。",
            vec![
                field("players", "CapricornTeamPlayer[]", "队伍成员列表。"),
                field("ticks", "int", "队伍状态时间标记。"),
            ],
        ),
        "CapricornSecondTask" => (
            "摩羯宫二阶任务。",
            vec![
                field("task_type", "int", "任务类型。"),
                field("data1", "int", "任务数据 1。"),
                field("data2", "int", "任务数据 2。"),
                field("step", "int", "任务步骤。"),
                field("current", "int", "当前进度。"),
            ],
        ),
        "StarTowerTop" => (
            "星辰塔顶层挑战信息。",
            vec![
                field("star", "int", "星级。"),
                field("refresh", "int", "刷新状态。"),
                field("fight_desc", "string", "战斗说明。"),
                field("task_desc", "string", "任务说明。"),
                field("fight_id", "int", "战斗 ID。"),
                field("tokens", "int[]", "令牌列表。"),
                field("exchanges", "int[]", "兑换状态列表。"),
                field("missions", "StarTowerTopMission[]", "顶层任务列表。"),
                field("rewards", "StarTowerTopReward[]", "顶层奖励列表。"),
            ],
        ),
        "AdventureItem" => (
            "冒险系统奖励物品。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "物品数量。"),
            ],
        ),
        "AdventureStatus" => (
            "冒险系统当前状态。",
            vec![
                field("last_point", "int", "当前可挑战的最后关卡。"),
                field("got_daily", "bool", "是否已经领取每日奖励。"),
                field("props", "int[]", "三种冒险消耗品的数量。"),
                field("auto_running", "bool", "是否正在自动挑战。"),
                field("vip", "bool", "冒险系统返回的 VIP 状态。"),
                field("guide_level", "int", "冒险引导等级。"),
                field("medal_bits", "int", "已获得勋章的位图。"),
                field("first", "int", "首次进入状态。"),
            ],
        ),
        "AdventureRewards" => (
            "冒险系统奖励结果。",
            vec![field("items", "AdventureItem[]", "奖励物品列表。")],
        ),
        "StarTowerInfo" => ("星辰塔返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("mop", "int", "魔法值。"),
                field("boss_id", "int", "Boss ID。"),
                field("countdown", "int", "倒计时。"),
                field("auto_sell", "bool", "是否自动出售。"),
                field("money", "int", "洛克贝。"),
                field("clips", "int[]", "碎片列表。"),
                field("storeys", "StarTowerStorey[]", "楼层列表。"),
                field("has_top", "bool", "是否包含顶层挑战信息。"),
                field("top", "RocoOptionalStarTowerTop", "结构化可选顶层信息。"),
            ]
        }),
        "StarTowerStorey" => (
            "星辰塔楼层信息。",
            vec![
                field("storey_index", "int", "楼层索引。"),
                field("first", "int", "首通状态。"),
                field("can_quick_fight", "bool", "是否可快速挑战。"),
                field("nodes", "StarTowerNode[]", "Boss 节点列表。"),
                field(
                    "exchange_items",
                    "StarTowerExchangeItem[]",
                    "楼层兑换碎片列表。",
                ),
            ],
        ),
        "StarTowerNode" => (
            "星辰塔 Boss 节点。",
            vec![
                field("node_index", "int", "节点索引。"),
                field("star", "int", "星级。"),
                field("spirit_id", "int", "宠物 ID。"),
                field("fight_id", "int", "战斗 ID。"),
                field("item_id", "int", "奖励物品 ID。"),
                field("reward", "int", "奖励状态。"),
                field("equip_id", "int", "装备 ID。"),
            ],
        ),
        "StarTowerExchangeItem" => (
            "星辰塔兑换碎片。",
            vec![
                field("index", "int", "兑换索引。"),
                field("item_id", "int", "碎片物品 ID。"),
                field("item_name", "string", "碎片物品名称。"),
                field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
                field("spirit_name", "string", "宠物名称。"),
                field("owned", "int", "已拥有碎片数量。"),
                field("required", "int", "所需碎片数量。"),
            ],
        ),
        "CapricornBagCandidate" => ("摩羯宫背包候选宠物。", bag_candidate_fields()),
        "CapricornTeamOperationInfo" => ("摩羯宫队伍操作返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "team",
                    "RocoOptionalCapricornTeamSnapshot",
                    "结构化可选队伍快照。",
                ),
            ]
        }),
        "CapricornSecondInfo" => ("摩羯宫二阶返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("current", "RocoOptionalI64", "当前进度。"),
                field("position", "RocoOptionalI64", "当前位置。"),
                field(
                    "second_task",
                    "RocoOptionalCapricornSecondTask",
                    "结构化可选二阶任务。",
                ),
                field(
                    "bag_candidates",
                    "CapricornBagCandidate[]",
                    "背包候选宠物。",
                ),
            ]
        }),
        "CapricornThirdInfo" => ("摩羯宫三阶返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("current", "RocoOptionalI64", "当前进度。"),
                field("remain", "RocoOptionalI64", "剩余数量。"),
                field("price", "RocoOptionalI64", "价格。"),
                field("limit", "RocoOptionalI64", "上限。"),
                field("progress_percent", "RocoOptionalI64", "进度百分比。"),
                field("reward_num", "RocoOptionalI64", "奖励数量。"),
                field("tips", "RocoOptionalI64", "提示状态。"),
                field(
                    "bag_candidates",
                    "CapricornBagCandidate[]",
                    "背包候选宠物。",
                ),
            ]
        }),
        "PiscesBagCandidate" => ("双鱼宫背包候选宠物。", bag_candidate_fields()),
        "TaurusBagCandidate" => ("金牛宫背包候选宠物。", bag_candidate_fields()),
        "ThreeStartersBagCandidate" => ("三主宠活动背包候选宠物。", bag_candidate_fields()),
        "AlchemyFurnaceBagCandidate" => ("炼丹炉背包候选宠物。", bag_candidate_fields()),
        "UnicornBagCandidate" => ("独角兽活动背包候选宠物。", bag_candidate_fields()),
        "IceCrystalBagCandidate" => ("冰晶活动背包候选宠物。", bag_candidate_fields()),
        "GeminiBagCandidate" => ("双子宫背包候选宠物。", bag_candidate_fields()),
        "SagittariusBagCandidate" => ("射手宫背包候选宠物。", bag_candidate_fields()),
        "ScorpioBagCandidate" => ("天蝎宫背包候选宠物。", bag_candidate_fields()),
        "AriesBagCandidate" => ("白羊宫背包候选宠物。", bag_candidate_fields()),
        "LibraBagCandidate" => ("天秤宫背包候选宠物。", bag_candidate_fields()),
        "LeoBagCandidate" => ("狮子宫背包候选宠物。", bag_candidate_fields()),
        "VirgoBellFoxExchangeInfo" => ("处女宫铃狐兑换结果。", exchange_display_item_fields()),
        "AquariusSecondExchangeInfo" => ("水瓶宫二阶兑换结果。", exchange_display_item_fields()),
        "AriesThirdExchangeInfo" => ("白羊宫三阶兑换结果。", exchange_display_item_fields()),
        "LibraThirdExchangeInfo" => ("天秤宫三阶兑换结果。", exchange_display_item_fields()),
        "LeoFirstExchangeInfo" => ("狮子宫一阶兑换结果。", exchange_display_item_fields()),
        "MonkeyCultivationInfo" => ("灵猴修炼返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("pill_counts", "int[]", "丹药数量。"),
                field("daytimes", "RocoOptionalI64", "当日次数。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("progress", "RocoOptionalI64", "进度。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "MonkeyEvoInfo" => ("灵猴进化返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("pill_counts", "int[]", "丹药数量。"),
                field("branch_type", "RocoOptionalI64", "进化分支类型。"),
                field("done", "RocoOptionalI64", "完成状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field(
                    "bag_candidates",
                    "AlchemyFurnaceBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "RagingFireInfo" => ("烈火活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("vip", "RocoOptionalI64", "VIP 状态。"),
                field("daytimes", "RocoOptionalI64", "当日次数。"),
                field("required_stone_indexes", "int[]", "所需石头索引。"),
                field("progress", "int[]", "三段进度。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("fusion", "RocoOptionalI64", "融合状态。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field(
                    "bag_candidates",
                    "AlchemyFurnaceBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "UnicornBossInfo" => ("独角兽 Boss 信息。", {
            vec![
                field("slot", "int", "槽位。"),
                field("npc_index", "int", "NPC 索引。"),
                field("spirit_id", "RocoOptionalI64", "Boss 宠物 ID。"),
                field("fight_id", "RocoOptionalI64", "战斗 ID。"),
            ]
        }),
        "UnicornInfo" => ("独角兽活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("bosses", "UnicornBossInfo[]", "Boss 列表。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("start", "RocoOptionalI64", "开始状态。"),
                field("total", "RocoOptionalI64", "总数。"),
                field("book", "RocoOptionalI64", "图鉴状态。"),
                field("cultivation_times", "int[]", "培养次数。"),
                field("evolution_energy_costs", "int[]", "进化能量消耗。"),
                field("one_key_diamond_costs", "int[]", "一键钻石消耗。"),
                field("purple_vine_count", "RocoOptionalI64", "紫藤数量。"),
                field("energy", "RocoOptionalI64", "能量。"),
                field("fruit_count", "RocoOptionalI64", "果实数量。"),
                field("increase", "RocoOptionalI64", "增量。"),
                field("bag_candidates", "UnicornBagCandidate[]", "背包候选宠物。"),
                field("rewards", "UnicornRewardItem[]", "奖励列表。"),
            ]
        }),
        "WaterSourceInfo" => ("水源活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("battle", "RocoOptionalI64", "战斗状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("time", "RocoOptionalI64", "时间。"),
                field("increase", "RocoOptionalI64", "增量。"),
                field("water", "int[]", "水源状态数组。"),
            ]
        }),
        "FiresWillInfo" => ("火焰意志返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("num", "RocoOptionalI64", "数量。"),
                field("fire", "int[]", "火焰状态数组。"),
            ]
        }),
        "BatheSunInfo" => ("日光浴返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("battle", "RocoOptionalI64", "战斗状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("time", "RocoOptionalI64", "时间。"),
                field("num", "RocoOptionalI64", "数量。"),
                field("act", "RocoOptionalI64", "动作状态。"),
                field("times", "RocoOptionalI64", "次数。"),
                field("sun", "RocoOptionalI64", "日光值。"),
                field("add", "RocoOptionalI64", "增量。"),
            ]
        }),
        "FourSeasonsInfo" => ("四季活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("month", "RocoOptionalI64", "月份。"),
                field("map", "RocoOptionalI64", "地图。"),
                field("position_1based", "RocoOptionalI64", "1 基位置。"),
                field("times", "RocoOptionalI64", "次数。"),
                field("ticket", "RocoOptionalI64", "票数。"),
                field("used_tool_index", "RocoOptionalI64", "已使用工具索引。"),
                field("need_item_index", "RocoOptionalI64", "所需道具索引。"),
                field("add", "RocoOptionalI64", "增量。"),
                field("point", "RocoOptionalI64", "点数。"),
                field("boxes", "int[]", "格子状态。"),
                field("tools", "int[]", "工具状态。"),
                field("tool_shop_indexes", "int[]", "工具商店索引。"),
                field("tool_shop_flags", "int[]", "工具商店标记。"),
                field("pass_boxes", "int[]", "已通过格子。"),
                field("tool_costs", "int[]", "工具消耗。"),
                field("event_item_counts", "int[]", "事件道具数量。"),
                field("shop_rewards", "FourSeasonsShopRewardInfo[]", "商店奖励。"),
                field(
                    "monthly_spirit_rewards",
                    "FourSeasonsMonthlySpiritRewardInfo[]",
                    "月度宠物奖励。",
                ),
                field("rewards", "FourSeasonsRewardItem[]", "奖励列表。"),
            ]
        }),
        "DiamondTearInfo" => ("钻石泪活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("buy", "RocoOptionalI64", "购买状态。"),
                field("level", "RocoOptionalI64", "等级。"),
                field("count_down", "RocoOptionalI64", "倒计时。"),
                field("tear_state", "RocoOptionalI64", "钻石泪状态。"),
                field("rewards", "DiamondTearRewardItem[]", "奖励列表。"),
            ]
        }),
        "IceCrystalInfo" => ("冰晶活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("progress", "RocoOptionalI64", "进度。"),
                field("battle_times", "RocoOptionalI64", "战斗次数。"),
                field("battle_index", "RocoOptionalI64", "战斗索引。"),
                field("get_times", "RocoOptionalI64", "领取次数。"),
                field("add", "RocoOptionalI64", "增量。"),
                field("item_counts", "int[]", "道具数量。"),
                field("crystal_counts", "int[]", "冰晶数量。"),
                field("item_costs", "int[]", "道具消耗。"),
                field("one_key_diamond_costs", "int[]", "一键钻石消耗。"),
                field(
                    "current_battle",
                    "RocoOptionalIceCrystalBattleInfo",
                    "结构化可选当前战斗。",
                ),
                field(
                    "bag_candidates",
                    "IceCrystalBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "IceCrystalRewardItem[]", "奖励列表。"),
            ]
        }),
        "MultiEvolutionInfo" => ("多元进化返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("candidates", "MultiEvolutionCandidate[]", "候选宠物列表。"),
                field("rewards", "MultiEvolutionRewardItem[]", "奖励列表。"),
                field("pet_id", "RocoOptionalI64", "进化结果宠物 ID。"),
                field("result_side", "RocoOptionalI64", "水系进化结果侧。"),
                field("item_id", "RocoOptionalI64", "助燃道具 ID。"),
                field("count", "int", "道具数量或服务器返回计数。"),
                field("available", "bool", "奖励是否可领取。"),
            ]
        }),
        "CancerSharpScorpionInfo" => ("巨蟹宫尖角蜘蛛状态。", {
            let fields = vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("light_num", "int", "光数量。"),
                field("tail_num", "int", "尾巴数量。"),
                field("boss_left_hp", "int", "Boss 剩余 HP。"),
                field("boss_full_hp", "int", "Boss 总 HP。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
                field("add_hit_level", "int", "追加命中等级。"),
                field("today_sum_hit", "int", "今日累计命中。"),
                field("exchange_count0", "int", "兑换计数 0。"),
                field("exchange_count1", "int", "兑换计数 1。"),
                field(
                    "display_item",
                    "RocoOptionalDisplayItem",
                    "结构化可选展示物品。",
                ),
            ];
            fields
        }),
        _ => return None,
    })
}
