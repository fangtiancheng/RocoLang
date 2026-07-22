use super::super::to_array;
use crate::types::*;
use ::rhai::Engine;

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
