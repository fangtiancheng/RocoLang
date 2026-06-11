use super::super::*;

pub trait RocoLibraActivityStdLib: Send {
    fn libra_first_query(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_query")
    }
    fn libra_first_submit_game(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_submit_game")
    }
    fn libra_first_settle_fight(&mut self, _prop_index: i64) -> Result<LibraFirstInfo> {
        unsupported("libra::first_settle_fight")
    }
    fn libra_first_get_gift(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_get_gift")
    }
    fn libra_first_notify_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraFirstInfo> {
        unsupported("libra::first_notify_full_level")
    }
    fn libra_first_query_bag(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_query_bag")
    }
    fn libra_first_advance(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<LibraFirstInfo> {
        unsupported("libra::first_advance")
    }
    fn libra_second_query(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_query")
    }
    fn libra_second_settle_fight(&mut self, _npc_index: i64) -> Result<LibraSecondInfo> {
        unsupported("libra::second_settle_fight")
    }
    fn libra_second_awaken(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_awaken")
    }
    fn libra_second_buy_challenge_count(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_buy_challenge_count")
    }
    fn libra_second_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraSecondInfo> {
        unsupported("libra::second_full_level")
    }
    fn libra_second_query_bag(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_query_bag")
    }
    fn libra_second_evolution(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraSecondInfo> {
        unsupported("libra::second_evolution")
    }
    fn libra_third_query_status(&mut self) -> Result<LibraThirdStatusInfo> {
        unsupported("libra::third_query_status")
    }
    fn libra_third_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<LibraThirdExchangeInfo> {
        unsupported("libra::third_exchange_item")
    }
    fn libra_third_buy_tail(&mut self, _count: i64) -> Result<LibraThirdInfo> {
        unsupported("libra::third_buy_tail")
    }
    fn libra_third_buy_wish(&mut self) -> Result<LibraThirdInfo> {
        unsupported("libra::third_buy_wish")
    }
    fn libra_third_exchange_pet(&mut self) -> Result<LibraThirdInfo> {
        unsupported("libra::third_exchange_pet")
    }
}
