use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
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
        "AquariusRewardItem" => (
            "水瓶宫奖励项。",
            vec![
                field("item_index", "int", "奖励项索引。"),
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "数量。"),
                field("item_type", "RocoOptionalI64", "结构化可选物品类型。"),
            ],
        ),
        "AquariusBagCandidate" => (
            "水瓶宫背包候选宠物。",
            vec![
                field("candidate_index", "int", "候选项索引。"),
                field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
                field("bag_index", "RocoOptionalI64", "结构化可选背包位置。"),
                field("catch_time", "RocoOptionalI64", "结构化可选捕获时间。"),
                field("level", "RocoOptionalI64", "结构化可选等级。"),
                field("need_money", "RocoOptionalI64", "结构化可选所需洛克贝。"),
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
        "StaticSpiritEvolutionEdge" => (
            "宠物进化链边。",
            vec![
                field("target_id", "int", "进化目标宠物 ID。"),
                field("kind", "int", "进化边类型。"),
            ],
        ),
        _ => return None,
    })
}
