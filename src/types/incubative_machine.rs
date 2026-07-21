use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineEggInfo {
    pub egg_id: i64,
    pub incubate_time: i64,
    pub property: String,
    pub catch_time: i64,
    pub egg_uin: i64,
    pub egg_name: String,
    pub roco_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineIncubationInfo {
    pub egg_type: i64,
    pub spirit_id: i64,
    pub egg_id: i64,
    pub percentile: i64,
    pub remainder_time: i64,
    pub stage: i64,
    pub egg_name: String,
    pub property: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineEggListResult {
    pub result_code: i64,
    pub message: String,
    pub eggs: Vec<IncubativeMachineEggInfo>,
    pub egg_type: i64,
    pub total_pages: i64,
    pub current_page: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineInfo {
    pub result_code: i64,
    pub message: String,
    pub guide: i64,
    pub incubator_type: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub eggs: Vec<IncubativeMachineEggInfo>,
    pub incubation: IncubativeMachineIncubationInfo,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineIncubationResult {
    pub result_code: i64,
    pub message: String,
    pub incubation: IncubativeMachineIncubationInfo,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineActionResult {
    pub result_code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncubativeMachineGetSpiritResult {
    pub result_code: i64,
    pub message: String,
    pub spirit_id: i64,
    pub spirit_level: i64,
}
