use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "spirit_book",
            "get_my_states",
            "spirit_book::get_my_states() -> SpiritBookStates",
            "查询当前账号图鉴拥有状态。",
            params: [],
            returns: "图鉴拥有状态。",
            examples: ["let states = spirit_book::get_my_states();"]
        ),
        super::stdlib_doc!(
            "spirit_book",
            "get_role_states",
            "spirit_book::get_role_states(uin: int) -> SpiritBookStates",
            "查询指定角色的图鉴拥有状态。",
            params: ["uin" => "角色 uin。"],
            returns: "图鉴拥有状态。",
            examples: ["let states = spirit_book::get_role_states(123456);"]
        ),
        super::stdlib_doc!(
            "spirit_book",
            "get_my_spirit_state",
            "spirit_book::get_my_spirit_state(spirit_id: int) -> SpiritBookSpiritState",
            "查询当前账号指定宠物的图鉴状态。",
            params: ["spirit_id" => "宠物 ID。"],
            returns: "指定宠物图鉴状态。",
            examples: ["let state = spirit_book::get_my_spirit_state(1);"]
        ),
        super::stdlib_doc!(
            "spirit_book",
            "get_role_spirit_state",
            "spirit_book::get_role_spirit_state(uin: int, spirit_id: int) -> SpiritBookSpiritState",
            "查询指定角色某个宠物的图鉴状态。",
            params: ["uin" => "角色 uin。", "spirit_id" => "宠物 ID。"],
            returns: "指定宠物图鉴状态。",
            examples: ["let state = spirit_book::get_role_spirit_state(123456, 1);"]
        ),
    ]
}
