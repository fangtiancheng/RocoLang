use super::super::super::*;

pub trait RocoAriesActivityStdLib: Send {
    fn aries_first_query(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_query")
    }
    fn aries_first_start(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_start")
    }
    fn aries_first_buy_times(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_buy_times")
    }
    fn aries_first_dice(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_dice")
    }
    fn aries_first_settle_battle(&mut self, _battle_type: i64) -> Result<AriesFirstInfo> {
        unsupported("aries::first_settle_battle")
    }
    fn aries_first_query_bag(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_query_bag")
    }
    fn aries_first_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesFirstInfo> {
        unsupported("aries::first_level_up")
    }
    fn aries_first_evolve(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<AriesFirstInfo> {
        unsupported("aries::first_evolve")
    }
    fn aries_first_buy_direct(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_buy_direct")
    }
    fn aries_first_get_gold(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_get_gold")
    }
    fn aries_second_query(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_query")
    }
    fn aries_second_submit_game(&mut self, _power: i64) -> Result<AriesSecondInfo> {
        unsupported("aries::second_submit_game")
    }
    fn aries_second_query_bag(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_query_bag")
    }
    fn aries_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesSecondInfo> {
        unsupported("aries::second_evolve")
    }
    fn aries_second_buy_direct(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_buy_direct")
    }
    fn aries_second_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesSecondInfo> {
        unsupported("aries::second_buy_level")
    }
    fn aries_third_query_status(&mut self) -> Result<AriesThirdStatusInfo> {
        unsupported("aries::third_query_status")
    }
    fn aries_third_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<AriesThirdExchangeInfo> {
        unsupported("aries::third_exchange_item")
    }
    fn aries_third_buy_tail(&mut self, _count: i64) -> Result<AriesThirdInfo> {
        unsupported("aries::third_buy_tail")
    }
    fn aries_third_buy_wish(&mut self) -> Result<AriesThirdInfo> {
        unsupported("aries::third_buy_wish")
    }
    fn aries_third_exchange_pet(&mut self) -> Result<AriesThirdInfo> {
        unsupported("aries::third_exchange_pet")
    }
}
