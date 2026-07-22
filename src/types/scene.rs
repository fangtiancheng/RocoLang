use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSpiritInfo {
    pub spirit_id: i64,
    pub count: i64,
    pub area_index: i64,
    pub is_rare: bool,
    pub is_boss: bool,
    pub is_npc_boss: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneRoleInfo {
    pub uin: i64,
    pub id: i64,
    pub nick_name: String,
    pub level: i64,
    pub loc_x: i64,
    pub loc_y: i64,
    pub pk_state: i64,
    pub is_in_combat: bool,
    pub is_vip: bool,
    pub vip_level: i64,
    pub trainer_level: i64,
    pub trainer_exp: i64,
}
