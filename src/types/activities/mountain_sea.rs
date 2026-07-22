use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaInfo {
    pub result_code: i64,
    pub message: String,
    pub fight_id: i64,
    pub seal_count: i64,
    pub success: i64,
    pub attrs: Vec<i64>,
    pub bosses: Vec<MountainSeaBossInfo>,
    pub souls: Vec<MountainSeaSoulInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaBossInfo {
    pub index: i64,
    pub boss_type: i64,
    pub fight_id: i64,
    pub name: String,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountainSeaSoulInfo {
    pub soul_type: i64,
    pub boss_type: i64,
    pub name: String,
    pub count: i64,
}
