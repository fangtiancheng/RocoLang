use super::to_array;
use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        SpiritInfo,
        spirit_id,
        position,
        catch_time,
        name,
        level,
        personality,
        hp,
        max_hp
    );
    engine.register_get("skills", |value: &mut SpiritInfo| to_array(&value.skills));
    register_getters!(engine, SpiritSkillInfo, skill_id, pp, inherited);
    register_getters!(
        engine,
        SkillPoolSkillInfo,
        skill_id,
        pp,
        inherited,
        position
    );
    register_getters!(engine, SkillPoolInfo, spirit_id, position);
    engine.register_get("skills", |value: &mut SkillPoolInfo| {
        to_array(&value.skills)
    });
    register_getters!(
        engine,
        SkillSwitchResult,
        spirit_id,
        position,
        skill_slot,
        skill_id
    );
    register_getters!(engine, SkillStoneSkillInfo, skill_id, pp, inherited);
    register_getters!(
        engine,
        SkillStoneResult,
        ok,
        result_code,
        message,
        item_id,
        position,
        needs_replace
    );
    engine.register_get("old_skills", |value: &mut SkillStoneResult| {
        to_array(&value.old_skills)
    });
    engine.register_get("new_skills", |value: &mut SkillStoneResult| {
        to_array(&value.new_skills)
    });
    register_getters!(
        engine,
        StorageSpiritInfo,
        spirit_id,
        catch_time,
        storage_time,
        level,
        sex,
        skin_flag,
        talent_type,
        talent_level
    );
    register_getters!(
        engine,
        StorageSpiritDetailInfo,
        spirit_id,
        catch_time,
        storage_time,
        name,
        level,
        personality,
        hp,
        max_hp,
        pa,
        pd,
        ma,
        md,
        sp,
        hp_ability
    );
    engine.register_get("skills", |value: &mut StorageSpiritDetailInfo| {
        to_array(&value.skills)
    });
    register_getters!(engine, BagItemInfo, item_id, count);
    engine.register_get("spirits", |value: &mut SpiritBagInfo| {
        to_array(&value.spirits)
    });
    register_extra_rhai_getters(engine);
}

fn register_extra_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        TalentRefreshResult,
        position,
        pa_ability_old,
        pd_ability_old,
        ma_ability_old,
        md_ability_old,
        sp_ability_old,
        hp_ability_old,
        pa_ability_new,
        pd_ability_new,
        ma_ability_new,
        md_ability_new,
        sp_ability_new,
        hp_ability_new,
        pa_talent_old,
        pd_talent_old,
        ma_talent_old,
        md_talent_old,
        sp_talent_old,
        hp_talent_old,
        pa_talent_new,
        pd_talent_new,
        ma_talent_new,
        md_talent_new,
        sp_talent_new,
        hp_talent_new,
    );
    register_getters!(engine, BloodGiftItemRequirement, item_id, count, need);
    register_getters!(
        engine,
        BloodGiftOption,
        blood_index,
        talent_type,
        talent_name,
        talent_description,
        awakened,
    );
    engine.register_get("required_items", |value: &mut BloodGiftOption| {
        to_array(&value.required_items)
    });
    register_getters!(
        engine,
        BloodGiftInfo,
        result_code,
        message,
        position,
        equipped_index
    );
    engine.register_get("options", |value: &mut BloodGiftInfo| {
        to_array(&value.options)
    });
    register_getters!(
        engine,
        AmendNatureCandidate,
        spirit_id,
        catch_time,
        level,
        personality,
        personality_name,
        need_money,
    );
    register_getters!(
        engine,
        AmendNatureInfo,
        result_code,
        message,
        new_personality,
        new_personality_name,
    );
    engine.register_get("eligible_spirit_ids", |value: &mut AmendNatureInfo| {
        to_array(&value.eligible_spirit_ids)
    });
    engine.register_get("candidates", |value: &mut AmendNatureInfo| {
        to_array(&value.candidates)
    });
    register_getters!(
        engine,
        SpiritEquipmentInfo,
        server_id,
        catch_time,
        base_attr,
        base_value,
        special_attr,
        special_value,
        spirit_id,
        spirit_catch_time,
    );
    register_getters!(
        engine,
        SpiritEquipmentBagInfo,
        equipment_count,
        all_num,
        need
    );
    engine.register_get("equipments", |value: &mut SpiritEquipmentBagInfo| {
        to_array(&value.equipments)
    });
}
