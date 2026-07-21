use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteSceneData {
    pub scene_id: i64,
    pub values: rhai::Blob,
}
