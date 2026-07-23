use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "pet_egg",
            "query_info",
            return_type: "PetEggInfo",
            "查询宠物培育室当前的父母宠物、宠物蛋和容量信息。",
            params: [],
            returns: "PetEggInfo，包含结果码、培育容量以及父方、母方和宠物蛋信息。",
            examples: ["let info = pet_egg::query_info();"]
        ),
        super::stdlib_doc!(
            "pet_egg",
            "vip_speed_up",
            return_type: "PetEggSpeedUpResult",
            "使用 VIP 加速当前宠物蛋的培育过程。",
            params: [],
            returns: "PetEggSpeedUpResult，包含操作结果和更新后的宠物蛋容量信息。",
            examples: ["let result = pet_egg::vip_speed_up();"]
        ),
        super::stdlib_doc!(
            "pet_egg",
            "begin",
            return_type: "PetEggBeginResult",
            "使用背包中的两只宠物开始培育宠物蛋。",
            params: ["male_index" => "父方宠物的背包索引。", "female_index" => "母方宠物的背包索引。"],
            returns: "PetEggBeginResult，包含操作结果和当前最大宠物蛋容量。",
            examples: ["let result = pet_egg::begin(0, 1);"]
        ),
        super::stdlib_doc!(
            "pet_egg",
            "cancel",
            return_type: "PetEggCancelResult",
            "取消当前宠物蛋培育。服务端协议不需要额外参数。",
            params: [],
            returns: "PetEggCancelResult，包含操作结果和服务端详细结果码。",
            examples: ["let result = pet_egg::cancel();"]
        ),
        super::stdlib_doc!(
            "pet_egg",
            "preview",
            return_type: "PetEggPreviewResult",
            "预览两只背包宠物可能培育出的宠物蛋信息。",
            params: ["male_index" => "父方宠物的背包索引。", "female_index" => "母方宠物的背包索引。"],
            returns: "PetEggPreviewResult，egg 字段包含预览生成的宠物蛋信息。",
            examples: ["let preview = pet_egg::preview(0, 1);"]
        ),
    ]
}
