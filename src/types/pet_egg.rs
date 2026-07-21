use serde::{Deserialize, Serialize};

use super::SpiritSkillInfo;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggSpiritInfo {
    pub spirit_id: i64,
    pub level: i64,
    pub exp_to_next_level: i64,
    pub personality: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub caught_time: i64,
    pub caught_location: i64,
    pub storage_time: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggInfo {
    pub result_code: i64,
    pub message: String,
    pub current_egg_count: i64,
    pub max_egg_count: i64,
    pub vip_count: i64,
    pub male: PetEggSpiritInfo,
    pub female: PetEggSpiritInfo,
    pub egg: PetEggSpiritInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggSpeedUpResult {
    pub result_code: i64,
    pub message: String,
    pub current_egg_count: i64,
    pub max_egg_count: i64,
    pub vip_count: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggBeginResult {
    pub result_code: i64,
    pub message: String,
    pub max_egg_count: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggCancelResult {
    pub result_code: i64,
    pub message: String,
    pub detail_code: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetEggPreviewResult {
    pub result_code: i64,
    pub message: String,
    pub egg: PetEggSpiritInfo,
}
