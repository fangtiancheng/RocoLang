use super::super::super::*;

pub trait RocoPetTrainingActivityStdLib: Send {
    fn pet_training_query(&mut self, _training_type: i64) -> Result<PetTrainingResult> {
        unsupported("pet_training::query")
    }

    fn pet_training_settle(
        &mut self,
        _training_type: i64,
        _pet_id: i64,
    ) -> Result<PetTrainingResult> {
        unsupported("pet_training::settle")
    }
}
