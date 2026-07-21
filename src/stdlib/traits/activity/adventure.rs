use super::super::*;

pub trait RocoAdventureActivityStdLib: Send {
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
    fn summon_query(&mut self) -> Result<SummonInfo> {
        unsupported("summon::query")
    }
    fn summon_query_data(&mut self) -> Result<SummonInfo> {
        unsupported("summon::query_data")
    }
    fn summon_set_wish(&mut self, _pool_version: i64, _wish_index: i64) -> Result<SummonInfo> {
        unsupported("summon::set_wish")
    }
    fn summon_cancel_wish(&mut self, _pool_version: i64) -> Result<SummonInfo> {
        unsupported("summon::cancel_wish")
    }
    fn summon_draw(&mut self, _pool_version: i64, _draw_count: i64) -> Result<SummonInfo> {
        unsupported("summon::draw")
    }
    fn summon_exchange(
        &mut self,
        _exchange_kind: i64,
        _pool_version: i64,
        _item_index: i64,
        _count: i64,
    ) -> Result<SummonInfo> {
        unsupported("summon::exchange")
    }
    fn summon_query_record(&mut self) -> Result<SummonInfo> {
        unsupported("summon::query_record")
    }
    fn play_guide_week_task_query(&mut self) -> Result<WeekTaskInfo> {
        unsupported("play_guide::week_task_query")
    }
    fn play_guide_week_task_claim_task(&mut self, _index: i64) -> Result<WeekTaskInfo> {
        unsupported("play_guide::week_task_claim_task")
    }
    fn play_guide_week_task_exchange(
        &mut self,
        _exchange_type: i64,
        _index: i64,
    ) -> Result<WeekTaskInfo> {
        unsupported("play_guide::week_task_exchange")
    }
    fn play_guide_diamond_task_query(&mut self) -> Result<DiamondTaskInfo> {
        unsupported("play_guide::diamond_task_query")
    }
    fn play_guide_diamond_task_claim_reward(&mut self, _index: i64) -> Result<DiamondTaskInfo> {
        unsupported("play_guide::diamond_task_claim_reward")
    }
    fn play_guide_qq_game_hall_gift(&mut self) -> Result<QqGameHallGiftInfo> {
        unsupported("play_guide::qq_game_hall_gift")
    }
    fn jump_machine_query(&mut self) -> Result<JumpMachineInfo> {
        unsupported("jump_machine::query")
    }
    fn jump_machine_play(&mut self) -> Result<JumpMachineInfo> {
        unsupported("jump_machine::play")
    }
}
