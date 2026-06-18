use super::{StdlibFieldDoc, StdlibReturnDoc};

pub fn return_doc_for(type_name: &str) -> Option<StdlibReturnDoc> {
    let normalized = normalize_type_name(type_name);
    let (description, fields) = match normalized.as_str() {
        "ActionResult" => (
            "操作型 try_* 接口的标准返回结构。",
            vec![
                field("ok", "bool", "操作是否成功。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示业务失败。"),
                field(
                    "message",
                    "string",
                    "失败原因或服务器返回信息，成功时通常为空。",
                ),
            ],
        ),
        "CombatActionResult" => (
            "战斗动作提交和等待结果。",
            vec![
                field("ok", "bool", "提交和等待流程是否成功完成。"),
                field(
                    "code",
                    "int",
                    "结果码；0 表示成功，非 0 表示动作不可用或执行失败。",
                ),
                field("message", "string", "失败原因或服务器返回信息。"),
                field("ack_received", "bool", "是否收到服务器对本次动作的 ACK。"),
                field("combat_finished", "bool", "动作后战斗是否已经结束。"),
                field(
                    "next_action_ready",
                    "bool",
                    "动作后是否已经进入下一次可行动状态。",
                ),
            ],
        ),
        "CombatActions" => (
            "当前战斗回合可执行动作集合。",
            vec![
                field(
                    "can_submit_action",
                    "bool",
                    "当前是否可以提交任意战斗动作。",
                ),
                field("can_use_skill", "bool", "当前是否可以使用技能。"),
                field("can_capture", "bool", "当前是否可以捕捉。"),
                field("can_use_item", "bool", "当前是否可以使用道具。"),
                field("can_change_spirit", "bool", "当前是否可以换宠。"),
                field("can_escape", "bool", "当前是否可以逃跑。"),
                field("can_use_any_skill", "bool", "当前是否至少有一个技能可用。"),
                field(
                    "can_change_to_any_spirit",
                    "bool",
                    "当前是否至少有一个可切换精灵。",
                ),
            ],
        ),
        "CombatActionSnapshot" => (
            "战斗状态和可行动信息的组合快照。",
            vec![
                field("is_finished", "bool", "战斗是否已经结束。"),
                field(
                    "state",
                    "CombatState",
                    "当前战斗状态；包含回合、天气、我方和敌方状态。",
                ),
                field("actions", "CombatActions", "当前可执行动作集合。"),
            ],
        ),
        "CombatState" => (
            "底层战斗状态快照。",
            vec![
                field("round", "int", "当前回合数。"),
                field("weather", "int", "当前天气或环境 ID。"),
                field("weather_round", "int", "天气或环境剩余回合数。"),
                field("my_side", "CombatSideState", "我方战斗状态。"),
                field("rival_side", "CombatSideState", "敌方战斗状态。"),
            ],
        ),
        "CombatSideState" => (
            "战斗一方的出战状态。",
            vec![
                field("active_position", "int", "当前出战精灵位置。"),
                field("spirits", "CombatSpiritState[]", "该方精灵状态列表。"),
            ],
        ),
        "CombatSpiritState" => (
            "战斗中的精灵状态。",
            vec![
                field("position", "int", "背包或战斗位置。"),
                field("spirit_id", "int", "精灵 ID。"),
                field("level", "int", "等级。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "SpiritInfo" => (
            "背包、仓库或战斗中的精灵详细信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置；非背包来源可能为 0。"),
                field("catch_time", "int", "捕获时间，用于区分同种精灵个体。"),
                field("name", "string", "精灵名称。"),
                field("level", "int", "精灵等级。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "SpiritSkillInfo" => (
            "精灵技能信息。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "当前 PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
            ],
        ),
        "StorageSpiritInfo" => (
            "仓库精灵摘要。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("catch_time", "int", "捕获时间，用于区分同种精灵个体。"),
                field("storage_time", "int", "入库时间。"),
                field("level", "int", "精灵等级。"),
                field("sex", "int", "性别标识。"),
                field("skin_flag", "int", "皮肤标识。"),
                field("talent_type", "int", "天赋类型。"),
                field("talent_level", "int", "天赋等级。"),
            ],
        ),
        "BagItemInfo" => (
            "背包物品数量。",
            vec![
                field("item_id", "int", "道具 ID。"),
                field("count", "int", "数量。"),
            ],
        ),
        "SkillPoolInfo" => (
            "精灵技能池信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置。"),
                field("skills", "SkillPoolSkillInfo[]", "技能池技能列表。"),
            ],
        ),
        "SkillPoolSkillInfo" => (
            "技能池中的技能信息。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
                field("position", "int", "技能所在槽位。"),
            ],
        ),
        "SkillSwitchResult" => (
            "技能切换结果。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置。"),
                field("skill_slot", "int", "技能槽位。"),
                field("skill_id", "int", "技能 ID。"),
            ],
        ),
        "SkillStoneResult" => (
            "技能石预览、应用或替换结果。",
            vec![
                field("ok", "bool", "操作是否成功。"),
                field("result_code", "int", "服务器结果码。"),
                field("message", "string", "失败原因或提示信息。"),
                field("item_id", "int", "技能石道具 ID。"),
                field("position", "int", "背包位置。"),
                field("needs_replace", "bool", "是否需要选择旧技能进行替换。"),
                field(
                    "old_skills",
                    "SkillStoneSkillInfo[]",
                    "可被替换的旧技能列表。",
                ),
                field(
                    "new_skills",
                    "SkillStoneSkillInfo[]",
                    "技能石提供的新技能列表。",
                ),
            ],
        ),
        "SkillStoneSkillInfo" => (
            "技能石候选技能。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
            ],
        ),
        "BattleResult" => (
            "战斗结算结果。",
            vec![
                field("winner", "int", "胜负方标识。"),
                field("total_rounds", "int", "战斗总回合数。"),
                field("finish_code", "int", "战斗结束原因码。"),
                field("trainer_exp", "int", "获得的训练师经验。"),
                field(
                    "next_level_trainer_exp",
                    "int",
                    "距离下一训练师等级所需经验。",
                ),
                field("honour_point", "int", "获得的荣誉点。"),
                field("exp_add_bits", "int", "经验加成位标记。"),
                field("obtained_items", "BagItemInfo[]", "获得的物品列表。"),
                field(
                    "spirit_results",
                    "BattleSpiritResult[]",
                    "参战精灵经验/等级变化列表。",
                ),
                field(
                    "captured_spirits",
                    "BattleCapturedSpirit[]",
                    "本场捕获精灵列表。",
                ),
            ],
        ),
        "BattleResultQueryResult" => (
            "不会因战斗结果暂不可用而抛错的查询结果。",
            vec![
                field("ok", "bool", "是否成功取得战斗结果。"),
                field(
                    "code",
                    "int",
                    "结果码；0 表示成功，非 0 表示暂不可用或失败。",
                ),
                field("message", "string", "失败原因，成功时通常为空。"),
                field(
                    "result",
                    "BattleResult",
                    "战斗结算结果；ok 为 false 时为默认空结果。",
                ),
            ],
        ),
        "MiniGameSubmitResult" => (
            "小游戏分数提交结果。",
            vec![
                field("ok", "bool", "提交是否成功。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示失败。"),
                field("message", "string", "失败原因或服务器返回信息。"),
                field("game_id", "int", "小游戏 ID。"),
                field("score", "int", "提交的分数。"),
                field("game_type", "int", "小游戏类型。"),
                field("items", "MiniGameRewardItem[]", "获得的奖励物品。"),
                field(
                    "extra_fields",
                    "MiniGameExtraField[]",
                    "协议返回的附加字段。",
                ),
            ],
        ),
        "MiniGameSubmitTryResult" => (
            "不会因网络或业务失败抛错的小游戏提交结果。",
            vec![
                field("ok", "bool", "是否成功提交。"),
                field(
                    "code",
                    "int",
                    "结果码；0 表示成功，1001 表示网络错误，其它非 0 表示业务失败。",
                ),
                field("message", "string", "失败原因，成功时通常为空。"),
                field(
                    "result",
                    "MiniGameSubmitResult",
                    "小游戏提交结果；失败时为默认失败结果。",
                ),
            ],
        ),
        "UserInfo" => (
            "当前登录用户信息。",
            vec![
                field("uin", "int", "用户 UIN。"),
                field("id", "int", "角色 ID。"),
                field("nick_name", "string", "角色昵称。"),
                field("level", "int", "角色等级。"),
                field("is_vip", "bool", "是否为 VIP。"),
                field("vip_level", "int", "VIP 等级。"),
                field("vip_expiring_days", "int", "VIP 剩余天数。"),
                field("vip_lulu", "int", "Lulu VIP 状态。"),
                field("trainer_level", "int", "训练师等级。"),
                field("trainer_exp", "int", "训练师经验。"),
            ],
        ),
        "SceneSpiritInfo" => (
            "场景中的宠物刷新信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("count", "int", "刷新数量。"),
                field("area_index", "int", "区域索引。"),
                field("is_rare", "bool", "是否稀有宠物。"),
                field("is_boss", "bool", "是否 Boss。"),
                field("is_npc_boss", "bool", "是否 NPC Boss。"),
            ],
        ),
        "SceneRoleInfo" => (
            "场景中的角色缓存信息。",
            vec![
                field("uin", "int", "角色 UIN。"),
                field("id", "int", "角色 ID。"),
                field("nick_name", "string", "昵称。"),
                field("level", "int", "等级。"),
                field("loc_x", "int", "场景 X 坐标。"),
                field("loc_y", "int", "场景 Y 坐标。"),
                field("pk_state", "int", "PK 状态。"),
                field("is_in_combat", "bool", "是否处于战斗中。"),
                field("is_vip", "bool", "是否 VIP。"),
                field("vip_level", "int", "VIP 等级。"),
                field("trainer_level", "int", "训练师等级。"),
                field("trainer_exp", "int", "训练师经验。"),
            ],
        ),
        "BattleInfo" => (
            "Combat session information.",
            vec![
                field("battle_id", "string", "Battle ID."),
                field("my_uin", "int", "My UIN."),
                field("rival_uin", "int", "Rival UIN."),
                field("started", "bool", "Whether the battle has started."),
            ],
        ),
        "RoundResult" => (
            "Round wait result.",
            vec![
                field("round", "int", "Current round number."),
                field("my_hp", "int", "Current HP of my active spirit."),
                field("rival_hp", "int", "Current HP of the rival active spirit."),
                field("finished", "bool", "Whether the battle has finished."),
            ],
        ),
        "NewsTimesReportsResult" => (
            "News report query result.",
            vec![
                field("gift_gotten", "int", "Gift claim status."),
                field("reports", "NewsTimesReport[]", "News report entries."),
                field(
                    "player_status_today",
                    "int[]",
                    "Per-player status for today.",
                ),
                field(
                    "player_status_forever",
                    "int[]",
                    "Persistent per-player status.",
                ),
            ],
        ),
        "NewsActiveItem" => (
            "Active news or activity item.",
            vec![
                field("id", "int", "Activity ID."),
                field("scene_id", "int", "Related scene ID."),
                field("npc_x", "int", "NPC X coordinate."),
                field("npc_y", "int", "NPC Y coordinate."),
                field("time", "string", "Activity time text."),
                field("content", "string", "Activity content text."),
                field(
                    "auto_start",
                    "bool",
                    "Whether the item starts automatically.",
                ),
                field("script_url", "string", "Script URL."),
                field("app_url", "string", "Application entry URL."),
            ],
        ),
        "TalentRefreshResult" => (
            "Talent refresh result.",
            vec![
                field("position", "int", "Bag position."),
                field("pa_old", "int", "Physical attack talent before refresh."),
                field("pd_old", "int", "Physical defense talent before refresh."),
                field("ma_old", "int", "Magic attack talent before refresh."),
                field("md_old", "int", "Magic defense talent before refresh."),
                field("sp_old", "int", "Speed talent before refresh."),
                field("hp_old", "int", "HP talent before refresh."),
                field("pa_new", "int", "Physical attack talent after refresh."),
                field("pd_new", "int", "Physical defense talent after refresh."),
                field("ma_new", "int", "Magic attack talent after refresh."),
                field("md_new", "int", "Magic defense talent after refresh."),
                field("sp_new", "int", "Speed talent after refresh."),
                field("hp_new", "int", "HP talent after refresh."),
                field(
                    "pa_level_old",
                    "int",
                    "Physical attack talent grade before refresh.",
                ),
                field(
                    "pd_level_old",
                    "int",
                    "Physical defense talent grade before refresh.",
                ),
                field(
                    "ma_level_old",
                    "int",
                    "Magic attack talent grade before refresh.",
                ),
                field(
                    "md_level_old",
                    "int",
                    "Magic defense talent grade before refresh.",
                ),
                field("sp_level_old", "int", "Speed talent grade before refresh."),
                field("hp_level_old", "int", "HP talent grade before refresh."),
                field(
                    "pa_level_new",
                    "int",
                    "Physical attack talent grade after refresh.",
                ),
                field(
                    "pd_level_new",
                    "int",
                    "Physical defense talent grade after refresh.",
                ),
                field(
                    "ma_level_new",
                    "int",
                    "Magic attack talent grade after refresh.",
                ),
                field(
                    "md_level_new",
                    "int",
                    "Magic defense talent grade after refresh.",
                ),
                field("sp_level_new", "int", "Speed talent grade after refresh."),
                field("hp_level_new", "int", "HP talent grade after refresh."),
            ],
        ),
        "BloodGiftInfo" => (
            "Blood gift information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field("position", "int", "Bag position."),
                field(
                    "equipped_index",
                    "int",
                    "Currently equipped blood gift index.",
                ),
                field(
                    "options",
                    "BloodGiftOption[]",
                    "Available blood gift options.",
                ),
            ],
        ),
        "AmendNatureInfo" => (
            "Nature amendment information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field(
                    "eligible_spirit_ids",
                    "int[]",
                    "Spirit IDs eligible for nature amendment.",
                ),
                field(
                    "candidates",
                    "AmendNatureCandidate[]",
                    "Candidate spirits for nature amendment.",
                ),
                field("new_personality", "int", "New personality enum value."),
                field("new_personality_name", "string", "New personality name."),
            ],
        ),
        "SpiritBagInfo" => (
            "Spirit bag information.",
            vec![field(
                "spirits",
                "SpiritInfo[]",
                "Spirits currently in the bag.",
            )],
        ),
        "SpiritEquipmentBagInfo" => (
            "Spirit equipment bag information.",
            vec![
                field("equipment_count", "int", "Number of equipment items."),
                field("all_num", "int", "Capacity or total count."),
                field("need", "int", "Required value for the operation."),
                field("equipments", "SpiritEquipmentInfo[]", "Equipment list."),
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
        "ManorSeedInfo" => (
            "Manor seed information.",
            vec![
                field("item_id", "int", "Seed item ID."),
                field("item_count", "int", "Seed count."),
            ],
        ),
        "StaticItemInfo" => (
            "Static item information.",
            vec![
                field("id", "int", "Item ID."),
                field("name", "string", "Item name."),
                field("description", "string", "Item description."),
                field("unique", "bool", "Whether the item is unique."),
                field("item_type", "int", "Item type."),
                field("subtype", "int", "Item subtype."),
                field("price", "int", "Price."),
                field("expire_time", "string", "Expiration time."),
            ],
        ),
        "StaticStriveItemInfo" => (
            "Static effort item information.",
            vec![
                field("id", "int", "Item ID."),
                field("name", "string", "Item name."),
                field("item_type", "int", "Item type."),
                field("ghp", "int", "HP effort gain."),
                field("gpa", "int", "Physical attack effort gain."),
                field("gpd", "int", "Physical defense effort gain."),
                field("gma", "int", "Magic attack effort gain."),
                field("gmd", "int", "Magic defense effort gain."),
                field("gsp", "int", "Speed effort gain."),
                field("src", "string", "Resource path."),
            ],
        ),
        "StaticGuardianPetPropertyInfo" => (
            "Static guardian pet property information.",
            vec![
                field("level", "int", "Level."),
                field("phase", "int", "Phase."),
                field("energy", "int", "Energy."),
                field("attack", "int", "Attack."),
                field("defend", "int", "Defense."),
                field("magic_attack", "int", "Magic attack."),
                field("magic_defend", "int", "Magic defense."),
                field(
                    "need_level_to_next_phase",
                    "int",
                    "Level required for next phase.",
                ),
            ],
        ),
        "StaticTitleInfo" => (
            "Static title information.",
            vec![
                field("id", "int", "Title ID."),
                field("title_name", "string", "Title name."),
            ],
        ),
        "StaticMagicInfo" => (
            "Static magic information.",
            vec![
                field("id", "int", "Magic ID."),
                field("name", "string", "Magic name."),
                field("item_id", "int", "Related item ID."),
                field("target", "int", "Target type."),
                field("magic_type", "int", "Magic type."),
                field("duration", "int", "Duration."),
                field("action_type", "int", "Action type."),
                field("app", "string", "Application identifier."),
                field("description", "string", "Description."),
            ],
        ),
        "StaticPluginInfo" => (
            "Static plugin information.",
            vec![
                field("name", "string", "Plugin name."),
                field("label", "string", "Display label."),
                field("domain", "string", "Domain."),
                field("version", "string", "Version."),
                field("command_type", "string", "Command type."),
                field("plugin_class", "string", "Plugin class name."),
                field("plugin_src", "string", "Plugin resource path."),
                field("plugin_url", "string", "Plugin URL."),
            ],
        ),
        "LadderMatchConfig" => (
            "Ladder match configuration.",
            vec![
                field(
                    "error",
                    "string",
                    "Configuration parse error; empty when valid.",
                ),
                field("match_rewards", "string[]", "Match reward configuration."),
                field("win_rewards", "string[]", "Win reward configuration."),
                field("season_rewards", "string[]", "Season reward configuration."),
                field(
                    "task0_descriptions",
                    "LadderQuestConfigEntry[]",
                    "Task 0 description configuration.",
                ),
                field(
                    "task1_descriptions",
                    "LadderQuestConfigEntry[]",
                    "Task 1 description configuration.",
                ),
                field(
                    "spirit_costs",
                    "LadderSpiritCostEntry[]",
                    "Spirit cost configuration.",
                ),
                field("limit_spirits", "int[]", "Limited spirit ID list."),
            ],
        ),
        "StaticTalentInfo" => (
            "Static talent information.",
            vec![
                field("id", "int", "Talent type ID."),
                field("name", "string", "Talent name."),
                field("description", "string", "Talent description."),
            ],
        ),
        "StaticSkillInfo" => (
            "Static skill information.",
            vec![
                field("id", "int", "Skill ID."),
                field("name", "string", "Skill name."),
                field("description", "string", "Skill description."),
                field("description2", "string", "Extended skill description."),
                field("power", "string", "Power."),
                field("pp_max", "int", "Maximum PP."),
                field("property", "int", "Property ID."),
                field("src", "string", "Resource path."),
                field("attack_type", "int", "Attack type."),
                field("speed", "int", "Priority speed."),
                field("damage_type", "int", "Damage type."),
                field("catch_rate", "int", "Capture-related rate."),
                field("super_form_id", "int", "Super form ID."),
                field("super_form_src", "string", "Super form resource path."),
            ],
        ),
        "SpiritBookStates" => (
            "图鉴拥有状态集合。",
            vec![
                field("uin", "int", "状态所属角色 UIN。"),
                field("count", "int", "状态条目数量。"),
                field("states", "Map", "宠物 ID 到图鉴状态的映射。"),
            ],
        ),
        "SpiritBookSpiritState" => (
            "单个宠物的图鉴拥有状态。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("state", "int", "图鉴状态枚举值。"),
                field("owned", "bool", "是否已拥有。"),
            ],
        ),
        "SpiritBookSummary" => (
            "图鉴册摘要。",
            vec![
                field("id", "int", "图鉴册 ID。"),
                field("name", "string", "图鉴册名称。"),
                field("count", "int", "条目总数。"),
            ],
        ),
        "SpiritBookEntry" => (
            "图鉴宠物条目。",
            vec![
                field("id", "int", "宠物 ID。"),
                field("starred", "bool", "是否星标。"),
                field("unknown", "bool", "是否未知条目。"),
                field("newed", "bool", "是否新条目。"),
            ],
        ),
        "SpiritBookInfo" => (
            "图鉴册详情。",
            vec![
                field("id", "int", "图鉴册 ID。"),
                field("name", "string", "图鉴册名称。"),
                field("groups", "SpiritBookGroup[]", "图鉴分组列表。"),
            ],
        ),
        "StaticSpiritInfoLookupResult" => (
            "静态宠物资料查询结果。",
            vec![
                field("ok", "bool", "是否找到宠物资料。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示不存在或失败。"),
                field("message", "string", "失败原因，成功时通常为空。"),
                field(
                    "result",
                    "StaticSpiritInfo",
                    "宠物静态资料；失败时为 unknown 占位。",
                ),
            ],
        ),
        "StaticSpiritInfo" => (
            "宠物静态资料。",
            vec![
                field("id", "int", "宠物 ID。"),
                field("name", "string", "宠物名称。"),
                field("first_id", "int", "普通进化链首个形态 ID。"),
                field(
                    "evolution_edges",
                    "StaticSpiritEvolutionEdge[]",
                    "进化边列表。",
                ),
            ],
        ),
        _ => return None,
    };

    Some(StdlibReturnDoc {
        type_name: type_name.to_string(),
        description: description.to_string(),
        fields,
    })
}

pub fn infer_return_type(signature: &str) -> Option<String> {
    let return_type = signature.split("->").nth(1)?.trim();
    if return_type.is_empty() || return_type == "()" {
        return None;
    }
    Some(return_type.to_string())
}

fn normalize_type_name(type_name: &str) -> String {
    type_name
        .trim()
        .trim_end_matches("[]")
        .trim_start_matches("Vec<")
        .trim_end_matches('>')
        .to_string()
}

fn field(name: &str, type_name: &str, description: &str) -> StdlibFieldDoc {
    StdlibFieldDoc {
        name: name.to_string(),
        type_name: type_name.to_string(),
        description: description.to_string(),
    }
}
