use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("friend", "query_initial_state", return_type: "int", "查询好友系统初始状态。", params: [], returns: "服务端初始状态值。", examples: ["let state = friend::query_initial_state();"]),
        super::stdlib_doc!("friend", "send_application", return_type: "()", "向指定 UIN 发送好友申请。", params: ["friend_uin" => "目标好友 UIN。"], returns: "成功返回空值。", examples: ["friend::send_application(123456);"]),
        super::stdlib_doc!("friend", "handle_application", return_type: "FriendApplicationHandleResult", "接受或拒绝好友申请。", params: ["sender_uin" => "申请人 UIN。", "accept" => "是否接受申请。"], returns: "处理结果和剩余申请列表。", examples: ["let result = friend::handle_application(123456, true);"]),
        super::stdlib_doc!("friend", "delete", return_type: "()", "删除指定好友。", params: ["friend_uin" => "好友 UIN。"], returns: "成功返回空值。", examples: ["friend::delete(123456);"]),
        super::stdlib_doc!("friend", "send_chat", return_type: "int", "向好友发送聊天消息。", params: ["friend_uin" => "好友 UIN。", "message" => "聊天内容。"], returns: "服务端返回的发送者 VIP 等级。", examples: ["let vip_level = friend::send_chat(123456, \"你好\");"]),
        super::stdlib_doc!("friend", "query_online", return_type: "FriendOnlineStatus[]", "查询好友在线状态。", params: [], returns: "好友在线状态和房间列表。", examples: ["let online = friend::query_online();"]),
        super::stdlib_doc!("friend", "query_nicknames", return_type: "FriendNickname[]", "批量查询好友昵称对象。", params: ["friend_uins" => "好友 UIN 数组。"], returns: "包含 uin 和 nickname 字段的好友昵称对象列表。", examples: ["let friends = friend::query_nicknames([123456]);\nprint(friends[0].nickname);"]),
    ]
}
