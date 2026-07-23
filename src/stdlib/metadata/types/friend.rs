use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "FriendApplication" => (
            "好友申请。",
            vec![
                field("sender_uin", "int", "申请人 UIN。"),
                field("sender_nickname", "string", "申请人昵称。"),
                field("timestamp", "int", "申请时间。"),
            ],
        ),
        "FriendApplicationHandleResult" => (
            "好友申请处理结果。",
            vec![
                field("accepted", "bool", "是否接受。"),
                field("friend_uin", "int", "好友 UIN。"),
                field("friend_nickname", "string", "好友昵称。"),
                field("applications", "FriendApplication[]", "剩余申请列表。"),
            ],
        ),
        "FriendOnlineStatus" => (
            "好友在线状态。",
            vec![
                field("uin", "int", "好友 UIN。"),
                field("online_state", "int", "在线状态原始值。"),
                field("room_id", "int", "所在房间 ID。"),
            ],
        ),
        "FriendNickname" => (
            "好友昵称。",
            vec![
                field("uin", "int", "好友 UIN。"),
                field("nickname", "string", "好友昵称。"),
            ],
        ),
        _ => return None,
    })
}
