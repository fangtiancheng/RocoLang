//! Standard library trait and namespace registration modules.

use crate::error::{Result, RocoError};
use crate::types::*;

pub mod combat;
pub mod dark_city;
pub mod game;
pub mod lookup;
pub mod manor;
pub mod mountain_sea;
pub mod mystery_fusion;
pub mod news;
pub mod news_times;
pub mod profile;
pub mod role;
pub mod scene;
pub mod sentinel_intelligence;
pub mod session;
pub mod spirit;
pub mod star_tower;
pub mod system;
pub mod treasure_realm;
pub mod util;

fn unsupported<T>(name: &str) -> Result<T> {
    Err(RocoError::StdLibError(format!(
        "{name} unsupported by this runtime"
    )))
}

/// Native APIs exposed to Rhai scripts.
///
/// Convention: operation-style `try_*` APIs return `ActionResult`
/// (`ok/code/message`) instead of raising expected business failures. Plain
/// APIs may raise or return their domain result directly. Query APIs should not
/// use `try_*` unless they also follow the `ActionResult` convention.
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

    fn list_storage_spirits(&mut self) -> Result<Vec<StorageSpiritInfo>> {
        unsupported("spirit::list_storage_spirits")
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
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
    }

    fn try_store_spirit(&mut self, position: i64) -> Result<ActionResult> {
        match self.store_spirit(position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("store_spirit returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
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

    fn try_restore_spirit(&mut self, spirit_id: i64, position: i64) -> Result<ActionResult> {
        match self.restore_spirit(spirit_id, position) {
            Ok(true) => Ok(ActionResult::ok()),
            Ok(false) => Ok(ActionResult::failed("restore_spirit returned false")),
            Err(error) => Ok(ActionResult::failed(error.to_string())),
        }
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

    fn get_blood_gift_info(&mut self, _position: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::get_blood_gift_info")
    }

    fn awaken_blood_gift(&mut self, _position: i64, _blood_index: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::awaken_blood_gift")
    }

    fn equip_blood_gift(&mut self, _position: i64, _blood_index: i64) -> Result<BloodGiftInfo> {
        unsupported("spirit::equip_blood_gift")
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

    fn get_combat_state(&mut self) -> Result<CombatState> {
        unsupported("combat::get_combat_state")
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

    fn manor_get_ground_info(&mut self) -> Result<ManorInfo> {
        unsupported("manor::get_ground_info")
    }

    fn manor_get_seed_bag(&mut self) -> Result<Vec<ManorItemCount>> {
        unsupported("manor::get_seed_bag")
    }

    fn manor_sow(&mut self, _seed_id: i64, _ground_id: i64) -> Result<ManorSowResult> {
        unsupported("manor::sow")
    }

    fn manor_reap(&mut self, _ground_id: i64) -> Result<ManorReapResult> {
        unsupported("manor::reap")
    }

    fn manor_uproot(&mut self, _ground_id: i64) -> Result<ManorUprootResult> {
        unsupported("manor::uproot")
    }

    fn manor_weed(&mut self, _ground_id: i64, _weed_type: i64) -> Result<ManorWeedResult> {
        unsupported("manor::weed")
    }

    fn manor_use_fertilizer(
        &mut self,
        _ground_id: i64,
        _fertilizer_item_id: i64,
    ) -> Result<ManorFertilizerResult> {
        unsupported("manor::use_fertilizer")
    }

    fn news_times_query_reports(&mut self) -> Result<NewsTimesReportsResult> {
        unsupported("news_times::query_reports")
    }

    fn news_query_reports(&mut self) -> Result<NewsTimesReportsResult> {
        self.news_times_query_reports()
    }

    fn news_query_active_ids(&mut self) -> Result<Vec<i64>> {
        unsupported("news::query_active_ids")
    }

    fn news_query_active_items(&mut self) -> Result<Vec<NewsActiveItem>> {
        unsupported("news::query_active_items")
    }

    fn news_list_active_config_items(&mut self) -> Result<Vec<NewsActiveItem>> {
        unsupported("news::list_active_config_items")
    }

    fn star_tower_query(&mut self) -> Result<StarTowerInfo> {
        unsupported("star_tower::query")
    }

    fn star_tower_settle_floor_fight(
        &mut self,
        _storey_index: i64,
        _node_index: i64,
    ) -> Result<StarTowerInfo> {
        unsupported("star_tower::settle_floor_fight")
    }

    fn star_tower_get_floor_award(&mut self, _storey_index: i64) -> Result<StarTowerInfo> {
        unsupported("star_tower::get_floor_award")
    }

    fn star_tower_quick_fight(
        &mut self,
        _storey: i64,
        _storey1: i64,
        _sell: bool,
    ) -> Result<StarTowerInfo> {
        unsupported("star_tower::quick_fight")
    }

    fn star_tower_toggle_auto_sell(&mut self) -> Result<StarTowerInfo> {
        unsupported("star_tower::toggle_auto_sell")
    }

    fn star_tower_settle_top_boss_fight(&mut self) -> Result<StarTowerInfo> {
        unsupported("star_tower::settle_top_boss_fight")
    }

    fn star_tower_get_top_reward(&mut self, _reward_index: i64) -> Result<StarTowerInfo> {
        unsupported("star_tower::get_top_reward")
    }

    fn star_tower_query_bag(&mut self) -> Result<StarTowerInfo> {
        unsupported("star_tower::query_bag")
    }

    fn star_tower_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<StarTowerInfo> {
        unsupported("star_tower::full_level")
    }

    fn sentinel_intelligence_query(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::query")
    }

    fn sentinel_intelligence_start_fight(
        &mut self,
        _boss_index: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::start_fight")
    }

    fn sentinel_intelligence_settle_fight(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::settle_fight")
    }

    fn sentinel_intelligence_refresh_mission(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::refresh_mission")
    }

    fn sentinel_intelligence_refresh_boss(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::refresh_boss")
    }

    fn sentinel_intelligence_refresh_exchange(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::refresh_exchange")
    }

    fn sentinel_intelligence_exchange_item(
        &mut self,
        _index: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::exchange_item")
    }

    fn sentinel_intelligence_exchange_spirit(
        &mut self,
        _index: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::exchange_spirit")
    }

    fn sentinel_intelligence_evolve_spirit(
        &mut self,
        _index: i64,
        _catch_time: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::evolve_spirit")
    }

    fn sentinel_intelligence_query_all(&mut self) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::query_all")
    }

    fn sentinel_intelligence_get_prize(
        &mut self,
        _boss_index: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::get_prize")
    }

    fn sentinel_intelligence_query_bag(
        &mut self,
        _evolve_spirit_id: i64,
    ) -> Result<SentinelIntelligenceInfo> {
        unsupported("sentinel_intelligence::query_bag")
    }

    fn mountain_sea_query(&mut self) -> Result<MountainSeaInfo> {
        unsupported("mountain_sea::query")
    }

    fn mountain_sea_open(&mut self) -> Result<MountainSeaInfo> {
        unsupported("mountain_sea::open")
    }

    fn mountain_sea_enter_boss(&mut self, _boss_index: i64) -> Result<MountainSeaInfo> {
        unsupported("mountain_sea::enter_boss")
    }

    fn mountain_sea_settle_fight(&mut self) -> Result<MountainSeaInfo> {
        unsupported("mountain_sea::settle_fight")
    }

    fn mountain_sea_summon(
        &mut self,
        _page_index: i64,
        _soul_type: i64,
        _soul_count: i64,
    ) -> Result<MountainSeaInfo> {
        unsupported("mountain_sea::summon")
    }

    fn dark_city_expedition_query(&mut self) -> Result<DarkCityExpeditionInfo> {
        unsupported("dark_city::expedition_query")
    }

    fn dark_city_expedition_start_fight(
        &mut self,
        _vip_boost: bool,
    ) -> Result<DarkCityExpeditionInfo> {
        unsupported("dark_city::expedition_start_fight")
    }

    fn dark_city_expedition_settle_fight(&mut self) -> Result<DarkCityExpeditionInfo> {
        unsupported("dark_city::expedition_settle_fight")
    }

    fn dark_city_expedition_set_vip_pass(
        &mut self,
        _enabled: bool,
    ) -> Result<DarkCityExpeditionInfo> {
        unsupported("dark_city::expedition_set_vip_pass")
    }

    fn dark_city_reputation_query_exchange(&mut self) -> Result<DarkCityReputationInfo> {
        unsupported("dark_city::reputation_query_exchange")
    }

    fn dark_city_reputation_exchange(
        &mut self,
        _index: i64,
        _count: i64,
    ) -> Result<DarkCityReputationInfo> {
        unsupported("dark_city::reputation_exchange")
    }

    fn mystery_fusion_query(&mut self) -> Result<MysteryFusionInfo> {
        unsupported("mystery_fusion::query")
    }

    fn mystery_fusion_prepare_battle(&mut self, _battle_index: i64) -> Result<MysteryFusionInfo> {
        unsupported("mystery_fusion::prepare_battle")
    }

    fn mystery_fusion_submit_battle(&mut self) -> Result<MysteryFusionInfo> {
        unsupported("mystery_fusion::submit_battle")
    }

    fn mystery_fusion_query_material_bag(
        &mut self,
        _spirit_id: i64,
    ) -> Result<MysteryFusionMaterialBag> {
        unsupported("mystery_fusion::query_material_bag")
    }

    fn mystery_fusion_claim_reward(&mut self) -> Result<MysteryFusionInfo> {
        unsupported("mystery_fusion::claim_reward")
    }

    fn mystery_fusion_fuse(
        &mut self,
        _recipe_index: i64,
        _material_bag_indexes: Vec<i64>,
        _personality: i64,
    ) -> Result<MysteryFusionInfo> {
        unsupported("mystery_fusion::fuse")
    }

    fn treasure_realm_query(&mut self) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::query")
    }

    fn treasure_realm_buy(&mut self, _index: i64) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::buy")
    }

    fn treasure_realm_boost_by_item(&mut self, _index: i64) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::boost_by_item")
    }

    fn treasure_realm_boost_by_vip(&mut self) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::boost_by_vip")
    }

    fn treasure_realm_start_battle(&mut self) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::start_battle")
    }

    fn treasure_realm_submit_battle(&mut self) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::submit_battle")
    }

    fn treasure_realm_get_gift(&mut self, _index: i64) -> Result<TreasureRealmInfo> {
        unsupported("treasure_realm::get_gift")
    }

    fn lookup_item_info(&mut self, _item_id: i64) -> Result<StaticItemInfo> {
        unsupported("lookup::lookup_item_info")
    }

    fn lookup_items_info(&mut self, _item_ids: Vec<i64>) -> Result<Vec<StaticItemInfo>> {
        unsupported("lookup::lookup_items_info")
    }

    fn lookup_strive_item_info(&mut self, _item_id: i64) -> Result<StaticStriveItemInfo> {
        unsupported("lookup::lookup_strive_item_info")
    }

    fn list_strive_item_infos(&mut self) -> Result<Vec<StaticStriveItemInfo>> {
        unsupported("lookup::list_strive_item_infos")
    }

    fn list_features_name(&mut self) -> Result<Vec<String>> {
        unsupported("lookup::list_features_name")
    }

    fn lookup_guardian_pet_property_info(
        &mut self,
        _level: i64,
    ) -> Result<StaticGuardianPetPropertyInfo> {
        unsupported("lookup::lookup_guardian_pet_property_info")
    }

    fn lookup_title_info(&mut self, _title_id: i64) -> Result<StaticTitleInfo> {
        unsupported("lookup::lookup_title_info")
    }

    fn lookup_magic_info(&mut self, _magic_id: i64) -> Result<StaticMagicInfo> {
        unsupported("lookup::lookup_magic_info")
    }

    fn lookup_plugin_info(&mut self, _plugin_name: &str) -> Result<StaticPluginInfo> {
        unsupported("lookup::lookup_plugin_info")
    }

    fn list_plugin_infos(&mut self) -> Result<Vec<StaticPluginInfo>> {
        unsupported("lookup::list_plugin_infos")
    }

    fn lookup_talent_info(&mut self, _talent_type: i64) -> Result<StaticTalentInfo> {
        unsupported("lookup::lookup_talent_info")
    }

    fn list_talent_infos(&mut self) -> Result<Vec<StaticTalentInfo>> {
        unsupported("lookup::list_talent_infos")
    }

    fn lookup_skill_info(&mut self, _skill_id: i64) -> Result<StaticSkillInfo> {
        unsupported("lookup::lookup_skill_info")
    }

    fn lookup_skills_info(&mut self, _skill_ids: Vec<i64>) -> Result<Vec<StaticSkillInfo>> {
        unsupported("lookup::lookup_skills_info")
    }

    fn lookup_spirit_info(&mut self, _spirit_id: i64) -> Result<StaticSpiritInfo> {
        unsupported("lookup::lookup_spirit_info")
    }

    fn lookup_spirits_info(&mut self, _spirit_ids: Vec<i64>) -> Result<Vec<StaticSpiritInfo>> {
        unsupported("lookup::lookup_spirits_info")
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
