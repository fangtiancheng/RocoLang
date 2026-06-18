use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "manor",
            "get_ground_info",
            "manor::get_ground_info() -> ManorGroundInfo",
            "查询家园农场土地信息。",
            params: [],
            returns: "土地信息。",
            examples: ["let ground = manor::get_ground_info();"]
        ),
        super::stdlib_doc!(
            "manor",
            "get_seed_bag",
            "manor::get_seed_bag() -> ManorSeedInfo[]",
            "查询家园农场种子背包。",
            params: [],
            returns: "种子列表。",
            examples: ["let seeds = manor::get_seed_bag();"]
        ),
        super::stdlib_doc!(
            "manor",
            "sow",
            "manor::sow(seed_id: int, ground_id: int) -> bool",
            "在指定土地播种。",
            params: ["seed_id" => "种子 ID。", "ground_id" => "土地 ID。"],
            returns: "播种成功返回 true。",
            examples: ["manor::sow(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "reap",
            "manor::reap(ground_id: int) -> bool",
            "收获指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "收获成功返回 true。",
            examples: ["manor::reap(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "uproot",
            "manor::uproot(ground_id: int) -> bool",
            "铲除指定土地作物。",
            params: ["ground_id" => "土地 ID。"],
            returns: "铲除成功返回 true。",
            examples: ["manor::uproot(1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "weed",
            "manor::weed(ground_id: int, weed_type: int) -> bool",
            "清理指定土地杂草。",
            params: ["ground_id" => "土地 ID。", "weed_type" => "杂草类型。"],
            returns: "清理成功返回 true。",
            examples: ["manor::weed(1, 1);"]
        ),
        super::stdlib_doc!(
            "manor",
            "use_fertilizer",
            "manor::use_fertilizer(ground_id: int, fertilizer_item_id: int) -> bool",
            "对指定土地使用肥料。",
            params: ["ground_id" => "土地 ID。", "fertilizer_item_id" => "肥料道具 ID。"],
            returns: "使用成功返回 true。",
            examples: ["manor::use_fertilizer(1, 300000);"]
        ),
    ]
}
