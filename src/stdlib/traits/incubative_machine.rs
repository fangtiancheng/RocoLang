use super::*;

pub trait RocoIncubativeMachineStdLib: Send {
    fn incubative_machine_query_egg_list(
        &mut self,
        _egg_type: i64,
        _page_index: i64,
    ) -> Result<IncubativeMachineEggListResult> {
        unsupported("incubative_machine::query_egg_list")
    }
    fn incubative_machine_query_info(&mut self, _which: i64) -> Result<IncubativeMachineInfo> {
        unsupported("incubative_machine::query_info")
    }
    fn incubative_machine_begin(
        &mut self,
        _which: i64,
        _egg_type: i64,
        _egg_id: i64,
        _catch_time: i64,
        _egg_uin: i64,
    ) -> Result<IncubativeMachineIncubationResult> {
        unsupported("incubative_machine::begin")
    }
    fn incubative_machine_vip_speed_up(
        &mut self,
        _which: i64,
    ) -> Result<IncubativeMachineIncubationResult> {
        unsupported("incubative_machine::vip_speed_up")
    }
    fn incubative_machine_teach(
        &mut self,
        _which: i64,
    ) -> Result<IncubativeMachineIncubationResult> {
        unsupported("incubative_machine::teach")
    }
    fn incubative_machine_terminate(
        &mut self,
        _which: i64,
    ) -> Result<IncubativeMachineActionResult> {
        unsupported("incubative_machine::terminate")
    }
    fn incubative_machine_get_spirit(
        &mut self,
        _which: i64,
    ) -> Result<IncubativeMachineGetSpiritResult> {
        unsupported("incubative_machine::get_spirit")
    }
    fn incubative_machine_set_complete_guide(&mut self) -> Result<IncubativeMachineActionResult> {
        unsupported("incubative_machine::set_complete_guide")
    }
}
