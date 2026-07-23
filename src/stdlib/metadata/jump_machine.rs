use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("jump_machine", "play", return_type: "JumpMachineInfo", "进行一次弹弹机玩法操作。", params: [], returns: "返回弹弹机进度和奖励状态。", examples: ["let info = jump_machine::play();"]),
        super::stdlib_doc!("jump_machine", "query", return_type: "JumpMachineInfo", "查询弹弹机玩法状态。", params: [], returns: "返回弹弹机进度和奖励状态。", examples: ["let info = jump_machine::query();"]),
    ]
}
