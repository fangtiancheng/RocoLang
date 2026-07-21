use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};

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

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(engine, MysteryFusionBattleInfo, index, battle_id);
    engine.register_get("attr_types", |value: &mut MysteryFusionBattleInfo| {
        to_array(&value.attr_types)
    });
    register_getters!(
        engine,
        MysteryFusionRecipeInfo,
        index,
        spirit_id,
        energy_cost
    );
    engine.register_get(
        "required_spirit_ids",
        |value: &mut MysteryFusionRecipeInfo| to_array(&value.required_spirit_ids),
    );
    register_getters!(
        engine,
        MysteryFusionInfo,
        result_code,
        message,
        times,
        energy,
        added_energy,
    );
    engine.register_get("battles", |value: &mut MysteryFusionInfo| {
        to_array(&value.battles)
    });
    engine.register_get("recipes", |value: &mut MysteryFusionInfo| {
        to_array(&value.recipes)
    });

    register_getters!(
        engine,
        MysteryFusionMaterialCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        level,
        personality,
    );
    register_getters!(engine, MysteryFusionMaterialBag, result_code, message);
    engine.register_get("candidates", |value: &mut MysteryFusionMaterialBag| {
        to_array(&value.candidates)
    });
}
