use super::super::*;

pub trait RocoAlchemyActivityStdLib: Send {
    fn alchemy_furnace_monkey_cultivation_query(&mut self) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_query")
    }
    fn alchemy_furnace_monkey_cultivation_submit_default(
        &mut self,
    ) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_submit_default")
    }
    fn alchemy_furnace_monkey_cultivation_submit_pills(
        &mut self,
        _dragon_tiger: bool,
        _cultivate_origin: bool,
        _nine_turn: bool,
    ) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_submit_pills")
    }
    fn alchemy_furnace_monkey_cultivation_get_gift(&mut self) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_get_gift")
    }
    fn alchemy_furnace_monkey_evo_query(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_query")
    }
    fn alchemy_furnace_monkey_evo_report_fight(
        &mut self,
        _fight_type: i64,
    ) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_report_fight")
    }
    fn alchemy_furnace_monkey_evo_give_up(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_give_up")
    }
    fn alchemy_furnace_monkey_evo_submit_default(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_submit_default")
    }
    fn alchemy_furnace_monkey_evo_submit_pills(
        &mut self,
        _dragon_tiger: bool,
        _cultivate_origin: bool,
        _nine_turn: bool,
    ) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_submit_pills")
    }
    fn alchemy_furnace_monkey_evo_query_bag(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_query_bag")
    }
    fn alchemy_furnace_monkey_evo_evolve(&mut self, _catch_time: i64) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_evolve")
    }
    fn alchemy_furnace_monkey_evo_get_gift(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_get_gift")
    }
    fn alchemy_furnace_raging_fire_query(&mut self) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_query")
    }
    fn alchemy_furnace_raging_fire_submit_stone(&mut self, _count: i64) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_submit_stone")
    }
    fn alchemy_furnace_raging_fire_report_fight(&mut self, _target: i64) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_report_fight")
    }
    fn alchemy_furnace_raging_fire_buy(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_buy")
    }
    fn alchemy_furnace_raging_fire_query_bag(&mut self) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_query_bag")
    }
    fn alchemy_furnace_raging_fire_get_gift(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_get_gift")
    }
}
