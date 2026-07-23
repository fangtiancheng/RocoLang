use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "incubative_machine",
            "query_egg_list",
            return_type: "IncubativeMachineEggListResult",
            "分页查询可用于孵化的宠物蛋。",
            params: [
                "egg_type" => "宠物蛋类型。",
                "page_index" => "从 1 开始的页码。"
            ],
            returns: "返回宠物蛋列表、当前页和总页数。",
            examples: ["let result = incubative_machine::query_egg_list(0, 1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "query_info",
            return_type: "IncubativeMachineInfo",
            "查询指定孵化槽的当前状态。",
            params: ["which" => "孵化槽编号。"],
            returns: "返回孵化槽、可用宠物蛋和当前孵化进度。",
            examples: ["let info = incubative_machine::query_info(1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "begin",
            return_type: "IncubativeMachineIncubationResult",
            "将指定宠物蛋放入孵化槽开始孵化。",
            params: [
                "which" => "孵化槽编号。",
                "egg_type" => "宠物蛋类型。",
                "egg_id" => "宠物蛋 ID。",
                "catch_time" => "宠物蛋捕获时间。",
                "egg_uin" => "宠物蛋实例 UIN。"
            ],
            returns: "返回开始孵化后的槽位状态。",
            examples: ["let result = incubative_machine::begin(1, 0, 67305577, 0, 0);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "vip_speed_up",
            return_type: "IncubativeMachineIncubationResult",
            "使用 VIP 能力加速指定孵化槽。",
            params: ["which" => "孵化槽编号。"],
            returns: "返回加速后的槽位状态。",
            examples: ["let result = incubative_machine::vip_speed_up(1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "teach",
            return_type: "IncubativeMachineIncubationResult",
            "执行指定孵化槽的蛋教操作。",
            params: ["which" => "孵化槽编号。"],
            returns: "返回蛋教后的槽位状态。",
            examples: ["let result = incubative_machine::teach(1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "terminate",
            return_type: "IncubativeMachineActionResult",
            "终止指定孵化槽中的孵化。",
            params: ["which" => "孵化槽编号。"],
            returns: "返回终止操作结果。",
            examples: ["let result = incubative_machine::terminate(1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "get_spirit",
            return_type: "IncubativeMachineGetSpiritResult",
            "领取指定孵化槽中已经孵化完成的宠物。",
            params: ["which" => "孵化槽编号。"],
            returns: "返回领取结果、宠物 ID 和宠物等级。",
            examples: ["let result = incubative_machine::get_spirit(1);"]
        ),
        super::stdlib_doc!(
            "incubative_machine",
            "set_complete_guide",
            return_type: "IncubativeMachineActionResult",
            "标记孵化引导流程已完成。",
            params: [],
            returns: "返回引导标记操作结果。",
            examples: ["let result = incubative_machine::set_complete_guide();"]
        ),
    ]
}
