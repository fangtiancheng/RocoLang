use super::*;

pub trait RocoRemoteStateStdLib: Send {
    fn load_remote_scene_data(&mut self, _scene_id: i64) -> Result<RemoteSceneData> {
        unsupported("remote_state::load_scene_data")
    }

    fn update_remote_scene_data(
        &mut self,
        _scene_id: i64,
        _values: rhai::Blob,
    ) -> Result<RemoteSceneData> {
        unsupported("remote_state::update_scene_data")
    }

    fn load_remote_npc_value(&mut self, _npc_id: i64) -> Result<i64> {
        unsupported("remote_state::load_npc_value")
    }

    fn update_remote_npc_value(&mut self, _npc_id: i64, _value: i64) -> Result<i64> {
        unsupported("remote_state::update_npc_value")
    }
}
