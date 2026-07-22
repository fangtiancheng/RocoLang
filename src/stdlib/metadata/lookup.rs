use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "lookup",
            "lookup_item_info",
            return_type: "StaticItemInfo",
            "查询静态道具资料。",
            params: ["item_id" => "道具 ID。"],
            returns: "道具静态资料。",
            examples: ["let item = lookup::lookup_item_info(300000);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_items_info",
            return_type: "StaticItemInfo[]",
            "批量查询静态道具资料。",
            params: ["item_ids" => "道具 ID 数组。"],
            returns: "道具静态资料数组。",
            examples: ["let items = lookup::lookup_items_info([300000, 300001]);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_strive_item_info",
            return_type: "StaticStriveItemInfo",
            "查询努力值道具静态资料。",
            params: ["item_id" => "道具 ID。"],
            returns: "努力值道具资料。",
            examples: ["let item = lookup::lookup_strive_item_info(300000);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_strive_item_infos",
            return_type: "StaticStriveItemInfo[]",
            "列出全部努力值道具静态资料。",
            params: [],
            returns: "努力值道具资料数组。",
            examples: ["let items = lookup::list_strive_item_infos();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_features_name",
            return_type: "string[]",
            "列出全部特性名称。",
            params: [],
            returns: "特性名称数组。",
            examples: ["let names = lookup::list_features_name();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_guardian_pet_property_info",
            return_type: "StaticGuardianPetPropertyInfo",
            "查询守护兽指定等级属性资料。",
            params: ["level" => "守护兽等级。"],
            returns: "守护兽属性资料。",
            examples: ["let info = lookup::lookup_guardian_pet_property_info(100);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_title_info",
            return_type: "StaticTitleInfo",
            "查询称号静态资料。",
            params: ["title_id" => "称号 ID。"],
            returns: "称号静态资料。",
            examples: ["let title = lookup::lookup_title_info(1);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_magic_info",
            return_type: "StaticMagicInfo",
            "查询魔法静态资料。",
            params: ["magic_id" => "魔法 ID。"],
            returns: "魔法静态资料。",
            examples: ["let magic = lookup::lookup_magic_info(1);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_plugin_info",
            return_type: "StaticPluginInfo",
            "按插件名查询静态插件资料。",
            params: ["plugin_name" => "插件名称。"],
            returns: "插件静态资料。",
            examples: ["let plugin = lookup::lookup_plugin_info(\"foo\");"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_plugin_infos",
            return_type: "StaticPluginInfo[]",
            "列出全部静态插件资料。",
            params: [],
            returns: "插件静态资料数组。",
            examples: ["let plugins = lookup::list_plugin_infos();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "get_ladder_match_config",
            return_type: "LadderMatchConfig",
            "查询天梯匹配静态配置。",
            params: [],
            returns: "天梯匹配配置。",
            examples: ["let config = lookup::get_ladder_match_config();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_talent_info",
            return_type: "StaticTalentInfo",
            "查询天赋静态资料。",
            params: ["talent_type" => "天赋类型。"],
            returns: "天赋静态资料。",
            examples: ["let talent = lookup::lookup_talent_info(1);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_talent_infos",
            return_type: "StaticTalentInfo[]",
            "列出全部天赋静态资料。",
            params: [],
            returns: "天赋静态资料数组。",
            examples: ["let talents = lookup::list_talent_infos();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_skill_info",
            return_type: "StaticSkillInfo",
            "查询静态技能资料。",
            params: ["skill_id" => "技能 ID。"],
            returns: "技能静态资料。",
            examples: ["let skill = lookup::lookup_skill_info(497);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_skills_info",
            return_type: "StaticSkillInfo[]",
            "批量查询静态技能资料。",
            params: ["skill_ids" => "技能 ID 数组。"],
            returns: "技能静态资料数组。",
            examples: ["let skills = lookup::lookup_skills_info([497, 498]);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_spirit_info",
            return_type: "StaticSpiritInfo",
            "查询静态宠物资料；不存在时抛出结构化错误。",
            params: ["spirit_id" => "宠物 ID。"],
            returns: "宠物静态资料。",
            examples: ["let info = lookup::lookup_spirit_info(1);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "try_lookup_spirit_info",
            return_type: "StaticSpiritInfoLookupResult",
            "尝试查询静态宠物资料；不存在时返回 unknown 结果。",
            params: ["spirit_id" => "宠物 ID。"],
            returns: "宠物资料查询结果。",
            examples: ["let result = lookup::try_lookup_spirit_info(1);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "lookup_spirits_info",
            return_type: "StaticSpiritInfo[]",
            "批量查询静态宠物资料。",
            params: ["spirit_ids" => "宠物 ID 数组。"],
            returns: "宠物静态资料数组。",
            examples: ["let infos = lookup::lookup_spirits_info([1, 4, 7]);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_spirit_book_summaries",
            return_type: "SpiritBookSummary[]",
            "列出图鉴册摘要。",
            params: [],
            returns: "图鉴册摘要列表。",
            examples: ["let summaries = lookup::list_spirit_book_summaries();"]
        ),
        super::stdlib_doc!(
            "lookup",
            "get_spirit_book",
            return_type: "SpiritBookInfo",
            "查询指定图鉴册详情。",
            params: ["book_id" => "图鉴册 ID。宠物大全为 10。"],
            returns: "图鉴册详情。",
            examples: ["let book = lookup::get_spirit_book(10);"]
        ),
        super::stdlib_doc!(
            "lookup",
            "list_spirit_book_entries",
            return_type: "SpiritBookEntry[]",
            "列出指定图鉴册中的全部宠物条目。",
            params: ["book_id" => "图鉴册 ID。宠物大全为 10。"],
            returns: "图鉴条目数组。",
            examples: ["let entries = lookup::list_spirit_book_entries(10);"]
        ),
    ]
}
