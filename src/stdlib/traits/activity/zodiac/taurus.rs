use super::super::super::*;

pub trait RocoTaurusActivityStdLib: Send {
    fn taurus_first_query(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_query")
    }
    fn taurus_first_get_leather(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_leather")
    }
    fn taurus_first_get_nail(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_nail")
    }
    fn taurus_first_get_ding(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_ding")
    }
    fn taurus_first_get_glue(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_glue")
    }
    fn taurus_first_mix(&mut self, _part_index: i64) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_mix")
    }
    fn taurus_first_buy_pet(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_buy_pet")
    }
    fn taurus_first_buy_item(&mut self, _item_index: i64, _count: i64) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_buy_item")
    }
    fn taurus_second_query(&mut self) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_query")
    }
    fn taurus_second_query_bag(&mut self) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_query_bag")
    }
    fn taurus_second_submit_game(
        &mut self,
        _game_index: i64,
        _score: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_submit_game")
    }
    fn taurus_second_buy_game(&mut self, _game_index: i64) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_buy_game")
    }
    fn taurus_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_evolve")
    }
    fn taurus_second_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_buy_level")
    }
    fn taurus_third_query(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_query")
    }
    fn taurus_third_settle_npc_fight(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_settle_npc_fight")
    }
    fn taurus_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_evolve")
    }
    fn taurus_third_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_buy_level")
    }
    fn taurus_third_buy_score(&mut self, _kind: i64) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_buy_score")
    }
    fn taurus_third_query_bag(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_query_bag")
    }
    fn taurus_third_get_task(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_get_task")
    }
}
