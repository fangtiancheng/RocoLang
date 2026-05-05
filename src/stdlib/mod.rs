//! Standard library trait and namespace registration modules.

use crate::error::{Result, RocoError};
use crate::types::*;

pub mod combat;
pub mod game;
pub mod lookup;
pub mod profile;
pub mod scene;
pub mod session;
pub mod spirit;
pub mod system;
pub mod util;

fn unsupported<T>(name: &str) -> Result<T> {
    Err(RocoError::StdLibError(format!(
        "{name} unsupported by this runtime"
    )))
}

pub trait RocoStdLib: Send {
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

    fn query_server_time(&mut self) -> Result<i64> {
        unsupported("profile::query_server_time")
    }

    fn get_pause(&mut self) -> Result<bool> {
        unsupported("game::get_pause")
    }

    fn set_pause(&mut self, _enabled: bool) -> Result<bool> {
        unsupported("game::set_pause")
    }

    fn try_set_pause(&mut self, enabled: bool) -> Result<ActionResult> {
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

    fn fetch_spirit(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<bool> {
        unsupported("spirit::fetch_spirit")
    }

    fn start_combat(
        &mut self,
        _server_type: i64,
        _combat_type: i64,
        _rival_id: i64,
        _catch_time: i64,
    ) -> Result<bool> {
        unsupported("combat::start_combat")
    }

    fn clear_lineup(&mut self) -> Result<bool> {
        unsupported("spirit::clear_lineup")
    }

    fn store_spirit(&mut self, _position: i64) -> Result<bool> {
        unsupported("spirit::store_spirit")
    }

    fn get_spirit_bag(&mut self) -> Result<SpiritBagInfo> {
        unsupported("spirit::get_spirit_bag")
    }

    fn get_bag_items(&mut self) -> Result<Vec<BagItemInfo>> {
        Ok(Vec::new())
    }

    fn take_pushed_drops(&mut self) -> Result<Vec<BagItemInfo>> {
        Ok(Vec::new())
    }

    fn recover_all_spirits(&mut self) -> Result<bool> {
        unsupported("spirit::recover_all_spirits")
    }

    fn try_recover_all_spirits(&mut self) -> Result<ActionResult> {
        match self.recover_all_spirits() {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("recover_all_spirits returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn use_spirit_item(
        &mut self,
        _spirit_id: i64,
        _position: i64,
        _item_id: i64,
        _count: i64,
    ) -> Result<bool> {
        unsupported("spirit::use_spirit_item")
    }

    fn restore_spirit(&mut self, _spirit_id: i64, _position: i64) -> Result<bool> {
        unsupported("spirit::restore_spirit")
    }

    fn use_talent_refresh_item(
        &mut self,
        _spirit_id: i64,
        _position: i64,
        _item_id: i64,
        _count: i64,
    ) -> Result<TalentRefreshResult> {
        unsupported("spirit::use_talent_refresh_item")
    }

    fn allocate_exp(&mut self, _position: i64, _exp: i64) -> Result<bool> {
        unsupported("spirit::allocate_exp")
    }

    #[allow(clippy::too_many_arguments)]
    fn save_strive_add(
        &mut self,
        _position: i64,
        _pa: i64,
        _pd: i64,
        _ma: i64,
        _md: i64,
        _sp: i64,
        _hp: i64,
    ) -> Result<bool> {
        unsupported("spirit::save_strive_add")
    }

    fn get_combat_lineup(&mut self) -> Result<[Option<SpiritInfo>; 6]> {
        unsupported("combat::get_combat_lineup")
    }

    fn learn_skill(&mut self, _position: i64, _skill_id: i64) -> Result<bool> {
        unsupported("spirit::learn_skill")
    }

    fn get_skills(&mut self, _position: i64) -> Result<[Option<SkillInfo>; 4]> {
        unsupported("spirit::get_skills")
    }

    fn equip_item(
        &mut self,
        _position: i64,
        _equipment_server_id: i64,
        _equipment_catch_time: i64,
        _spirit_id: i64,
        _spirit_catch_time: i64,
    ) -> Result<bool> {
        unsupported("spirit::equip_item")
    }

    fn lookup_item_info(&mut self, _item_id: i64) -> Result<StaticItemInfo> {
        unsupported("lookup::lookup_item_info")
    }

    fn lookup_skill_info(&mut self, _skill_id: i64) -> Result<StaticSkillInfo> {
        unsupported("lookup::lookup_skill_info")
    }

    fn lookup_spirit_info(&mut self, _spirit_id: i64) -> Result<StaticSpiritInfo> {
        unsupported("lookup::lookup_spirit_info")
    }

    fn invite_pk(&mut self, _target_uin: i64) -> Result<BattleInfo> {
        unsupported("combat::invite_pk")
    }

    fn accept_pk(&mut self) -> Result<bool> {
        unsupported("combat::accept_pk")
    }

    fn reject_pk(&mut self) -> Result<bool> {
        unsupported("combat::reject_pk")
    }

    fn use_skill(&mut self, _skill_id: i64) -> Result<bool> {
        unsupported("combat::use_skill")
    }

    fn try_use_skill(&mut self, skill_id: i64) -> Result<ActionResult> {
        match self.can_use_skill(skill_id) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("skill unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.use_skill(skill_id) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("use_skill returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn use_item(&mut self, _item_id: i64) -> Result<bool> {
        unsupported("combat::use_item")
    }

    fn try_use_item(&mut self, item_id: i64) -> Result<ActionResult> {
        match self.can_use_item(item_id) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("item unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.use_item(item_id) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("use_item returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn change_spirit(&mut self, _position: i64) -> Result<bool> {
        unsupported("combat::change_spirit")
    }

    fn try_change_spirit(&mut self, position: i64) -> Result<ActionResult> {
        match self.can_change_to_spirit(position) {
            Ok(true) => {}
            Ok(false) => return Ok(ActionResult::unavailable("target spirit unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.change_spirit(position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("change_spirit returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn combat_escape(&mut self) -> Result<bool> {
        unsupported("combat::combat_escape")
    }

    fn try_combat_escape(&mut self) -> Result<ActionResult> {
        match self.get_combat_actions() {
            Ok(actions) if actions.can_escape => {}
            Ok(_) => return Ok(ActionResult::unavailable("combat escape unavailable")),
            Err(error) => return Ok(ActionResult::failed(error.to_string())),
        }

        match self.combat_escape() {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("combat_escape returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn wait_round_end(&mut self) -> Result<RoundResult> {
        unsupported("combat::wait_round_end")
    }

    fn get_battle_result(&mut self) -> Result<BattleResult> {
        unsupported("combat::get_battle_result")
    }

    fn get_combat_actions(&mut self) -> Result<CombatActions> {
        unsupported("combat::get_combat_actions")
    }

    fn can_use_skill(&mut self, _skill_id: i64) -> Result<bool> {
        unsupported("combat::can_use_skill")
    }

    fn can_use_item(&mut self, _item_id: i64) -> Result<bool> {
        unsupported("combat::can_use_item")
    }

    fn can_change_to_spirit(&mut self, _position: i64) -> Result<bool> {
        unsupported("combat::can_change_to_spirit")
    }

    fn can_capture(&mut self) -> Result<bool> {
        unsupported("combat::can_capture")
    }

    fn get_battle_history(&mut self) -> Result<String> {
        unsupported("combat::get_battle_history")
    }

    fn get_my_hp(&mut self) -> Result<i64> {
        unsupported("combat::get_my_hp")
    }

    fn get_my_max_hp(&mut self) -> Result<i64> {
        unsupported("combat::get_my_max_hp")
    }

    fn get_rival_hp(&mut self) -> Result<i64> {
        unsupported("combat::get_rival_hp")
    }

    fn get_rival_max_hp(&mut self) -> Result<i64> {
        unsupported("combat::get_rival_max_hp")
    }

    fn get_my_pp(&mut self, _slot: i64) -> Result<i64> {
        unsupported("combat::get_my_pp")
    }

    fn get_my_spirit_info(&mut self, _position: i64) -> Result<SpiritInfo> {
        unsupported("combat::get_my_spirit_info")
    }

    fn get_rival_spirit_info(&mut self) -> Result<SpiritInfo> {
        unsupported("combat::get_rival_spirit_info")
    }

    fn is_combat_finished(&mut self) -> Result<bool> {
        unsupported("combat::is_combat_finished")
    }

    fn get_current_round(&mut self) -> Result<i64> {
        unsupported("combat::get_current_round")
    }

    fn sleep(&mut self, _ms: i64) -> Result<()> {
        unsupported("system::sleep")
    }

    fn now_ms(&mut self) -> Result<i64> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| RocoError::StdLibError(error.to_string()))?;
        i64::try_from(now.as_millis())
            .map_err(|_| RocoError::StdLibError("current timestamp exceeds i64 range".to_string()))
    }

    fn sleep_until_ms(&mut self, target_ms: i64) -> Result<()> {
        let now = self.now_ms()?;
        if target_ms <= now {
            return Ok(());
        }
        self.sleep(target_ms - now)
    }

    fn format_time(&mut self, timestamp: i64) -> Result<String> {
        Ok(timestamp.to_string())
    }

    fn log(&mut self, _message: &str) -> Result<()> {
        unsupported("system::log")
    }

    fn assert(&mut self, condition: bool, message: &str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(RocoError::AssertionError(message.to_string()))
        }
    }
}
