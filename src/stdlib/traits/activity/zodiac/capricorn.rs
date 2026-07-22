use super::super::super::*;

pub trait RocoCapricornActivityStdLib: Send {
    fn capricorn_query_palace_notes(&mut self) -> Result<CapricornPalaceNotesInfo> {
        unsupported("capricorn::query_palace_notes")
    }
    fn capricorn_query_invite_list(&mut self) -> Result<CapricornInviteListInfo> {
        unsupported("capricorn::query_invite_list")
    }
    fn capricorn_invite_player(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::invite_player")
    }
    fn capricorn_cancel_invite(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::cancel_invite")
    }
    fn capricorn_accept_invite(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::accept_invite")
    }
    fn capricorn_refuse_invite(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::refuse_invite")
    }
    fn capricorn_leave_team(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::leave_team")
    }
    fn capricorn_disband_team(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::disband_team")
    }
    fn capricorn_star_palace_summon(&mut self) -> Result<CapricornStarPalaceInfo> {
        unsupported("capricorn::star_palace_summon")
    }
    fn capricorn_star_palace_quick_summon(&mut self) -> Result<CapricornStarPalaceInfo> {
        unsupported("capricorn::star_palace_quick_summon")
    }
    fn capricorn_second_query(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_query")
    }
    fn capricorn_second_random_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_random_task")
    }
    fn capricorn_second_settle_battle_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_settle_battle_task")
    }
    fn capricorn_second_give_up_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_give_up_task")
    }
    fn capricorn_second_accept_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_accept_task")
    }
    fn capricorn_second_answer_quiz(&mut self, _answer_index: i64) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_answer_quiz")
    }
    fn capricorn_second_query_bag(&mut self, _kind: i64) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_query_bag")
    }
    fn capricorn_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_level_up")
    }
    fn capricorn_second_buy_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_buy_up")
    }
    fn capricorn_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_evolve")
    }
    fn capricorn_third_query(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_query")
    }
    fn capricorn_third_settle_battle(&mut self, _boss_index: i64) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_settle_battle")
    }
    fn capricorn_third_buy_star_item(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_star_item")
    }
    fn capricorn_third_buy_progress(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_progress")
    }
    fn capricorn_third_add_challenge_count(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_add_challenge_count")
    }
    fn capricorn_third_buy_star_num(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_star_num")
    }
    fn capricorn_third_query_bag(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_query_bag")
    }
    fn capricorn_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_evolve")
    }
}
