use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "spirit",
            "fetch_spirit",
            return_type: "bool",
            "按宠物 ID 和捕获时间取回精灵。",
            params: ["spirit_id" => "宠物 ID。", "catch_time" => "捕获时间。"],
            returns: "取回成功返回 true。",
            examples: ["spirit::fetch_spirit(1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "list_storage_spirits",
            return_type: "StorageSpiritInfo[]",
            "查询仓库精灵列表。",
            params: [],
            returns: "仓库精灵列表。",
            examples: ["let spirits = spirit::list_storage_spirits();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "list_abandoned_storage_spirits",
            return_type: "StorageSpiritInfo[]",
            "查询放生仓库精灵列表。",
            params: [],
            returns: "放生仓库精灵列表。",
            examples: ["let spirits = spirit::list_abandoned_storage_spirits();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_storage_spirit_detail",
            return_type: "StorageSpiritDetailInfo",
            "查询仓库中指定精灵的详细信息。",
            params: ["spirit_id" => "宠物 ID。", "catch_time" => "捕获时间。"],
            returns: "仓库精灵详细信息，包含真实能力值。",
            examples: ["let detail = spirit::get_storage_spirit_detail(1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "recover_all_spirits",
            return_type: "bool",
            "恢复背包全部精灵体力。",
            params: [],
            returns: "恢复成功返回 true。",
            examples: ["spirit::recover_all_spirits();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_auto_recover_enabled",
            return_type: "bool",
            "查询 PVE 战斗后自动恢复开关状态。",
            params: [],
            returns: "已开启返回 true，未开启返回 false。",
            examples: ["let enabled = spirit::get_auto_recover_enabled();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "toggle_auto_recover",
            return_type: "bool",
            "切换 PVE 战斗后自动恢复开关，并返回切换后的状态。",
            params: [],
            returns: "切换后的开关状态。",
            examples: ["let enabled = spirit::toggle_auto_recover();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "try_recover_all_spirits",
            return_type: "ActionResult",
            "尝试恢复背包全部精灵体力，失败时返回结构化结果。",
            params: [],
            returns: "结构化操作结果。",
            examples: ["let result = spirit::try_recover_all_spirits();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "use_spirit_item",
            return_type: "bool",
            "对指定背包精灵使用精灵道具。",
            params: ["spirit_id" => "宠物 ID。", "position" => "背包位置，从 1 开始。", "item_id" => "道具 ID。", "count" => "使用数量。"],
            returns: "使用成功返回 true。",
            examples: ["spirit::use_spirit_item(1, 1, 300000, 1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "restore_spirit",
            return_type: "bool",
            "将指定精灵恢复到背包位置。",
            params: ["spirit_id" => "宠物 ID。", "position" => "背包位置，从 1 开始。"],
            returns: "恢复成功返回 true。",
            examples: ["spirit::restore_spirit(1, 1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "try_restore_spirit",
            return_type: "ActionResult",
            "尝试恢复指定精灵，失败时返回结构化结果。",
            params: ["spirit_id" => "宠物 ID。", "position" => "背包位置，从 1 开始。"],
            returns: "结构化操作结果。",
            examples: ["let result = spirit::try_restore_spirit(1, 1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "use_talent_refresh_item",
            return_type: "TalentRefreshResult",
            "对指定精灵使用天赋刷新道具。",
            params: ["spirit_id" => "宠物 ID。", "position" => "背包位置，从 1 开始。", "item_id" => "道具 ID。"],
            returns: "天赋刷新结果。",
            examples: ["let result = spirit::use_talent_refresh_item(1, 1, 300000);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_blood_gift_info",
            return_type: "BloodGiftInfo",
            "查询指定背包精灵的血脉天赋信息。",
            params: ["position" => "背包位置，从 1 开始。"],
            returns: "血脉天赋信息。",
            examples: ["let info = spirit::get_blood_gift_info(1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "awaken_blood_gift",
            return_type: "BloodGiftInfo",
            "觉醒指定背包精灵的血脉天赋。",
            params: ["position" => "背包位置，从 1 开始。", "blood_index" => "血脉槽位。"],
            returns: "更新后的血脉天赋信息。",
            examples: ["let info = spirit::awaken_blood_gift(1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "equip_blood_gift",
            return_type: "BloodGiftInfo",
            "装备指定背包精灵的血脉天赋。",
            params: ["position" => "背包位置，从 1 开始。", "blood_index" => "血脉槽位。"],
            returns: "更新后的血脉天赋信息。",
            examples: ["let info = spirit::equip_blood_gift(1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "amend_nature_query_eligible_spirit_ids",
            return_type: "int[]",
            "查询可进行性格修正的精灵 ID 列表。",
            params: [],
            returns: "宠物 ID 数组。",
            examples: ["let ids = spirit::amend_nature_query_eligible_spirit_ids();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "amend_nature_query_candidates",
            return_type: "AmendNatureInfo",
            "查询当前可选的性格修正候选信息。",
            params: [],
            returns: "性格修正候选信息。",
            examples: ["let info = spirit::amend_nature_query_candidates();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "random_amend_nature",
            return_type: "AmendNatureInfo",
            "随机修正指定精灵性格。",
            params: ["spirit_id" => "宠物 ID。", "catch_time" => "捕获时间。"],
            returns: "性格修正结果。",
            examples: ["let info = spirit::random_amend_nature(1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "choose_amend_nature",
            return_type: "AmendNatureInfo",
            "选择指定性格修正精灵。",
            params: ["spirit_id" => "宠物 ID。", "catch_time" => "捕获时间。", "personality" => "目标性格枚举值。"],
            returns: "性格修正结果。",
            examples: ["let info = spirit::choose_amend_nature(1, 0, 1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "allocate_exp",
            return_type: "bool",
            "给指定背包位置精灵分配经验。",
            params: ["position" => "背包位置，从 1 开始。", "exp" => "分配经验值。"],
            returns: "分配成功返回 true。",
            examples: ["spirit::allocate_exp(1, 1000);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "save_strive_add",
            return_type: "bool",
            "保存指定精灵的努力值加点。",
            params: ["position" => "背包位置，从 1 开始。", "pa" => "物攻努力值。", "pd" => "物防努力值。", "ma" => "魔攻努力值。", "md" => "魔防努力值。", "sp" => "速度努力值。", "hp" => "精力努力值。"],
            returns: "保存成功返回 true。",
            examples: ["spirit::save_strive_add(1, 0, 0, 252, 0, 252, 6);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "clear_lineup",
            return_type: "bool",
            "清空当前背包阵容。",
            params: [],
            returns: "清空成功返回 true。",
            examples: ["spirit::clear_lineup();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "store_spirit",
            return_type: "bool",
            "将背包指定位置精灵放入仓库。",
            params: ["position" => "背包位置，从 1 开始。"],
            returns: "入库成功返回 true。",
            examples: ["spirit::store_spirit(6);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "swap_spirits",
            return_type: "bool",
            "交换两个背包位置的精灵。",
            params: ["first_position" => "第一个背包位置。", "second_position" => "第二个背包位置。"],
            returns: "交换成功返回 true。",
            examples: ["spirit::swap_spirits(1, 2);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "try_swap_spirits",
            return_type: "ActionResult",
            "尝试交换两个背包位置的精灵，失败时返回结构化结果。",
            params: ["first_position" => "第一个背包位置。", "second_position" => "第二个背包位置。"],
            returns: "结构化操作结果。",
            examples: ["let result = spirit::try_swap_spirits(1, 2);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "try_store_spirit",
            return_type: "ActionResult",
            "尝试将背包指定位置精灵放入仓库，失败时返回结构化结果。",
            params: ["position" => "背包位置，从 1 开始。"],
            returns: "结构化操作结果。",
            examples: ["let result = spirit::try_store_spirit(6);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_spirit_bag",
            return_type: "SpiritBagInfo",
            "查询当前背包精灵。",
            params: [],
            returns: "背包精灵信息。",
            examples: ["let bag = spirit::get_spirit_bag();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_bag_items",
            return_type: "BagItemInfo[]",
            "查询背包道具列表。",
            params: [],
            returns: "背包道具列表。",
            examples: ["let items = spirit::get_bag_items();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "take_pushed_drops",
            return_type: "BagItemInfo[]",
            "取出当前会话缓存的推送掉落道具列表。",
            params: [],
            returns: "掉落道具列表；调用后会消费缓存。",
            examples: ["let drops = spirit::take_pushed_drops();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "query_skill_pool",
            return_type: "SkillPoolInfo",
            "查询指定背包精灵的技能池。",
            params: ["position" => "背包位置，从 1 开始。"],
            returns: "技能池信息。",
            examples: ["let pool = spirit::query_skill_pool(1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "add_skill_from_pool",
            return_type: "SkillSwitchResult",
            "从技能池给指定精灵添加技能。",
            params: ["position" => "背包位置，从 1 开始。", "skill_id" => "技能 ID。"],
            returns: "技能切换结果。",
            examples: ["let result = spirit::add_skill_from_pool(1, 497);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "switch_skill",
            return_type: "SkillSwitchResult",
            "替换指定精灵技能槽中的技能。",
            params: ["position" => "背包位置，从 1 开始。", "skill_slot" => "技能槽位，从 1 开始。", "skill_id" => "新技能 ID。"],
            returns: "技能切换结果。",
            examples: ["let result = spirit::switch_skill(1, 1, 497);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "use_skill_stone_preview",
            return_type: "SkillStoneResult",
            "预览指定精灵使用技能石后的候选技能。",
            params: ["position" => "背包位置，从 1 开始。", "item_id" => "技能石道具 ID。"],
            returns: "技能石预览结果。",
            examples: ["let preview = spirit::use_skill_stone_preview(1, 300000);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "use_skill_stone_apply",
            return_type: "SkillStoneResult",
            "对指定精灵应用技能石。",
            params: ["position" => "背包位置，从 1 开始。", "item_id" => "技能石道具 ID。"],
            returns: "技能石应用结果。",
            examples: ["let result = spirit::use_skill_stone_apply(1, 300000);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "use_skill_stone_replace",
            return_type: "SkillStoneResult",
            "用技能石结果替换指定旧技能。",
            params: ["position" => "背包位置，从 1 开始。", "item_id" => "技能石道具 ID。", "old_skill_id" => "要替换的旧技能 ID。", "new_skill_id" => "要学习的新技能 ID。"],
            returns: "技能石替换结果。",
            examples: ["let result = spirit::use_skill_stone_replace(1, 300000, 1, 497);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "get_skills",
            return_type: "SpiritSkillInfo[]",
            "查询指定背包精灵当前技能。",
            params: ["position" => "背包位置，从 1 开始。"],
            returns: "技能数组。",
            examples: ["let skills = spirit::get_skills(1);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "equip_item",
            return_type: "bool",
            "给指定精灵装备道具。",
            params: ["position" => "背包位置，从 1 开始。", "equipment_server_id" => "装备服务器 ID。", "equipment_catch_time" => "装备捕获时间。", "spirit_id" => "宠物 ID。", "spirit_catch_time" => "宠物捕获时间。"],
            returns: "装备成功返回 true。",
            examples: ["spirit::equip_item(1, 100, 0, 1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "list_equipment_bag",
            return_type: "SpiritEquipmentBagInfo",
            "查询精灵装备背包。",
            params: [],
            returns: "装备背包信息。",
            examples: ["let bag = spirit::list_equipment_bag();"]
        ),
        super::stdlib_doc!(
            "spirit",
            "unequip_item",
            return_type: "bool",
            "卸下指定精灵装备。",
            params: ["equipment_server_id" => "装备服务器 ID。", "equipment_catch_time" => "装备捕获时间。", "spirit_id" => "宠物 ID。", "spirit_catch_time" => "宠物捕获时间。"],
            returns: "卸下成功返回 true。",
            examples: ["spirit::unequip_item(100, 0, 1, 0);"]
        ),
        super::stdlib_doc!(
            "spirit",
            "unequip_all_items",
            return_type: "bool",
            "卸下指定精灵全部装备。",
            params: ["spirit_id" => "宠物 ID。", "spirit_catch_time" => "宠物捕获时间。"],
            returns: "卸下成功返回 true。",
            examples: ["spirit::unequip_all_items(1, 0);"]
        ),
    ]
}
