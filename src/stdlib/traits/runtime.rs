use super::*;

/// Runtime APIs for scene, profile, game pause, mini-game, session state, and user info.
pub trait RocoRuntimeStdLib: Send {
    fn move_to_scene(&mut self, _scene_id: i64, _timeout_ms: i64) -> Result<i64> {
        unsupported("scene::move_to_scene")
    }

    fn try_move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<ActionResult> {
        match self.get_current_scene() {
            Ok(current_scene) if current_scene == scene_id => return Ok(ActionResult::ok()),
            Ok(_) => {}
            Err(error) => return Ok(ActionResult::failed_with_error(error)),
        }

        match self.move_to_scene(scene_id, timeout_ms) {
            Ok(confirmed_scene) if confirmed_scene == scene_id => Ok(ActionResult::ok()),
            Ok(confirmed_scene) => Ok(ActionResult::failed(format!(
                "server confirmed scene {}, expected {}",
                confirmed_scene, scene_id
            ))),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn get_current_scene(&mut self) -> Result<i64> {
        unsupported("scene::get_current_scene")
    }

    fn get_scene_spirits(&mut self) -> Result<Vec<SceneSpiritInfo>> {
        unsupported("scene::get_scene_spirits")
    }

    fn get_cached_scene_roles(&mut self) -> Result<Vec<SceneRoleInfo>> {
        unsupported("role::get_cached_scene_roles")
    }

    fn query_server_time(&mut self) -> Result<ServerTimeInfo> {
        unsupported("profile::query_server_time")
    }

    fn try_query_server_time(&mut self) -> Result<ServerTimeResult> {
        match self.query_server_time() {
            Ok(result) => Ok(ServerTimeResult::ok(result)),
            Err(error) => Ok(ServerTimeResult::failed_with_error(error)),
        }
    }

    fn get_pause(&mut self) -> Result<bool> {
        unsupported("game::get_pause")
    }

    fn set_pause(&mut self, _enabled: bool) -> Result<bool> {
        unsupported("game::set_pause")
    }

    fn start_mini_game(&mut self, _game_id: i64) -> Result<()> {
        unsupported("game::start_mini_game")
    }

    fn submit_mini_game_score(
        &mut self,
        _game_id: i64,
        _score: i64,
        _game_type: i64,
    ) -> Result<MiniGameSubmitResult> {
        unsupported("game::submit_mini_game_score")
    }

    fn try_submit_mini_game_score(
        &mut self,
        game_id: i64,
        score: i64,
        game_type: i64,
    ) -> Result<MiniGameSubmitTryResult> {
        match self.submit_mini_game_score(game_id, score, game_type) {
            Ok(result) if result.ok => Ok(MiniGameSubmitTryResult::ok(result)),
            Ok(result) => Ok(MiniGameSubmitTryResult {
                ok: false,
                code: result.code,
                message: result.message.clone(),
                error: None,
                result,
            }),
            Err(RocoError::NetworkError(error)) => Ok(
                MiniGameSubmitTryResult::network_error_with_error(RocoError::NetworkError(error)),
            ),
            Err(RocoError::TimeoutError(error)) => Ok(
                MiniGameSubmitTryResult::network_error_with_error(RocoError::TimeoutError(error)),
            ),
            Err(error) => Ok(MiniGameSubmitTryResult::failed_with_error(error)),
        }
    }

    fn try_set_pause(&mut self, enabled: bool) -> Result<ActionResult> {
        match self.get_pause() {
            Ok(current) if current == enabled => return Ok(ActionResult::ok()),
            Ok(_) => {}
            Err(_) => {}
        }

        match self.set_pause(enabled) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("set_pause returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn session_get_int(&mut self, _key: &str, default_value: i64) -> Result<i64> {
        Ok(default_value)
    }

    fn session_set_int(&mut self, _key: &str, _value: i64) -> Result<bool> {
        Ok(false)
    }

    fn session_get_string(&mut self, _key: &str, default_value: &str) -> Result<String> {
        Ok(default_value.to_string())
    }

    fn session_set_string(&mut self, _key: &str, _value: &str) -> Result<bool> {
        Ok(false)
    }

    fn session_get_bool(&mut self, _key: &str, default_value: bool) -> Result<bool> {
        Ok(default_value)
    }

    fn session_set_bool(&mut self, _key: &str, _value: bool) -> Result<bool> {
        Ok(false)
    }

    fn session_delete(&mut self, _key: &str) -> Result<bool> {
        Ok(false)
    }

    fn session_clear(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn session_list_keys(&mut self) -> Result<Vec<(String, String)>> {
        Ok(Vec::new())
    }

    fn memory_today(&mut self) -> Result<String> {
        unsupported("memory::today")
    }

    fn memory_daily_get_int(&mut self, _key: &str, _default_value: i64) -> Result<i64> {
        unsupported("memory::daily_get_int")
    }

    fn memory_daily_set_int(&mut self, _key: &str, _value: i64) -> Result<bool> {
        unsupported("memory::daily_set_int")
    }

    fn memory_daily_increment_int(&mut self, _key: &str, _delta: i64) -> Result<i64> {
        unsupported("memory::daily_increment_int")
    }

    fn memory_daily_get_string(&mut self, _key: &str, _default_value: &str) -> Result<String> {
        unsupported("memory::daily_get_string")
    }

    fn memory_daily_set_string(&mut self, _key: &str, _value: &str) -> Result<bool> {
        unsupported("memory::daily_set_string")
    }

    fn memory_daily_get_bool(&mut self, _key: &str, _default_value: bool) -> Result<bool> {
        unsupported("memory::daily_get_bool")
    }

    fn memory_daily_set_bool(&mut self, _key: &str, _value: bool) -> Result<bool> {
        unsupported("memory::daily_set_bool")
    }

    fn memory_daily_delete(&mut self, _key: &str) -> Result<bool> {
        unsupported("memory::daily_delete")
    }

    fn memory_daily_clear(&mut self) -> Result<bool> {
        unsupported("memory::daily_clear")
    }

    fn memory_daily_list_keys(&mut self) -> Result<Vec<(String, String)>> {
        unsupported("memory::daily_list_keys")
    }

    fn memory_daily_battle_observed_started(&mut self) -> Result<i64> {
        unsupported("memory::daily_battle_observed_started")
    }

    fn memory_daily_battle_observed_completed(&mut self) -> Result<i64> {
        unsupported("memory::daily_battle_observed_completed")
    }

    fn memory_daily_battle_tracking_since(&mut self) -> Result<i64> {
        unsupported("memory::daily_battle_tracking_since")
    }

    fn memory_daily_battle_limit_reached(&mut self) -> Result<bool> {
        unsupported("memory::daily_battle_limit_reached")
    }

    fn memory_daily_battle_limit(&mut self) -> Result<i64> {
        unsupported("memory::daily_battle_limit")
    }

    fn memory_daily_battle_limit_return_code(&mut self) -> Result<i64> {
        unsupported("memory::daily_battle_limit_return_code")
    }

    fn memory_daily_battle_limit_message(&mut self) -> Result<String> {
        unsupported("memory::daily_battle_limit_message")
    }

    fn is_in_combat(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn get_user_info(&mut self) -> Result<UserInfo> {
        unsupported("profile::get_user_info")
    }
}
