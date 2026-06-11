use super::super::*;

pub trait RocoScorpioActivityStdLib: Send {
    fn scorpio_first_query(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_query")
    }
    fn scorpio_first_submit_game(&mut self, _score: i64) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_submit_game")
    }
    fn scorpio_first_query_bag(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_query_bag")
    }
    fn scorpio_first_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_evolve")
    }
    fn scorpio_first_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_level_up")
    }
    fn scorpio_first_buy_direct(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_buy_direct")
    }
    fn scorpio_second_query(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_query")
    }
    fn scorpio_second_settle(&mut self, _reward_source: i64) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_settle")
    }
    fn scorpio_second_query_bag(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_query_bag")
    }
    fn scorpio_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_evolve")
    }
    fn scorpio_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_level_up")
    }
    fn scorpio_second_buy_direct(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_buy_direct")
    }
    fn scorpio_second_buy_challenge_count(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_buy_challenge_count")
    }
    fn scorpio_third_query(&mut self) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_query")
    }
    fn scorpio_third_submit_game(&mut self, _success: bool) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_submit_game")
    }
    fn scorpio_third_exchange_pet(&mut self) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_exchange_pet")
    }
    fn scorpio_third_buy_guess_count(&mut self, _count: i64) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_buy_guess_count")
    }
    fn scorpio_third_buy_red_sand(&mut self, _count: i64) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_buy_red_sand")
    }
}
