use super::super::super::*;

pub trait RocoHomeActivityStdLib: Send {
    fn home_get_overview(&mut self, _area_id: i64) -> Result<HomeOverview> {
        unsupported("home::get_overview")
    }

    fn home_get_friend_list(&mut self) -> Result<Vec<HomeFriendSummary>> {
        unsupported("home::get_friend_list")
    }

    fn home_get_training_spirits(&mut self) -> Result<Vec<HomeTrainingSpirit>> {
        unsupported("home::get_training_spirits")
    }

    fn home_get_training_spirit_report(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<HomeTrainingSpiritReport> {
        unsupported("home::get_training_spirit_report")
    }

    fn home_take_training_spirit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<HomeTakeTrainingSpiritResult> {
        unsupported("home::take_training_spirit")
    }

    fn home_query_coach_spirits(&mut self, _refresh: bool) -> Result<HomeCoachSpiritList> {
        unsupported("home::query_coach_spirits")
    }
}
