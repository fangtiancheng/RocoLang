use serde::{Deserialize, Serialize};

use super::{to_array, Engine};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, PetTrainingRewardItem, item_id, count);
    register_getters!(
        engine,
        PetTrainingResult,
        ok,
        result_code,
        message,
        training_type,
        pet_id,
        raw_text
    );
    engine.register_get("rewards", |value: &mut PetTrainingResult| {
        to_array(&value.rewards)
    });
}
