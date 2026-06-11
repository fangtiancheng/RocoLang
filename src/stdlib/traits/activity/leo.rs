use super::super::*;

pub trait RocoLeoActivityStdLib: Send {
    fn leo_first_query_status(&mut self) -> Result<LeoFirstStatusInfo> {
        unsupported("leo::first_query_status")
    }
    fn leo_first_exchange_item(&mut self, _exchange_position: i64) -> Result<LeoFirstExchangeInfo> {
        unsupported("leo::first_exchange_item")
    }
    fn leo_first_buy_tail(&mut self, _count: i64) -> Result<LeoFirstInfo> {
        unsupported("leo::first_buy_tail")
    }
    fn leo_first_buy_wish(&mut self) -> Result<LeoFirstInfo> {
        unsupported("leo::first_buy_wish")
    }
    fn leo_first_exchange_pet(&mut self) -> Result<LeoFirstInfo> {
        unsupported("leo::first_exchange_pet")
    }
    fn leo_second_query(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_query")
    }
    fn leo_second_settle_combat(&mut self, _hunt_index: i64) -> Result<LeoSecondInfo> {
        unsupported("leo::second_settle_combat")
    }
    fn leo_second_submit_onekey(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_submit_onekey")
    }
    fn leo_second_query_spirit(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_query_spirit")
    }
    fn leo_second_submit_spirit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LeoSecondInfo> {
        unsupported("leo::second_submit_spirit")
    }
    fn leo_second_buy_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LeoSecondInfo> {
        unsupported("leo::second_buy_full_level")
    }
    fn leo_third_query(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_query")
    }
    fn leo_third_submit_combat(
        &mut self,
        _challenge_index: i64,
        _win: bool,
    ) -> Result<LeoThirdInfo> {
        unsupported("leo::third_submit_combat")
    }
    fn leo_third_get_reward(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_get_reward")
    }
    fn leo_third_full_level(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_full_level")
    }
    fn leo_third_evolve(&mut self, _catch_time: i64) -> Result<LeoThirdInfo> {
        unsupported("leo::third_evolve")
    }
    fn leo_third_light_star(&mut self, _star_index: i64) -> Result<LeoThirdInfo> {
        unsupported("leo::third_light_star")
    }
    fn leo_third_query_bag(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_query_bag")
    }
}
