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
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.move_to_scene(scene_id, timeout_ms) {
            Ok(confirmed_scene) if confirmed_scene == scene_id => Ok(ActionResult::ok()),
            Ok(confirmed_scene) => Ok(ActionResult::failed(format!(
                "server confirmed scene {}, expected {}",
                confirmed_scene, scene_id
            ))),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
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

    fn query_server_time(&mut self) -> Result<i64> {
        unsupported("profile::query_server_time")
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
                result,
            }),
            Err(RocoError::NetworkError(message)) | Err(RocoError::TimeoutError(message)) => {
                Ok(MiniGameSubmitTryResult::network_error(message))
            }
            Err(error) => Ok(MiniGameSubmitTryResult::failed(error.to_string())),
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
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn try_query_server_time(&mut self) -> Result<ActionResult> {
        match self.query_server_time() {
            Ok(stamp) => Ok(ActionResult {
                ok: true,
                code: 0,
                message: stamp.to_string(),
            }),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
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

    fn is_in_combat(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn get_user_info(&mut self) -> Result<UserInfo> {
        unsupported("profile::get_user_info")
    }
}
