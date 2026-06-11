use super::*;

/// Combat action and combat state APIs.
pub trait RocoCombatStdLib: Send {
    fn get_combat_lineup(&mut self) -> Result<[Option<SpiritInfo>; 6]> {
        unsupported("combat::get_combat_lineup")
    }

    fn get_combat_state(&mut self) -> Result<CombatState> {
        unsupported("combat::get_combat_state")
    }

    fn get_action_snapshot(&mut self) -> Result<CombatActionSnapshot> {
        Ok(CombatActionSnapshot {
            is_finished: self.is_combat_finished()?,
            state: self.get_combat_state()?,
            actions: self.get_combat_actions()?,
        })
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

    fn try_use_skill_and_wait(&mut self, skill_id: i64) -> Result<CombatActionResult> {
        let action = self.try_use_skill(skill_id)?;
        if !action.ok {
            return Ok(if action.code == 1 {
                CombatActionResult::unavailable(action.message)
            } else {
                CombatActionResult::failed(action.message)
            });
        }
        let combat_finished = self.is_combat_finished()?;
        let next_action_ready = if combat_finished {
            true
        } else {
            self.wait_next_action()?
        };
        Ok(CombatActionResult::ok(combat_finished, next_action_ready))
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

    fn try_change_spirit_and_wait(&mut self, position: i64) -> Result<CombatActionResult> {
        let action = self.try_change_spirit(position)?;
        if !action.ok {
            return Ok(if action.code == 1 {
                CombatActionResult::unavailable(action.message)
            } else {
                CombatActionResult::failed(action.message)
            });
        }
        let combat_finished = self.is_combat_finished()?;
        let next_action_ready = if combat_finished {
            true
        } else {
            self.wait_next_action()?
        };
        Ok(CombatActionResult::ok(combat_finished, next_action_ready))
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

    fn try_combat_escape_and_wait(&mut self) -> Result<CombatActionResult> {
        let action = self.try_combat_escape()?;
        if !action.ok {
            return Ok(if action.code == 1 {
                CombatActionResult::unavailable(action.message)
            } else {
                CombatActionResult::failed(action.message)
            });
        }
        let combat_finished = self.is_combat_finished()?;
        let next_action_ready = if combat_finished {
            true
        } else {
            self.wait_next_action()?
        };
        Ok(CombatActionResult::ok(combat_finished, next_action_ready))
    }

    fn wait_round_end(&mut self) -> Result<RoundResult> {
        unsupported("combat::wait_round_end")
    }

    fn wait_next_action(&mut self) -> Result<bool> {
        unsupported("combat::wait_next_action")
    }

    fn get_battle_result(&mut self) -> Result<BattleResult> {
        unsupported("combat::get_battle_result")
    }

    fn try_get_battle_result(&mut self) -> Result<BattleResultQueryResult> {
        match self.get_battle_result() {
            Ok(result) => Ok(BattleResultQueryResult::ok(result)),
            Err(error) => Ok(BattleResultQueryResult::unavailable(error.to_string())),
        }
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
}
