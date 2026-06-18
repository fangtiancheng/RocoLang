use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "scene",
            "get_current_scene",
            "scene::get_current_scene() -> int",
            "返回当前场景 ID。",
            params: [],
            returns: "当前场景 ID。",
            examples: ["let scene_id = scene::get_current_scene();"]
        ),
        super::stdlib_doc!(
            "scene",
            "move_to_scene",
            "scene::move_to_scene(scene_id: int, timeout_ms: int = 30000) -> bool",
            "移动到指定场景，并等待场景切换完成。",
            params: ["scene_id" => "目标场景 ID。", "timeout_ms" => "等待超时时间，单位毫秒。"],
            returns: "移动成功返回 true。",
            examples: ["scene::move_to_scene(72, 30000);"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_move_to_scene",
            "scene::try_move_to_scene(scene_id: int, timeout_ms: int = 30000) -> ActionResult",
            "尝试移动到指定场景，失败时返回结构化结果。",
            params: ["scene_id" => "目标场景 ID。", "timeout_ms" => "等待超时时间，单位毫秒。"],
            returns: "操作结果。",
            examples: ["let result = scene::try_move_to_scene(72, 30000);"]
        ),
        super::stdlib_doc!(
            "scene",
            "get_scene_spirits",
            "scene::get_scene_spirits() -> SceneSpiritInfo[]",
            "查询当前场景宠物刷新信息。",
            params: [],
            returns: "当前场景宠物列表。",
            examples: ["let spirits = scene::get_scene_spirits();"]
        ),
    ]
}
