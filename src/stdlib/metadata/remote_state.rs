use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "remote_state",
            "load_scene_data",
            "remote_state::load_scene_data(scene_id: int) -> blob",
            "读取指定场景或业务命名空间的远程持久化字节。",
            params: ["scene_id" => "场景 ID 或业务保存 ID。"],
            returns: "服务器保存的原始字节。",
            examples: ["let values = remote_state::load_scene_data(245);"]
        ),
        super::stdlib_doc!(
            "remote_state",
            "update_scene_data",
            "remote_state::update_scene_data(scene_id: int, values: blob) -> blob",
            "覆盖指定场景或业务命名空间的远程持久化字节。",
            params: ["scene_id" => "场景 ID 或业务保存 ID。", "values" => "要保存的原始字节。"],
            returns: "服务器确认后的原始字节。",
            examples: ["remote_state::update_scene_data(40000, blob(0));"]
        ),
        super::stdlib_doc!(
            "remote_state",
            "load_npc_value",
            "remote_state::load_npc_value(npc_id: int) -> int",
            "读取指定 NPC 的账号远程状态值。",
            params: ["npc_id" => "NPC ID。"],
            returns: "0 到 255 的远程状态值。",
            examples: ["let value = remote_state::load_npc_value(1001);"]
        ),
        super::stdlib_doc!(
            "remote_state",
            "update_npc_value",
            "remote_state::update_npc_value(npc_id: int, value: int) -> int",
            "更新指定 NPC 的账号远程状态值。",
            params: ["npc_id" => "NPC ID。", "value" => "0 到 255 的新值。"],
            returns: "服务器确认后的状态值。",
            examples: ["remote_state::update_npc_value(1001, 1);"]
        ),
    ]
}
