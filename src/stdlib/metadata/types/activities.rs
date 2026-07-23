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
            "四时邀约商店奖励。",
            vec![
                field("reward_id", "int", "奖励 ID。"),
                field("reward_kind", "int", "奖励类型。"),
                field("count", "int", "奖励数量。"),
            ],
        ),
        "FourSeasonsMonthlySpiritRewardInfo" => (
            "四时邀约月度宠物奖励。",
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
        "DarkCityExpeditionInfo" => ("暗黑城远征返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("fight_id", "int", "当前战斗 ID。"),
                field("fight_index", "int", "当前战斗索引。"),
                field("vip", "bool", "是否拥有 VIP 状态。"),
                field("vip_pass_enabled", "bool", "是否启用 VIP 通行。"),
                field("schedule", "int", "远征进度。"),
                field("schedule_name", "string", "远征进度名称。"),
                field("added_reputation", "int", "本次新增声望。"),
            ]
        }),
        "DarkCityReputationInfo" => ("暗黑城声望返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("reputation", "int", "当前声望。"),
                field("exchanges", "DarkCityExchangeItem[]", "声望兑换项列表。"),
            ]
        }),
        "DarkCityExchangeItem" => ("暗黑城声望兑换项。", {
            vec![
                field("index", "int", "兑换项索引。"),
                field("item_id", "int", "兑换物品 ID。"),
                field("cost", "int", "兑换所需声望。"),
            ]
        }),
        "MountainSeaInfo" => ("山海秘境返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("fight_id", "int", "当前战斗 ID。"),
                field("seal_count", "int", "封印数量。"),
                field("success", "int", "最近操作是否成功。"),
                field("attrs", "int[]", "秘境属性列表。"),
                field("bosses", "MountainSeaBossInfo[]", "首领列表。"),
                field("souls", "MountainSeaSoulInfo[]", "魂魄列表。"),
            ]
        }),
        "MountainSeaBossInfo" => ("山海秘境首领信息。", {
            vec![
                field("index", "int", "首领索引。"),
                field("boss_type", "int", "首领类型。"),
                field("fight_id", "int", "战斗 ID。"),
                field("name", "string", "首领名称。"),
                field("status", "int", "首领状态。"),
            ]
        }),
        "MountainSeaSoulInfo" => ("山海秘境魂魄信息。", {
            vec![
                field("soul_type", "int", "魂魄类型。"),
                field("boss_type", "int", "对应首领类型。"),
                field("name", "string", "魂魄名称。"),
                field("count", "int", "魂魄数量。"),
            ]
        }),
        "MysteryFusionInfo" => ("神秘融合返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("times", "int", "已进行的融合次数。"),
                field("energy", "int", "当前融合能量。"),
                field("added_energy", "int", "本次新增能量。"),
                field("battles", "MysteryFusionBattleInfo[]", "可进行的战斗列表。"),
                field("recipes", "MysteryFusionRecipeInfo[]", "融合配方列表。"),
            ]
        }),
        "MysteryFusionBattleInfo" => ("神秘融合战斗信息。", {
            vec![
                field("index", "int", "战斗索引。"),
                field("battle_id", "int", "战斗 ID。"),
                field("attr_types", "int[]", "战斗属性类型。"),
            ]
        }),
        "MysteryFusionRecipeInfo" => ("神秘融合配方。", {
            vec![
                field("index", "int", "配方索引。"),
                field("spirit_id", "int", "融合产物宠物 ID。"),
                field("energy_cost", "int", "所需能量。"),
                field("required_spirit_ids", "int[]", "所需材料宠物 ID。"),
            ]
        }),
        "MysteryFusionMaterialBag" => ("神秘融合材料背包。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "candidates",
                    "MysteryFusionMaterialCandidate[]",
                    "可用材料候选列表。",
                ),
            ]
        }),
        "MysteryFusionMaterialCandidate" => ("神秘融合材料候选宠物。", {
            vec![
                field("candidate_index", "int", "候选索引。"),
                field("spirit_id", "int", "宠物 ID。"),
                field("bag_index", "int", "背包位置。"),
                field("level", "int", "宠物等级。"),
                field("personality", "int", "宠物性格 ID。"),
            ]
        }),
        "TreasureRealmInfo" => ("珍宝秘境返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("battle", "int", "当前战斗状态。"),
                field("battle_id", "int", "战斗 ID。"),
                field("schedule", "int", "秘境进度。"),
                field("possible", "int", "可操作状态。"),
                field("time", "int", "剩余时间。"),
                field("got_box", "bool", "是否已经领取宝箱。"),
                field("item_counts", "int[]", "物品数量列表。"),
                field("commits", "int[]", "提交状态列表。"),
            ]
        }),
        "NewsTimesReportsResult" => ("新闻次数查询结果。", {
            vec![
                field("reports", "NewsTimesReport[]", "新闻活动次数列表。"),
                field("gift_gotten", "bool", "是否已经领取今日礼包。"),
                field("player_status_today", "int[]", "今日玩家状态。"),
                field("player_status_forever", "int[]", "累计玩家状态。"),
            ]
        }),
        "NewsTimesReport" => ("新闻活动次数信息。", {
            vec![
                field("act_id", "int", "活动 ID。"),
                field("act_begin_time", "int", "活动开始时间。"),
                field("act_end_time", "int", "活动结束时间。"),
                field("times", "int", "当前次数。"),
            ]
        }),
        "SentinelIntelligenceInfo" => ("哨兵情报活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("fight_id", "int", "当前战斗 ID。"),
                field("added_bounty", "int", "本次新增悬赏值。"),
                field("refresh_count", "int", "首领刷新次数。"),
                field("exchange_refresh_count", "int", "兑换刷新次数。"),
                field("mission_type", "int", "当前任务类型。"),
                field("mission_values", "int[]", "任务参数。"),
                field("fight_times", "int", "剩余战斗次数。"),
                field("bounty", "int", "当前悬赏值。"),
                field("intelligence_count", "int", "当前情报数量。"),
                field("bosses", "SentinelBossInfo[]", "首领列表。"),
                field("exchanges", "SentinelExchangeInfo[]", "物品兑换列表。"),
                field("spirits", "SentinelSpiritExchangeInfo[]", "宠物兑换列表。"),
            ]
        }),
        "SentinelBossInfo" => ("哨兵情报活动首领信息。", {
            vec![
                field("index", "int", "首领索引。"),
                field("spirit_id", "int", "首领宠物 ID。"),
                field("difficulty", "int", "首领难度。"),
                field("status", "int", "首领状态。"),
                field("max_intelligence", "int", "情报上限。"),
                field("intelligence", "int", "当前情报值。"),
            ]
        }),
        "SentinelExchangeInfo" => ("哨兵情报活动物品兑换项。", {
            vec![
                field("index", "int", "兑换项索引。"),
                field("item_id", "int", "物品 ID。"),
                field("need_bounty", "int", "所需悬赏值。"),
                field("status", "int", "兑换状态。"),
            ]
        }),
        "SentinelSpiritExchangeInfo" => ("哨兵情报活动宠物兑换项。", {
            vec![
                field("index", "int", "兑换项索引。"),
                field("spirit_id", "int", "兑换宠物 ID。"),
                field("need_intelligence", "int", "所需情报数量。"),
                field("evolve_spirit_id", "int", "进化目标宠物 ID。"),
                field("status", "int", "兑换状态。"),
            ]
        }),
        "AquariusSecondStatusInfo" => ("水瓶宫二阶状态。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("light_num", "int", "当前光能数量。"),
                field("tail_num", "int", "当前尾部资源数量。"),
                field("boss_left_hp", "int", "首领剩余生命值。"),
                field("boss_full_hp", "int", "首领最大生命值。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
                field("add_hit_level", "int", "新增命中等级。"),
                field("today_sum_hit", "int", "今日累计命中次数。"),
                field("exchange_count0", "int", "兑换项 0 次数。"),
                field("exchange_count1", "int", "兑换项 1 次数。"),
            ]
        }),
        "AriesThirdStatusInfo" => ("白羊宫三阶状态。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("light_num", "int", "当前光能数量。"),
                field("tail_num", "int", "当前尾部资源数量。"),
                field("exchange_count0", "int", "兑换项 0 次数。"),
                field("exchange_count1", "int", "兑换项 1 次数。"),
                field("boss_left_hp", "int", "首领剩余生命值。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
            ]
        }),
        "LibraThirdStatusInfo" => ("天秤宫三阶状态。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("light_num", "int", "当前光能数量。"),
                field("tail_num", "int", "当前尾部资源数量。"),
                field("exchange_count0", "int", "兑换项 0 次数。"),
                field("exchange_count1", "int", "兑换项 1 次数。"),
                field("boss_left_hp", "int", "首领剩余生命值。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
            ]
        }),
        "CapricornPalaceNotesInfo" => ("摩羯宫宫殿笔记信息。", {
            vec![
                field("items", "CapricornPalaceNoteItem[]", "笔记物品列表。"),
                field("can_summon", "bool", "是否可以召唤。"),
            ]
        }),
        "CapricornPalaceNoteItem" => ("摩羯宫宫殿笔记物品。", {
            vec![
                field("item_index", "int", "物品索引。"),
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "当前数量。"),
                field("need", "int", "所需数量。"),
            ]
        }),
        "CapricornInviteListInfo" => ("摩羯宫邀请列表。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("players", "CapricornTeamPlayer[]", "队伍成员列表。"),
                field("ticks", "int", "队伍状态时间标记。"),
            ]
        }),
        "CapricornTeamPlayer" => ("摩羯宫队伍成员。", {
            vec![
                field("uin", "int", "成员 UIN。"),
                field("nick", "string", "成员昵称。"),
            ]
        }),
        "LeoFirstStatusInfo" => ("狮子宫一阶状态。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("light_num", "int", "当前光能数量。"),
                field("tail_num", "int", "当前尾部资源数量。"),
                field("exchange_count0", "int", "兑换项 0 次数。"),
                field("exchange_count1", "int", "兑换项 1 次数。"),
                field("boss_left_hp", "int", "首领剩余生命值。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
            ]
        }),
        "VirgoBellFoxStatusInfo" => ("处女宫铃狐状态。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("light_num", "int", "当前光能数量。"),
                field("tail_num", "int", "当前尾部资源数量。"),
                field("boss_left_hp", "int", "首领剩余生命值。"),
                field("boss_full_hp", "int", "首领最大生命值。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
                field("add_hit_level", "int", "新增命中等级。"),
                field("today_sum_hit", "int", "今日累计命中次数。"),
                field("exchange_count0", "int", "兑换项 0 次数。"),
                field("exchange_count1", "int", "兑换项 1 次数。"),
            ]
        }),
        "SummonInfo" => ("召唤活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("vip", "int", "VIP 状态。"),
                field("magic", "int", "当前召唤魔法值。"),
                field("count", "int", "当前召唤次数。"),
                field("show", "int", "展示状态。"),
                field("pools", "SummonPoolState[]", "召唤池状态列表。"),
                field("config_pools", "SummonPoolConfig[]", "召唤池配置列表。"),
                field("exchange_groups", "SummonExchangeGroup[]", "兑换组列表。"),
                field("rewards", "SummonRewardItem[]", "奖励列表。"),
                field("records", "SummonRecord[]", "召唤记录列表。"),
            ]
        }),
        "SummonPoolState" => ("召唤池当前状态。", {
            vec![
                field("version", "int", "召唤池版本。"),
                field("token_item_id", "int", "抽取消耗物品 ID。"),
                field("token_count", "int", "抽取消耗数量。"),
                field("today_draw_count", "int", "今日抽取次数。"),
                field("wish_index", "int", "当前祈愿项索引。"),
                field("succeeded", "bool", "是否已达成召唤目标。"),
                field("end_time", "int", "召唤池结束时间。"),
            ]
        }),
        "SummonPoolConfig" => ("召唤池配置。", {
            vec![
                field("version", "int", "召唤池版本。"),
                field("title", "string", "召唤池标题。"),
                field("vip_limit", "int", "VIP 限制。"),
                field("end_time", "int", "召唤池结束时间。"),
                field("daily_max", "int", "每日抽取上限。"),
                field("token_item_id", "int", "抽取消耗物品 ID。"),
                field("recommend", "string", "推荐说明。"),
                field("info", "string", "召唤池说明。"),
                field("reward_text", "string", "奖励说明。"),
                field("rewards", "SummonRewardItem[]", "奖励列表。"),
                field("wish_candidates", "SummonRewardItem[]", "祈愿候选列表。"),
            ]
        }),
        "SummonExchangeGroup" => ("召唤兑换组。", {
            vec![
                field("kind", "string", "兑换组类型。"),
                field("items", "SummonExchangeItem[]", "兑换项列表。"),
            ]
        }),
        "SummonExchangeItem" => ("召唤兑换项。", {
            vec![
                field("index", "int", "兑换项索引。"),
                field("reward", "SummonRewardItem", "兑换获得的奖励。"),
                field("cost", "SummonRewardItem", "兑换消耗。"),
                field("need", "int", "兑换所需数量。"),
                field("max", "int", "兑换上限。"),
                field("day_max", "int", "每日兑换上限。"),
                field("times", "int", "累计兑换次数。"),
                field("day_times", "int", "今日兑换次数。"),
                field("add", "int", "本次增加数量。"),
            ]
        }),
        "SummonRewardItem" => ("召唤奖励物品。", {
            vec![
                field("id", "int", "奖励 ID。"),
                field("item_type", "int", "奖励类型。"),
                field("count", "int", "奖励数量。"),
            ]
        }),
        "SummonRecord" => ("召唤历史记录。", {
            vec![
                field("pool_version", "int", "召唤池版本。"),
                field("title", "string", "召唤池标题。"),
                field("id", "int", "奖励 ID。"),
                field("item_type", "int", "奖励类型。"),
                field("count", "int", "奖励数量。"),
                field("year", "int", "年份。"),
                field("month", "int", "月份。"),
                field("day", "int", "日期。"),
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
        "FourSeasonsInfo" => ("四时邀约活动返回信息。", {
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
