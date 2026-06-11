use super::super::*;

pub trait RocoGeminiActivityStdLib: Send {
    fn gemini_first_query(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_query")
    }
    fn gemini_first_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_upgrade")
    }
    fn gemini_first_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_upgrade_to_100")
    }
    fn gemini_first_buy_ingredient(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_buy_ingredient")
    }
    fn gemini_first_buy_evolve_access(&mut self, _catch_time: i64) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_buy_evolve_access")
    }
    fn gemini_first_query_bag(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_query_bag")
    }
    fn gemini_second_query(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_query")
    }
    fn gemini_second_submit(&mut self, _kind: i64) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_submit")
    }
    fn gemini_second_get_gift(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_get_gift")
    }
    fn gemini_second_add_score(&mut self, _kind: i64, _score: i64) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_add_score")
    }
    fn gemini_second_buy(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_buy")
    }
    fn gemini_third_query(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_query")
    }
    fn gemini_third_settle_combat(&mut self, _side: i64, _index: i64) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_settle_combat")
    }
    fn gemini_third_submit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_submit")
    }
    fn gemini_third_submit_without_spirit(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_submit_without_spirit")
    }
    fn gemini_third_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_level")
    }
    fn gemini_third_buy_score(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_score")
    }
    fn gemini_third_query_bag(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_query_bag")
    }
    fn gemini_third_buy_challenge_count(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_challenge_count")
    }
    fn gemini_third_buy_score_by_index(
        &mut self,
        _side: i64,
        _index: i64,
        _score: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_score_by_index")
    }
}
