use super::super::*;

pub trait RocoTowerActivityStdLib: Send {
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
}
