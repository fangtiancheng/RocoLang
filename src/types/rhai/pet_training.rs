use super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
