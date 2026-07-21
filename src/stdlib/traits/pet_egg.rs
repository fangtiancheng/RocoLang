use super::*;

pub trait RocoPetEggStdLib: Send {
    fn pet_egg_query_info(&mut self) -> Result<PetEggInfo> {
        unsupported("pet_egg::query_info")
    }
    fn pet_egg_vip_speed_up(&mut self) -> Result<PetEggSpeedUpResult> {
        unsupported("pet_egg::vip_speed_up")
    }
    fn pet_egg_begin(&mut self, _male_index: i64, _female_index: i64) -> Result<PetEggBeginResult> {
        unsupported("pet_egg::begin")
    }
    fn pet_egg_cancel(&mut self) -> Result<PetEggCancelResult> {
        unsupported("pet_egg::cancel")
    }
    fn pet_egg_preview(
        &mut self,
        _male_index: i64,
        _female_index: i64,
    ) -> Result<PetEggPreviewResult> {
        unsupported("pet_egg::preview")
    }
}
