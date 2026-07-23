use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("mystery_fusion", "claim_reward", return_type: "MysteryFusionInfo", "领取神秘融合奖励。", params: [], returns: "返回领取后的融合状态。", examples: ["let info = mystery_fusion::claim_reward();"]),
        super::stdlib_doc!("mystery_fusion", "fuse", return_type: "MysteryFusionInfo", "提交材料进行神秘融合。", params: ["recipe_index" => "融合配方索引。", "material_bag_indexes" => "参与融合的背包位置数组。", "personality" => "融合结果使用的性格 ID。"], returns: "返回融合后的活动状态。", examples: ["let info = mystery_fusion::fuse(0, [1, 2], 0);"]),
        super::stdlib_doc!("mystery_fusion", "prepare_battle", return_type: "MysteryFusionInfo", "准备神秘融合活动战斗。", params: ["battle_index" => "战斗索引。"], returns: "返回准备战斗后的融合状态。", examples: ["let info = mystery_fusion::prepare_battle(0);"]),
        super::stdlib_doc!("mystery_fusion", "query", return_type: "MysteryFusionInfo", "查询神秘融合活动状态。", params: [], returns: "返回配方、材料和奖励信息。", examples: ["let info = mystery_fusion::query();"]),
        super::stdlib_doc!("mystery_fusion", "query_material_bag", return_type: "MysteryFusionMaterialBag", "查询指定宠物可用于神秘融合的材料背包。", params: ["spirit_id" => "宠物 ID。"], returns: "返回该宠物对应的融合材料。", examples: ["let bag = mystery_fusion::query_material_bag(100);"]),
        super::stdlib_doc!("mystery_fusion", "submit_battle", return_type: "MysteryFusionInfo", "提交神秘融合活动战斗结果。", params: [], returns: "返回战斗提交后的融合状态。", examples: ["let info = mystery_fusion::submit_battle();"]),
    ]
}
