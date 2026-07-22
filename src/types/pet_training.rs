use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetTrainingRewardItem {
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetTrainingResult {
    pub ok: bool,
    pub result_code: i64,
    pub message: String,
    pub training_type: i64,
    pub pet_id: i64,
    pub rewards: Vec<PetTrainingRewardItem>,
    pub raw_text: String,
}
