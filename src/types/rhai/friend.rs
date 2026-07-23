use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        FriendApplication,
        sender_uin,
        sender_nickname,
        timestamp
    );
    register_getters!(
        engine,
        FriendApplicationHandleResult,
        accepted,
        friend_uin,
        friend_nickname
    );
    engine.register_get(
        "applications",
        |value: &mut FriendApplicationHandleResult| to_array(&value.applications),
    );
    register_getters!(engine, FriendOnlineStatus, uin, online_state, room_id);
    register_getters!(
        engine,
        FriendDetail,
        uin,
        vip_level,
        version,
        roco_nickname,
        qq_nickname,
        qq_icon_url
    );
}
