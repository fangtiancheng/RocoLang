use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionBattleInfo {
    pub index: i64,
    pub battle_id: i64,
    pub attr_types: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionRecipeInfo {
    pub index: i64,
    pub spirit_id: i64,
    pub energy_cost: i64,
    pub required_spirit_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionInfo {
    pub result_code: i64,
    pub message: String,
    pub times: i64,
    pub energy: i64,
    pub added_energy: i64,
    pub battles: Vec<MysteryFusionBattleInfo>,
    pub recipes: Vec<MysteryFusionRecipeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionMaterialCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub bag_index: i64,
    pub level: i64,
    pub personality: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysteryFusionMaterialBag {
    pub result_code: i64,
    pub message: String,
    pub candidates: Vec<MysteryFusionMaterialCandidate>,
}
