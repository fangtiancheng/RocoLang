use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![super::stdlib_doc!(
        "role",
        "get_cached_scene_roles",
        return_type: "SceneRoleInfo[]",
        "获取当前缓存的场景角色列表。",
        params: [],
        returns: "场景角色数组。",
        examples: ["let roles = role::get_cached_scene_roles();"]
    )]
}
