use super::super::super::*;

pub trait RocoPiscesActivityStdLib: Send {
    fn pisces_first_query(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_query")
    }
    fn pisces_first_submit(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_submit")
    }
    fn pisces_first_get_gift(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_get_gift")
    }
    fn pisces_first_exchange(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_exchange")
    }
    fn pisces_first_buy(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_buy")
    }
    fn pisces_second_query(&mut self) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_query")
    }
    fn pisces_second_submit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_submit")
    }
    fn pisces_second_submit_without_spirit(&mut self) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_submit_without_spirit")
    }
    fn pisces_second_get_gift(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_get_gift")
    }
    fn pisces_second_settle_fight(&mut self, _fight_index: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_settle_fight")
    }
    fn pisces_second_repair(&mut self, _repair_index: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_repair")
    }
    fn pisces_second_view(&mut self, _kind: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_view")
    }
    fn pisces_second_evolution(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_evolution")
    }
    fn pisces_third_query(&mut self) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_query")
    }
    fn pisces_third_settle_fight(&mut self, _boss_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_settle_fight")
    }
    fn pisces_third_buy(
        &mut self,
        _kind: i64,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_buy")
    }
    fn pisces_third_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_full_level")
    }
    fn pisces_third_complete(&mut self, _level_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_complete")
    }
    fn pisces_third_get_item(&mut self, _reward_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_get_item")
    }
    fn pisces_third_query_bag(&mut self, _kind: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_query_bag")
    }
    fn pisces_third_up(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_up")
    }
}
