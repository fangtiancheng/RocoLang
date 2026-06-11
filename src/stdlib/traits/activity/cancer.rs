use super::super::*;

pub trait RocoCancerActivityStdLib: Send {
    fn cancer_sharp_scorpion_query(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_query")
    }
    fn cancer_sharp_scorpion_exchange_item(
        &mut self,
        _exc_pos: i64,
    ) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_exchange_item")
    }
    fn cancer_sharp_scorpion_buy_tail(&mut self, _num: i64) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_buy_tail")
    }
    fn cancer_sharp_scorpion_buy_wish(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_buy_wish")
    }
    fn cancer_sharp_scorpion_exc_pet(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_exc_pet")
    }
    fn cancer_mend_shape_query(&mut self) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_query")
    }
    fn cancer_mend_shape_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_upgrade")
    }
    fn cancer_mend_shape_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_upgrade_to_100")
    }
    fn cancer_mend_shape_buy(&mut self, _buy_type: i64) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_buy")
    }
    fn cancer_mend_shape_buy_full(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_buy_full")
    }
    fn cancer_mend_shape_query_bag(&mut self) -> Result<CancerMendShapeBagInfo> {
        unsupported("cancer::mend_shape_query_bag")
    }
    fn cancer_unseal_memories_query(&mut self) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_query")
    }
    fn cancer_unseal_memories_start_game(&mut self) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_start_game")
    }
    fn cancer_unseal_memories_commit(&mut self, _choice: i64) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_commit")
    }
    fn cancer_unseal_memories_bag_query(&mut self) -> Result<CancerUnsealMemoriesBagInfo> {
        unsupported("cancer::unseal_memories_bag_query")
    }
    fn cancer_unseal_memories_put_full(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerUnsealMemoriesBagInfo> {
        unsupported("cancer::unseal_memories_put_full")
    }
    fn cancer_unseal_memories_buy(&mut self, _buy_type: i64) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_buy")
    }
}
