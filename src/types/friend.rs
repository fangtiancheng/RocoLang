use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendApplication {
    pub sender_uin: i64,
    pub sender_nickname: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendApplicationHandleResult {
    pub accepted: bool,
    pub friend_uin: i64,
    pub friend_nickname: String,
    pub applications: Vec<FriendApplication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendOnlineStatus {
    pub uin: i64,
    pub online_state: i64,
    pub room_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendDetail {
    pub uin: i64,
    pub vip_level: i64,
    pub version: i64,
    pub roco_nickname: String,
    pub qq_nickname: String,
    pub qq_icon_url: String,
}
