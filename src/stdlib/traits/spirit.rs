use super::*;

/// Spirit, bag, lineup, talent, skill, and equipment APIs.
pub trait RocoSpiritStdLib: Send {
    fn fetch_spirit(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<bool> {
        unsupported("spirit::fetch_spirit")
    }

    fn list_storage_spirits(&mut self) -> Result<Vec<StorageSpiritInfo>> {
        unsupported("spirit::list_storage_spirits")
    }

    fn list_abandoned_storage_spirits(&mut self) -> Result<Vec<StorageSpiritInfo>> {
        unsupported("spirit::list_abandoned_storage_spirits")
    }

    fn get_storage_spirit_detail(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SpiritInfo> {
        unsupported("spirit::get_storage_spirit_detail")
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

    fn swap_spirits(&mut self, _first_position: i64, _second_position: i64) -> Result<bool> {
        unsupported("spirit::swap_spirits")
    }

    fn try_swap_spirits(
        &mut self,
        first_position: i64,
        second_position: i64,
    ) -> Result<ActionResult> {
        match self.swap_spirits(first_position, second_position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("swap_spirits returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn try_store_spirit(&mut self, position: i64) -> Result<ActionResult> {
        match self.store_spirit(position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("store_spirit returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
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
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn ladder_recover_spirits(&mut self) -> Result<bool> {
        unsupported("ladder::recover_spirits")
    }

    fn ladder_query_info(&mut self) -> Result<LadderInfo> {
        unsupported("ladder::query_info")
    }

    fn ladder_query_rank(&mut self) -> Result<LadderRankInfo> {
        unsupported("ladder::query_rank")
    }

    fn ladder_try_recover_spirits(&mut self) -> Result<ActionResult> {
        match self.ladder_recover_spirits() {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("recover_spirits returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn type_ladder_recover_spirits(&mut self) -> Result<bool> {
        unsupported("type_ladder::recover_spirits")
    }

    fn type_ladder_query_info(&mut self) -> Result<TypeLadderInfo> {
        unsupported("type_ladder::query_info")
    }

    fn type_ladder_query_rank(&mut self) -> Result<TypeLadderRankInfo> {
        unsupported("type_ladder::query_rank")
    }

    fn type_ladder_try_recover_spirits(&mut self) -> Result<ActionResult> {
        match self.type_ladder_recover_spirits() {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("recover_spirits returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
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

    fn try_restore_spirit(&mut self, spirit_id: i64, position: i64) -> Result<ActionResult> {
        match self.restore_spirit(spirit_id, position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("restore_spirit returned false")),
            Err(error) => Ok(ActionResult::failed_with_error(error)),
        }
    }

    fn use_talent_refresh_item(
        &mut self,
        _spirit_id: i64,
        _position: i64,
        _item_id: i64,
    ) -> Result<TalentRefreshResult> {
        unsupported("spirit::use_talent_refresh_item")
    }

    fn get_blood_gift_info(&mut self, _position: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::get_blood_gift_info")
    }

    fn awaken_blood_gift(&mut self, _position: i64, _blood_index: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::awaken_blood_gift")
    }

    fn equip_blood_gift(&mut self, _position: i64, _blood_index: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::equip_blood_gift")
    }

    fn amend_nature_query_eligible_spirit_ids(&mut self) -> Result<Vec<i64>> {
        unsupported("spirit::amend_nature_query_eligible_spirit_ids")
    }

    fn amend_nature_query_candidates(&mut self) -> Result<AmendNatureInfo> {
        unsupported("spirit::amend_nature_query_candidates")
    }

    fn random_amend_nature(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AmendNatureInfo> {
        unsupported("spirit::random_amend_nature")
    }

    fn choose_amend_nature(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
        _personality: i64,
    ) -> Result<AmendNatureInfo> {
        unsupported("spirit::choose_amend_nature")
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

    fn query_skill_pool(&mut self, _position: i64) -> Result<SkillPoolInfo> {
        unsupported("spirit::query_skill_pool")
    }

    fn add_skill_from_pool(&mut self, _position: i64, _skill_id: i64) -> Result<SkillSwitchResult> {
        unsupported("spirit::add_skill_from_pool")
    }

    fn switch_skill(
        &mut self,
        _position: i64,
        _skill_slot: i64,
        _skill_id: i64,
    ) -> Result<SkillSwitchResult> {
        unsupported("spirit::switch_skill")
    }

    fn use_skill_stone_preview(
        &mut self,
        _position: i64,
        _item_id: i64,
    ) -> Result<SkillStoneResult> {
        unsupported("spirit::use_skill_stone_preview")
    }

    fn use_skill_stone_apply(&mut self, _position: i64, _item_id: i64) -> Result<SkillStoneResult> {
        unsupported("spirit::use_skill_stone_apply")
    }

    fn use_skill_stone_replace(
        &mut self,
        _position: i64,
        _item_id: i64,
        _old_skill_id: i64,
        _new_skill_id: i64,
    ) -> Result<SkillStoneResult> {
        unsupported("spirit::use_skill_stone_replace")
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

    fn list_equipment_bag(&mut self) -> Result<SpiritEquipmentBagInfo> {
        unsupported("spirit::list_equipment_bag")
    }

    fn unequip_item(
        &mut self,
        _equipment_server_id: i64,
        _equipment_catch_time: i64,
        _spirit_id: i64,
        _spirit_catch_time: i64,
    ) -> Result<bool> {
        unsupported("spirit::unequip_item")
    }

    fn unequip_all_items(&mut self, _spirit_id: i64, _spirit_catch_time: i64) -> Result<bool> {
        unsupported("spirit::unequip_all_items")
    }
}
