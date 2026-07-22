use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        StaticItemInfo,
        id,
        name,
        description,
        unique,
        item_type,
        subtype,
        price,
        expire_time,
    );
    register_getters!(
        engine,
        StaticStriveItemInfo,
        id,
        name,
        item_type,
        ghp,
        gpa,
        gpd,
        gma,
        gmd,
        gsp,
        src,
    );

    register_getters!(
        engine,
        StaticGuardianPetPropertyInfo,
        level,
        phase,
        energy,
        attack,
        defend,
        magic_attack,
        magic_defend,
        need_level_to_next_phase,
    );
    register_getters!(engine, StaticTitleInfo, id, title_name);
    register_getters!(
        engine,
        StaticMagicInfo,
        id,
        name,
        item_id,
        target,
        magic_type,
        duration,
        action_type,
        app,
        description,
    );
    register_getters!(
        engine,
        StaticPluginInfo,
        name,
        label,
        domain,
        version,
        command_type,
        plugin_class,
        plugin_src,
        plugin_url,
    );
    register_getters!(
        engine,
        StaticSkillInfo,
        id,
        name,
        description,
        description2,
        power,
        pp_max,
        property,
        src,
        attack_type,
        speed,
        damage_type,
        catch_rate,
        super_form_id,
        super_form_src,
    );
    register_getters!(
        engine,
        StaticSpiritInfo,
        id,
        name,
        description,
        src,
        avatar,
        icon_src,
        preview_src,
        move_speed,
        height,
        weight,
        color,
        interest,
        habitat,
        catchrate,
        boss_phyle,
        boss_reward,
        scene_id,
        condition,
        require_level,
        wg,
        mg,
        mk,
        sm,
        sd,
        fy,
        reward,
        evolution_form_id,
        get_form,
        state,
        start_time,
        end_time,
        first_id,
        propo_level,
        is_in_book,
        skinnum,
        exp_type,
    );
    engine.register_get("features", |value: &mut StaticSpiritInfo| {
        to_array(&value.features)
    });
    engine.register_get("group", |value: &mut StaticSpiritInfo| {
        to_array(&value.group)
    });
    engine.register_get("evolution", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution)
    });
    engine.register_get("evolution_to_ids", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution_to_ids)
    });
    register_getters!(engine, StaticSpiritEvolutionEdge, target_id, kind);
    engine.register_get("evolution_edges", |value: &mut StaticSpiritInfo| {
        to_array(&value.evolution_edges)
    });
    register_getters!(engine, StaticSpiritInfoLookupResult, ok, code, message);
    engine.register_get("result", |value: &mut StaticSpiritInfoLookupResult| {
        value.result.clone()
    });
}
