use super::super::super::*;

pub trait RocoSagittariusActivityStdLib: Send {
    fn sagittarius_first_query(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_query")
    }
    fn sagittarius_first_submit(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_submit")
    }
    fn sagittarius_first_query_scene(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_query_scene")
    }
    fn sagittarius_first_pick_shine(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_pick_shine")
    }
    fn sagittarius_first_add_map(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_add_map")
    }
    fn sagittarius_first_show_map(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_show_map")
    }
    fn sagittarius_first_add_speed(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_add_speed")
    }
    fn sagittarius_first_complete(&mut self, _index: i64) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_complete")
    }
    fn sagittarius_first_get_pet(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_get_pet")
    }
    fn sagittarius_second_query(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_query")
    }
    fn sagittarius_second_start(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_start")
    }
    fn sagittarius_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_level_up")
    }
    fn sagittarius_second_buy_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_buy_up")
    }
    fn sagittarius_second_random(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_random")
    }
    fn sagittarius_second_task(&mut self, _task_type: i64) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_task")
    }
    fn sagittarius_second_query_bag(&mut self, _bag_type: i64) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_query_bag")
    }
    fn sagittarius_second_spirit_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_spirit_up")
    }
    fn sagittarius_third_query(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_query")
    }
    fn sagittarius_third_settle_combat(
        &mut self,
        _boss_index: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_settle_combat")
    }
    fn sagittarius_third_buy_pet(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_pet")
    }
    fn sagittarius_third_buy_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_evolve")
    }
    fn sagittarius_third_add_challenge_count(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_add_challenge_count")
    }
    fn sagittarius_third_buy_star_num(
        &mut self,
        _score_index: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_star_num")
    }
    fn sagittarius_third_query_bag(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_query_bag")
    }
    fn sagittarius_third_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_up")
    }
}
