use super::super::*;

pub trait RocoThreeStartersActivityStdLib: Send {
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
}
