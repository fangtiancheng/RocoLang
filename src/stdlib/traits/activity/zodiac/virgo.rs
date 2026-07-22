use super::super::super::*;

pub trait RocoVirgoActivityStdLib: Send {
    fn virgo_serve_god_query(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_query")
    }
    fn virgo_serve_god_accept_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_accept_task")
    }
    fn virgo_serve_god_give_up_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_give_up_task")
    }
    fn virgo_serve_god_finish_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_finish_task")
    }
    fn virgo_serve_god_buy_unlock(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_buy_unlock")
    }
    fn virgo_serve_god_settle_boss_combat(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_settle_boss_combat")
    }
    fn virgo_serve_god_query_bag(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_query_bag")
    }
    fn virgo_serve_god_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_upgrade_to_100")
    }
    fn virgo_serve_god_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_upgrade")
    }
    fn virgo_find_halidom_query(&mut self, _altar: bool) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_query")
    }
    fn virgo_find_halidom_clean(&mut self, _relic_index: i64) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_clean")
    }
    fn virgo_find_halidom_list_pet(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_list_pet")
    }
    fn virgo_find_halidom_put_pet(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_put_pet")
    }
    fn virgo_find_halidom_buy_pass(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_pass")
    }
    fn virgo_find_halidom_buy_search_count(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_search_count")
    }
    fn virgo_find_halidom_buy_top_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_top_level")
    }
    fn virgo_bell_fox_query_status(&mut self) -> Result<VirgoBellFoxStatusInfo> {
        unsupported("virgo::bell_fox_query_status")
    }
    fn virgo_bell_fox_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<VirgoBellFoxExchangeInfo> {
        unsupported("virgo::bell_fox_exchange_item")
    }
    fn virgo_bell_fox_buy_tail(&mut self, _count: i64) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_buy_tail")
    }
    fn virgo_bell_fox_buy_wish(&mut self) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_buy_wish")
    }
    fn virgo_bell_fox_exchange_pet(&mut self) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_exchange_pet")
    }
}
