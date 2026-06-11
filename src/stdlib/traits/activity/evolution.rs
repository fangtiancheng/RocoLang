use super::super::*;

pub trait RocoEvolutionActivityStdLib: Send {
    fn unicorn_query(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::query")
    }
    fn unicorn_summon(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::summon")
    }
    fn unicorn_settle_summon_fight(&mut self, _slot: i64) -> Result<UnicornInfo> {
        unsupported("unicorn::settle_summon_fight")
    }
    fn unicorn_harvest(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::harvest")
    }
    fn unicorn_submit_minigame(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::submit_minigame")
    }
    fn unicorn_start_cultivation(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::start_cultivation")
    }
    fn unicorn_submit_cultivation_task(&mut self, _task: i64) -> Result<UnicornInfo> {
        unsupported("unicorn::submit_cultivation_task")
    }
    fn unicorn_claim_cultivation_reward(&mut self) -> Result<UnicornInfo> {
        unsupported("unicorn::claim_cultivation_reward")
    }
    fn unicorn_query_evolution_bag(&mut self, _stage: i64) -> Result<UnicornInfo> {
        unsupported("unicorn::query_evolution_bag")
    }
    fn unicorn_evolve(&mut self, _stage: i64, _catch_time: i64) -> Result<UnicornInfo> {
        unsupported("unicorn::evolve")
    }
    fn unicorn_one_key_evolve(&mut self, _stage: i64, _catch_time: i64) -> Result<UnicornInfo> {
        unsupported("unicorn::one_key_evolve")
    }
    fn unicorn_exchange_skill_stone(
        &mut self,
        _skill: i64,
        _cost_kind: i64,
    ) -> Result<UnicornInfo> {
        unsupported("unicorn::exchange_skill_stone")
    }
    fn four_seasons_query(&mut self) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::query")
    }
    fn four_seasons_roll_dice(&mut self, _vip: bool) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::roll_dice")
    }
    fn four_seasons_submit_minigame(&mut self) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::submit_minigame")
    }
    fn four_seasons_upgrade_box(&mut self, _position: i64) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::upgrade_box")
    }
    fn four_seasons_use_tool(&mut self, _tool: i64) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::use_tool")
    }
    fn four_seasons_use_appointed_step_tool(&mut self, _step: i64) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::use_appointed_step_tool")
    }
    fn four_seasons_submit_event(&mut self) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::submit_event")
    }
    fn four_seasons_exchange_shop_item(&mut self, _slot: i64) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::exchange_shop_item")
    }
    fn four_seasons_claim_spirit_reward(&mut self, _index: i64) -> Result<FourSeasonsInfo> {
        unsupported("four_seasons::claim_spirit_reward")
    }
    fn diamond_tear_query(&mut self) -> Result<DiamondTearInfo> {
        unsupported("diamond_tear::query")
    }
    fn diamond_tear_buy_pet(&mut self) -> Result<DiamondTearInfo> {
        unsupported("diamond_tear::buy_pet")
    }
    fn diamond_tear_freeze(&mut self) -> Result<DiamondTearInfo> {
        unsupported("diamond_tear::freeze")
    }
    fn diamond_tear_claim_diamond(&mut self) -> Result<DiamondTearInfo> {
        unsupported("diamond_tear::claim_diamond")
    }
    fn ice_crystal_query(&mut self) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::query")
    }
    fn ice_crystal_charge_with_item(&mut self, _item: i64) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::charge_with_item")
    }
    fn ice_crystal_report_battle_win(&mut self) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::report_battle_win")
    }
    fn ice_crystal_condense_crystal(&mut self, _crystal: i64) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::condense_crystal")
    }
    fn ice_crystal_query_evolution_bag(&mut self, _target: i64) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::query_evolution_bag")
    }
    fn ice_crystal_evolve(&mut self, _target: i64, _catch_time: i64) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::evolve")
    }
    fn ice_crystal_one_key_evolve(
        &mut self,
        _target: i64,
        _catch_time: i64,
    ) -> Result<IceCrystalInfo> {
        unsupported("ice_crystal::one_key_evolve")
    }
    fn multi_evolution_fire_query_candidates(&mut self, _slot: i64) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::fire_query_candidates")
    }
    fn multi_evolution_fire_evolve(
        &mut self,
        _slot: i64,
        _spirit_id: i64,
        _catch_time: i64,
        _item_count: i64,
        _fire_score: i64,
    ) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::fire_evolve")
    }
    fn multi_evolution_fire_query_booster_item_count(&mut self) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::fire_query_booster_item_count")
    }
    fn multi_evolution_fire_claim_reward(&mut self) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::fire_claim_reward")
    }
    fn multi_evolution_fire_query_reward_available(&mut self) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::fire_query_reward_available")
    }
    fn multi_evolution_water_query_candidates(&mut self, _slot: i64) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::water_query_candidates")
    }
    fn multi_evolution_water_evolve(
        &mut self,
        _slot: i64,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::water_evolve")
    }
    fn multi_evolution_grass_query_candidates(&mut self, _slot: i64) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::grass_query_candidates")
    }
    fn multi_evolution_grass_first_evolve(
        &mut self,
        _slot: i64,
        _spirit_id: i64,
        _catch_time: i64,
        _sunlight: i64,
    ) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::grass_first_evolve")
    }
    fn multi_evolution_grass_second_evolve(
        &mut self,
        _slot: i64,
        _spirit_id: i64,
        _catch_time: i64,
        _sunlight: i64,
    ) -> Result<MultiEvolutionInfo> {
        unsupported("multi_evolution::grass_second_evolve")
    }
}
