use super::super::*;

pub trait RocoManorActivityStdLib: Send {
    fn manor_get_ground_info(&mut self) -> Result<ManorInfo> {
        unsupported("manor::get_ground_info")
    }
    fn manor_get_seed_bag(&mut self) -> Result<Vec<ManorItemCount>> {
        unsupported("manor::get_seed_bag")
    }
    fn manor_sow(&mut self, _seed_id: i64, _ground_id: i64) -> Result<ManorSowResult> {
        unsupported("manor::sow")
    }
    fn manor_reap(&mut self, _ground_id: i64) -> Result<ManorReapResult> {
        unsupported("manor::reap")
    }
    fn manor_uproot(&mut self, _ground_id: i64) -> Result<ManorUprootResult> {
        unsupported("manor::uproot")
    }
    fn manor_weed(&mut self, _ground_id: i64, _weed_type: i64) -> Result<ManorWeedResult> {
        unsupported("manor::weed")
    }
    fn manor_use_fertilizer(
        &mut self,
        _ground_id: i64,
        _fertilizer_item_id: i64,
    ) -> Result<ManorFertilizerResult> {
        unsupported("manor::use_fertilizer")
    }
}
