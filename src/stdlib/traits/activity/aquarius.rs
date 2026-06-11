use super::super::*;

pub trait RocoAquariusActivityStdLib: Send {
    fn aquarius_first_query(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_query")
    }
    fn aquarius_first_settle_combat(&mut self, _boss_index: i64) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_settle_combat")
    }
    fn aquarius_first_buy_evolve_access(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_evolve_access")
    }
    fn aquarius_first_add_challenge_count(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_add_challenge_count")
    }
    fn aquarius_first_buy_star_num(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_star_num")
    }
    fn aquarius_first_query_bag(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_query_bag")
    }
    fn aquarius_first_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_evolve")
    }
    fn aquarius_first_buy_direct(
        &mut self,
        _item_or_spirit_id: i64,
        _count: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_direct")
    }
    fn aquarius_second_query_status(&mut self) -> Result<AquariusSecondStatusInfo> {
        unsupported("aquarius::second_query_status")
    }
    fn aquarius_second_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<AquariusSecondExchangeInfo> {
        unsupported("aquarius::second_exchange_item")
    }
    fn aquarius_second_query_diamond(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_query_diamond")
    }
    fn aquarius_second_combat_again(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_combat_again")
    }
    fn aquarius_second_buy_tail(&mut self, _count: i64) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_tail")
    }
    fn aquarius_second_buy_wish(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_wish")
    }
    fn aquarius_second_exchange_pet(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_exchange_pet")
    }
    fn aquarius_second_buy_spirit(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_spirit")
    }
    fn aquarius_third_query(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_query")
    }
    fn aquarius_third_random(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_random")
    }
    fn aquarius_third_settle_combat(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_settle_combat")
    }
    fn aquarius_third_buy_level(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_buy_level")
    }
    fn aquarius_third_buy_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_buy_evolve")
    }
    fn aquarius_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_evolve")
    }
    fn aquarius_third_query_bag(&mut self, _bag_type: i64) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_query_bag")
    }
}
