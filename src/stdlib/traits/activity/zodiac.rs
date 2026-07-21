use super::super::*;

pub trait RocoZodiacActivityStdLib: Send {
    fn aries_first_query(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_query")
    }
    fn aries_first_start(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_start")
    }
    fn aries_first_buy_times(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_buy_times")
    }
    fn aries_first_dice(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_dice")
    }
    fn aries_first_settle_battle(&mut self, _battle_type: i64) -> Result<AriesFirstInfo> {
        unsupported("aries::first_settle_battle")
    }
    fn aries_first_query_bag(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_query_bag")
    }
    fn aries_first_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesFirstInfo> {
        unsupported("aries::first_level_up")
    }
    fn aries_first_evolve(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<AriesFirstInfo> {
        unsupported("aries::first_evolve")
    }
    fn aries_first_buy_direct(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_buy_direct")
    }
    fn aries_first_get_gold(&mut self) -> Result<AriesFirstInfo> {
        unsupported("aries::first_get_gold")
    }
    fn aries_second_query(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_query")
    }
    fn aries_second_submit_game(&mut self, _power: i64) -> Result<AriesSecondInfo> {
        unsupported("aries::second_submit_game")
    }
    fn aries_second_query_bag(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_query_bag")
    }
    fn aries_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesSecondInfo> {
        unsupported("aries::second_evolve")
    }
    fn aries_second_buy_direct(&mut self) -> Result<AriesSecondInfo> {
        unsupported("aries::second_buy_direct")
    }
    fn aries_second_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AriesSecondInfo> {
        unsupported("aries::second_buy_level")
    }
    fn aries_third_query_status(&mut self) -> Result<AriesThirdStatusInfo> {
        unsupported("aries::third_query_status")
    }
    fn aries_third_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<AriesThirdExchangeInfo> {
        unsupported("aries::third_exchange_item")
    }
    fn aries_third_buy_tail(&mut self, _count: i64) -> Result<AriesThirdInfo> {
        unsupported("aries::third_buy_tail")
    }
    fn aries_third_buy_wish(&mut self) -> Result<AriesThirdInfo> {
        unsupported("aries::third_buy_wish")
    }
    fn aries_third_exchange_pet(&mut self) -> Result<AriesThirdInfo> {
        unsupported("aries::third_exchange_pet")
    }
    fn libra_first_query(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_query")
    }
    fn libra_first_submit_game(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_submit_game")
    }
    fn libra_first_settle_fight(&mut self, _prop_index: i64) -> Result<LibraFirstInfo> {
        unsupported("libra::first_settle_fight")
    }
    fn libra_first_get_gift(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_get_gift")
    }
    fn libra_first_notify_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraFirstInfo> {
        unsupported("libra::first_notify_full_level")
    }
    fn libra_first_query_bag(&mut self) -> Result<LibraFirstInfo> {
        unsupported("libra::first_query_bag")
    }
    fn libra_first_advance(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<LibraFirstInfo> {
        unsupported("libra::first_advance")
    }
    fn libra_second_query(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_query")
    }
    fn libra_second_settle_fight(&mut self, _npc_index: i64) -> Result<LibraSecondInfo> {
        unsupported("libra::second_settle_fight")
    }
    fn libra_second_awaken(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_awaken")
    }
    fn libra_second_buy_challenge_count(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_buy_challenge_count")
    }
    fn libra_second_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraSecondInfo> {
        unsupported("libra::second_full_level")
    }
    fn libra_second_query_bag(&mut self) -> Result<LibraSecondInfo> {
        unsupported("libra::second_query_bag")
    }
    fn libra_second_evolution(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LibraSecondInfo> {
        unsupported("libra::second_evolution")
    }
    fn libra_third_query_status(&mut self) -> Result<LibraThirdStatusInfo> {
        unsupported("libra::third_query_status")
    }
    fn libra_third_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<LibraThirdExchangeInfo> {
        unsupported("libra::third_exchange_item")
    }
    fn libra_third_buy_tail(&mut self, _count: i64) -> Result<LibraThirdInfo> {
        unsupported("libra::third_buy_tail")
    }
    fn libra_third_buy_wish(&mut self) -> Result<LibraThirdInfo> {
        unsupported("libra::third_buy_wish")
    }
    fn libra_third_exchange_pet(&mut self) -> Result<LibraThirdInfo> {
        unsupported("libra::third_exchange_pet")
    }
    fn leo_first_query_status(&mut self) -> Result<LeoFirstStatusInfo> {
        unsupported("leo::first_query_status")
    }
    fn leo_first_exchange_item(&mut self, _exchange_position: i64) -> Result<LeoFirstExchangeInfo> {
        unsupported("leo::first_exchange_item")
    }
    fn leo_first_buy_tail(&mut self, _count: i64) -> Result<LeoFirstInfo> {
        unsupported("leo::first_buy_tail")
    }
    fn leo_first_buy_wish(&mut self) -> Result<LeoFirstInfo> {
        unsupported("leo::first_buy_wish")
    }
    fn leo_first_exchange_pet(&mut self) -> Result<LeoFirstInfo> {
        unsupported("leo::first_exchange_pet")
    }
    fn leo_second_query(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_query")
    }
    fn leo_second_settle_combat(&mut self, _hunt_index: i64) -> Result<LeoSecondInfo> {
        unsupported("leo::second_settle_combat")
    }
    fn leo_second_submit_onekey(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_submit_onekey")
    }
    fn leo_second_query_spirit(&mut self) -> Result<LeoSecondInfo> {
        unsupported("leo::second_query_spirit")
    }
    fn leo_second_submit_spirit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LeoSecondInfo> {
        unsupported("leo::second_submit_spirit")
    }
    fn leo_second_buy_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<LeoSecondInfo> {
        unsupported("leo::second_buy_full_level")
    }
    fn leo_third_query(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_query")
    }
    fn leo_third_submit_combat(
        &mut self,
        _challenge_index: i64,
        _win: bool,
    ) -> Result<LeoThirdInfo> {
        unsupported("leo::third_submit_combat")
    }
    fn leo_third_get_reward(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_get_reward")
    }
    fn leo_third_full_level(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_full_level")
    }
    fn leo_third_evolve(&mut self, _catch_time: i64) -> Result<LeoThirdInfo> {
        unsupported("leo::third_evolve")
    }
    fn leo_third_light_star(&mut self, _star_index: i64) -> Result<LeoThirdInfo> {
        unsupported("leo::third_light_star")
    }
    fn leo_third_query_bag(&mut self) -> Result<LeoThirdInfo> {
        unsupported("leo::third_query_bag")
    }
    fn cancer_sharp_scorpion_query(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_query")
    }
    fn cancer_sharp_scorpion_exchange_item(
        &mut self,
        _exc_pos: i64,
    ) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_exchange_item")
    }
    fn cancer_sharp_scorpion_buy_tail(&mut self, _num: i64) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_buy_tail")
    }
    fn cancer_sharp_scorpion_buy_wish(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_buy_wish")
    }
    fn cancer_sharp_scorpion_exc_pet(&mut self) -> Result<CancerSharpScorpionInfo> {
        unsupported("cancer::sharp_scorpion_exc_pet")
    }
    fn cancer_mend_shape_query(&mut self) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_query")
    }
    fn cancer_mend_shape_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_upgrade")
    }
    fn cancer_mend_shape_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_upgrade_to_100")
    }
    fn cancer_mend_shape_buy(&mut self, _buy_type: i64) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_buy")
    }
    fn cancer_mend_shape_buy_full(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerMendShapeInfo> {
        unsupported("cancer::mend_shape_buy_full")
    }
    fn cancer_mend_shape_query_bag(&mut self) -> Result<CancerMendShapeBagInfo> {
        unsupported("cancer::mend_shape_query_bag")
    }
    fn cancer_unseal_memories_query(&mut self) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_query")
    }
    fn cancer_unseal_memories_start_game(&mut self) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_start_game")
    }
    fn cancer_unseal_memories_commit(&mut self, _choice: i64) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_commit")
    }
    fn cancer_unseal_memories_bag_query(&mut self) -> Result<CancerUnsealMemoriesBagInfo> {
        unsupported("cancer::unseal_memories_bag_query")
    }
    fn cancer_unseal_memories_put_full(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CancerUnsealMemoriesBagInfo> {
        unsupported("cancer::unseal_memories_put_full")
    }
    fn cancer_unseal_memories_buy(&mut self, _buy_type: i64) -> Result<CancerUnsealMemoriesInfo> {
        unsupported("cancer::unseal_memories_buy")
    }
    fn virgo_serve_god_query(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_query")
    }
    fn virgo_serve_god_accept_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_accept_task")
    }
    fn virgo_serve_god_give_up_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_give_up_task")
    }
    fn virgo_serve_god_finish_task(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_finish_task")
    }
    fn virgo_serve_god_buy_unlock(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_buy_unlock")
    }
    fn virgo_serve_god_settle_boss_combat(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_settle_boss_combat")
    }
    fn virgo_serve_god_query_bag(&mut self) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_query_bag")
    }
    fn virgo_serve_god_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_upgrade_to_100")
    }
    fn virgo_serve_god_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoServeGodInfo> {
        unsupported("virgo::serve_god_upgrade")
    }
    fn virgo_find_halidom_query(&mut self, _altar: bool) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_query")
    }
    fn virgo_find_halidom_clean(&mut self, _relic_index: i64) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_clean")
    }
    fn virgo_find_halidom_list_pet(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_list_pet")
    }
    fn virgo_find_halidom_put_pet(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_put_pet")
    }
    fn virgo_find_halidom_buy_pass(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_pass")
    }
    fn virgo_find_halidom_buy_search_count(&mut self) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_search_count")
    }
    fn virgo_find_halidom_buy_top_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<VirgoFindHalidomInfo> {
        unsupported("virgo::find_halidom_buy_top_level")
    }
    fn virgo_bell_fox_query_status(&mut self) -> Result<VirgoBellFoxStatusInfo> {
        unsupported("virgo::bell_fox_query_status")
    }
    fn virgo_bell_fox_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<VirgoBellFoxExchangeInfo> {
        unsupported("virgo::bell_fox_exchange_item")
    }
    fn virgo_bell_fox_buy_tail(&mut self, _count: i64) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_buy_tail")
    }
    fn virgo_bell_fox_buy_wish(&mut self) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_buy_wish")
    }
    fn virgo_bell_fox_exchange_pet(&mut self) -> Result<VirgoBellFoxInfo> {
        unsupported("virgo::bell_fox_exchange_pet")
    }
    fn pisces_first_query(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_query")
    }
    fn pisces_first_submit(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_submit")
    }
    fn pisces_first_get_gift(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_get_gift")
    }
    fn pisces_first_exchange(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_exchange")
    }
    fn pisces_first_buy(&mut self) -> Result<PiscesFirstInfo> {
        unsupported("pisces::first_buy")
    }
    fn pisces_second_query(&mut self) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_query")
    }
    fn pisces_second_submit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_submit")
    }
    fn pisces_second_submit_without_spirit(&mut self) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_submit_without_spirit")
    }
    fn pisces_second_get_gift(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_get_gift")
    }
    fn pisces_second_settle_fight(&mut self, _fight_index: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_settle_fight")
    }
    fn pisces_second_repair(&mut self, _repair_index: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_repair")
    }
    fn pisces_second_view(&mut self, _kind: i64) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_view")
    }
    fn pisces_second_evolution(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesSecondInfo> {
        unsupported("pisces::second_evolution")
    }
    fn pisces_third_query(&mut self) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_query")
    }
    fn pisces_third_settle_fight(&mut self, _boss_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_settle_fight")
    }
    fn pisces_third_buy(
        &mut self,
        _kind: i64,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_buy")
    }
    fn pisces_third_full_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_full_level")
    }
    fn pisces_third_complete(&mut self, _level_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_complete")
    }
    fn pisces_third_get_item(&mut self, _reward_index: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_get_item")
    }
    fn pisces_third_query_bag(&mut self, _kind: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_query_bag")
    }
    fn pisces_third_up(&mut self, _spirit_id: i64, _catch_time: i64) -> Result<PiscesThirdInfo> {
        unsupported("pisces::third_up")
    }
    fn taurus_first_query(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_query")
    }
    fn taurus_first_get_leather(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_leather")
    }
    fn taurus_first_get_nail(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_nail")
    }
    fn taurus_first_get_ding(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_ding")
    }
    fn taurus_first_get_glue(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_get_glue")
    }
    fn taurus_first_mix(&mut self, _part_index: i64) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_mix")
    }
    fn taurus_first_buy_pet(&mut self) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_buy_pet")
    }
    fn taurus_first_buy_item(&mut self, _item_index: i64, _count: i64) -> Result<TaurusFirstInfo> {
        unsupported("taurus::first_buy_item")
    }
    fn taurus_second_query(&mut self) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_query")
    }
    fn taurus_second_query_bag(&mut self) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_query_bag")
    }
    fn taurus_second_submit_game(
        &mut self,
        _game_index: i64,
        _score: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_submit_game")
    }
    fn taurus_second_buy_game(&mut self, _game_index: i64) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_buy_game")
    }
    fn taurus_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_evolve")
    }
    fn taurus_second_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusSecondInfo> {
        unsupported("taurus::second_buy_level")
    }
    fn taurus_third_query(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_query")
    }
    fn taurus_third_settle_npc_fight(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_settle_npc_fight")
    }
    fn taurus_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_evolve")
    }
    fn taurus_third_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_buy_level")
    }
    fn taurus_third_buy_score(&mut self, _kind: i64) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_buy_score")
    }
    fn taurus_third_query_bag(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_query_bag")
    }
    fn taurus_third_get_task(&mut self) -> Result<TaurusThirdInfo> {
        unsupported("taurus::third_get_task")
    }
    fn three_starters_water_query(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_query")
    }
    fn three_starters_water_buy(&mut self, _catch_time: i64) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_buy")
    }
    fn three_starters_water_settle_fight(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_settle_fight")
    }
    fn three_starters_water_submit(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_submit")
    }
    fn three_starters_water_get_gift(&mut self, _catch_time: i64) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_get_gift")
    }
    fn three_starters_water_query_bag(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_query_bag")
    }
    fn three_starters_fire_query(&mut self) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_query")
    }
    fn three_starters_fire_buy(&mut self, _catch_time: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_buy")
    }
    fn three_starters_fire_start(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_start")
    }
    fn three_starters_fire_submit_battle(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_submit_battle")
    }
    fn three_starters_fire_submit_direct(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_submit_direct")
    }
    fn three_starters_fire_over(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_over")
    }
    fn three_starters_fire_get_gift(&mut self, _catch_time: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_get_gift")
    }
    fn three_starters_fire_query_bag(&mut self) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_query_bag")
    }
    fn three_starters_sun_query(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_query")
    }
    fn three_starters_sun_buy(&mut self, _catch_time: i64) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_buy")
    }
    fn three_starters_sun_start_fight(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_start_fight")
    }
    fn three_starters_sun_settle_fight(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_settle_fight")
    }
    fn three_starters_sun_start_collect(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_start_collect")
    }
    fn three_starters_sun_submit(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_submit")
    }
    fn three_starters_sun_get_gift(&mut self, _catch_time: i64) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_get_gift")
    }
    fn three_starters_sun_query_bag(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_query_bag")
    }
    fn gemini_first_query(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_query")
    }
    fn gemini_first_upgrade(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_upgrade")
    }
    fn gemini_first_upgrade_to_100(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_upgrade_to_100")
    }
    fn gemini_first_buy_ingredient(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_buy_ingredient")
    }
    fn gemini_first_buy_evolve_access(&mut self, _catch_time: i64) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_buy_evolve_access")
    }
    fn gemini_first_query_bag(&mut self) -> Result<GeminiFirstInfo> {
        unsupported("gemini::first_query_bag")
    }
    fn gemini_second_query(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_query")
    }
    fn gemini_second_submit(&mut self, _kind: i64) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_submit")
    }
    fn gemini_second_get_gift(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_get_gift")
    }
    fn gemini_second_add_score(&mut self, _kind: i64, _score: i64) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_add_score")
    }
    fn gemini_second_buy(&mut self) -> Result<GeminiSecondInfo> {
        unsupported("gemini::second_buy")
    }
    fn gemini_third_query(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_query")
    }
    fn gemini_third_settle_combat(&mut self, _side: i64, _index: i64) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_settle_combat")
    }
    fn gemini_third_submit(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_submit")
    }
    fn gemini_third_submit_without_spirit(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_submit_without_spirit")
    }
    fn gemini_third_buy_level(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_level")
    }
    fn gemini_third_buy_score(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_score")
    }
    fn gemini_third_query_bag(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_query_bag")
    }
    fn gemini_third_buy_challenge_count(&mut self) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_challenge_count")
    }
    fn gemini_third_buy_score_by_index(
        &mut self,
        _side: i64,
        _index: i64,
        _score: i64,
    ) -> Result<GeminiThirdInfo> {
        unsupported("gemini::third_buy_score_by_index")
    }
    fn sagittarius_first_query(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_query")
    }
    fn sagittarius_first_submit(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_submit")
    }
    fn sagittarius_first_query_scene(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_query_scene")
    }
    fn sagittarius_first_pick_shine(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_pick_shine")
    }
    fn sagittarius_first_add_map(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_add_map")
    }
    fn sagittarius_first_show_map(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_show_map")
    }
    fn sagittarius_first_add_speed(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_add_speed")
    }
    fn sagittarius_first_complete(&mut self, _index: i64) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_complete")
    }
    fn sagittarius_first_get_pet(&mut self) -> Result<SagittariusFirstInfo> {
        unsupported("sagittarius::first_get_pet")
    }
    fn sagittarius_second_query(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_query")
    }
    fn sagittarius_second_start(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_start")
    }
    fn sagittarius_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_level_up")
    }
    fn sagittarius_second_buy_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_buy_up")
    }
    fn sagittarius_second_random(&mut self) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_random")
    }
    fn sagittarius_second_task(&mut self, _task_type: i64) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_task")
    }
    fn sagittarius_second_query_bag(&mut self, _bag_type: i64) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_query_bag")
    }
    fn sagittarius_second_spirit_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusSecondInfo> {
        unsupported("sagittarius::second_spirit_up")
    }
    fn sagittarius_third_query(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_query")
    }
    fn sagittarius_third_settle_combat(
        &mut self,
        _boss_index: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_settle_combat")
    }
    fn sagittarius_third_buy_pet(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_pet")
    }
    fn sagittarius_third_buy_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_evolve")
    }
    fn sagittarius_third_add_challenge_count(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_add_challenge_count")
    }
    fn sagittarius_third_buy_star_num(
        &mut self,
        _score_index: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_buy_star_num")
    }
    fn sagittarius_third_query_bag(&mut self) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_query_bag")
    }
    fn sagittarius_third_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<SagittariusThirdInfo> {
        unsupported("sagittarius::third_up")
    }
    fn scorpio_first_query(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_query")
    }
    fn scorpio_first_submit_game(&mut self, _score: i64) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_submit_game")
    }
    fn scorpio_first_query_bag(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_query_bag")
    }
    fn scorpio_first_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_evolve")
    }
    fn scorpio_first_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_level_up")
    }
    fn scorpio_first_buy_direct(&mut self) -> Result<ScorpioFirstInfo> {
        unsupported("scorpio::first_buy_direct")
    }
    fn scorpio_second_query(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_query")
    }
    fn scorpio_second_settle(&mut self, _reward_source: i64) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_settle")
    }
    fn scorpio_second_query_bag(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_query_bag")
    }
    fn scorpio_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_evolve")
    }
    fn scorpio_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_level_up")
    }
    fn scorpio_second_buy_direct(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_buy_direct")
    }
    fn scorpio_second_buy_challenge_count(&mut self) -> Result<ScorpioSecondInfo> {
        unsupported("scorpio::second_buy_challenge_count")
    }
    fn scorpio_third_query(&mut self) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_query")
    }
    fn scorpio_third_submit_game(&mut self, _success: bool) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_submit_game")
    }
    fn scorpio_third_exchange_pet(&mut self) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_exchange_pet")
    }
    fn scorpio_third_buy_guess_count(&mut self, _count: i64) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_buy_guess_count")
    }
    fn scorpio_third_buy_red_sand(&mut self, _count: i64) -> Result<ScorpioThirdInfo> {
        unsupported("scorpio::third_buy_red_sand")
    }
    fn aquarius_first_query(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_query")
    }
    fn aquarius_first_settle_combat(&mut self, _boss_index: i64) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_settle_combat")
    }
    fn aquarius_first_buy_evolve_access(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_evolve_access")
    }
    fn aquarius_first_add_challenge_count(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_add_challenge_count")
    }
    fn aquarius_first_buy_star_num(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_star_num")
    }
    fn aquarius_first_query_bag(&mut self) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_query_bag")
    }
    fn aquarius_first_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_evolve")
    }
    fn aquarius_first_buy_direct(
        &mut self,
        _item_or_spirit_id: i64,
        _count: i64,
    ) -> Result<AquariusFirstInfo> {
        unsupported("aquarius::first_buy_direct")
    }
    fn aquarius_second_query_status(&mut self) -> Result<AquariusSecondStatusInfo> {
        unsupported("aquarius::second_query_status")
    }
    fn aquarius_second_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<AquariusSecondExchangeInfo> {
        unsupported("aquarius::second_exchange_item")
    }
    fn aquarius_second_query_diamond(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_query_diamond")
    }
    fn aquarius_second_combat_again(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_combat_again")
    }
    fn aquarius_second_buy_tail(&mut self, _count: i64) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_tail")
    }
    fn aquarius_second_buy_wish(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_wish")
    }
    fn aquarius_second_exchange_pet(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_exchange_pet")
    }
    fn aquarius_second_buy_spirit(&mut self) -> Result<AquariusSecondInfo> {
        unsupported("aquarius::second_buy_spirit")
    }
    fn aquarius_third_query(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_query")
    }
    fn aquarius_third_random(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_random")
    }
    fn aquarius_third_settle_combat(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_settle_combat")
    }
    fn aquarius_third_buy_level(&mut self) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_buy_level")
    }
    fn aquarius_third_buy_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_buy_evolve")
    }
    fn aquarius_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_evolve")
    }
    fn aquarius_third_query_bag(&mut self, _bag_type: i64) -> Result<AquariusThirdInfo> {
        unsupported("aquarius::third_query_bag")
    }
    fn capricorn_query_palace_notes(&mut self) -> Result<CapricornPalaceNotesInfo> {
        unsupported("capricorn::query_palace_notes")
    }
    fn capricorn_query_invite_list(&mut self) -> Result<CapricornInviteListInfo> {
        unsupported("capricorn::query_invite_list")
    }
    fn capricorn_invite_player(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::invite_player")
    }
    fn capricorn_cancel_invite(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::cancel_invite")
    }
    fn capricorn_accept_invite(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::accept_invite")
    }
    fn capricorn_refuse_invite(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::refuse_invite")
    }
    fn capricorn_leave_team(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::leave_team")
    }
    fn capricorn_disband_team(&mut self) -> Result<CapricornTeamOperationInfo> {
        unsupported("capricorn::disband_team")
    }
    fn capricorn_star_palace_summon(&mut self) -> Result<CapricornStarPalaceInfo> {
        unsupported("capricorn::star_palace_summon")
    }
    fn capricorn_star_palace_quick_summon(&mut self) -> Result<CapricornStarPalaceInfo> {
        unsupported("capricorn::star_palace_quick_summon")
    }
    fn capricorn_second_query(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_query")
    }
    fn capricorn_second_random_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_random_task")
    }
    fn capricorn_second_settle_battle_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_settle_battle_task")
    }
    fn capricorn_second_give_up_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_give_up_task")
    }
    fn capricorn_second_accept_task(&mut self) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_accept_task")
    }
    fn capricorn_second_answer_quiz(&mut self, _answer_index: i64) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_answer_quiz")
    }
    fn capricorn_second_query_bag(&mut self, _kind: i64) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_query_bag")
    }
    fn capricorn_second_level_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_level_up")
    }
    fn capricorn_second_buy_up(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_buy_up")
    }
    fn capricorn_second_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornSecondInfo> {
        unsupported("capricorn::second_evolve")
    }
    fn capricorn_third_query(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_query")
    }
    fn capricorn_third_settle_battle(&mut self, _boss_index: i64) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_settle_battle")
    }
    fn capricorn_third_buy_star_item(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_star_item")
    }
    fn capricorn_third_buy_progress(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_progress")
    }
    fn capricorn_third_add_challenge_count(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_add_challenge_count")
    }
    fn capricorn_third_buy_star_num(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_buy_star_num")
    }
    fn capricorn_third_query_bag(&mut self) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_query_bag")
    }
    fn capricorn_third_evolve(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<CapricornThirdInfo> {
        unsupported("capricorn::third_evolve")
    }
}
