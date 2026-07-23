use super::*;

pub trait RocoFriendStdLib: Send {
    fn friend_query_initial_state(&mut self) -> Result<i64> {
        unsupported("friend::query_initial_state")
    }

    fn friend_send_application(&mut self, _friend_uin: i64) -> Result<()> {
        unsupported("friend::send_application")
    }

    fn friend_handle_application(
        &mut self,
        _sender_uin: i64,
        _accept: bool,
    ) -> Result<FriendApplicationHandleResult> {
        unsupported("friend::handle_application")
    }

    fn friend_delete(&mut self, _friend_uin: i64) -> Result<()> {
        unsupported("friend::delete")
    }

    fn friend_send_chat(&mut self, _friend_uin: i64, _message: &str) -> Result<i64> {
        unsupported("friend::send_chat")
    }

    fn friend_query_online(&mut self) -> Result<Vec<FriendOnlineStatus>> {
        unsupported("friend::query_online")
    }

    fn friend_query_details(&mut self, _friend_uins: Vec<i64>) -> Result<Vec<FriendDetail>> {
        unsupported("friend::query_details")
    }
}
