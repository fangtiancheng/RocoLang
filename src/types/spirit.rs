use serde::{Deserialize, Serialize};

use super::{to_array, Engine, RocoOptionalI64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritInfo {
    pub spirit_id: i64,
    pub position: i64,
    pub catch_time: RocoOptionalI64,
    pub name: String,
    pub level: i64,
    pub personality: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPoolSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPoolInfo {
    pub spirit_id: i64,
    pub position: i64,
    pub skills: Vec<SkillPoolSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSwitchResult {
    pub spirit_id: i64,
    pub position: i64,
    pub skill_slot: i64,
    pub skill_id: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillStoneSkillInfo {
    pub skill_id: i64,
    pub pp: i64,
    pub inherited: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillStoneResult {
    pub ok: bool,
    pub result_code: i64,
    pub message: String,
    pub item_id: i64,
    pub position: i64,
    pub needs_replace: bool,
    pub old_skills: Vec<SkillStoneSkillInfo>,
    pub new_skills: Vec<SkillStoneSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpiritInfo {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub storage_time: i64,
    pub level: i64,
    pub sex: i64,
    pub skin_flag: i64,
    pub talent_type: i64,
    pub talent_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpiritDetailInfo {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub storage_time: i64,
    pub name: String,
    pub level: i64,
    pub personality: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub pa: i64,
    pub pd: i64,
    pub ma: i64,
    pub md: i64,
    pub sp: i64,
    pub hp_ability: i64,
    pub skills: Vec<SpiritSkillInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BagItemInfo {
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBagInfo {
    pub spirits: Vec<SpiritInfo>,
}

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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentRefreshResult {
    pub position: i64,
    pub pa_ability_old: i64,
    pub pd_ability_old: i64,
    pub ma_ability_old: i64,
    pub md_ability_old: i64,
    pub sp_ability_old: i64,
    pub hp_ability_old: i64,
    pub pa_ability_new: i64,
    pub pd_ability_new: i64,
    pub ma_ability_new: i64,
    pub md_ability_new: i64,
    pub sp_ability_new: i64,
    pub hp_ability_new: i64,
    pub pa_talent_old: i64,
    pub pd_talent_old: i64,
    pub ma_talent_old: i64,
    pub md_talent_old: i64,
    pub sp_talent_old: i64,
    pub hp_talent_old: i64,
    pub pa_talent_new: i64,
    pub pd_talent_new: i64,
    pub ma_talent_new: i64,
    pub md_talent_new: i64,
    pub sp_talent_new: i64,
    pub hp_talent_new: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftItemRequirement {
    pub item_id: i64,
    pub count: i64,
    pub need: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftOption {
    pub blood_index: i64,
    pub talent_type: i64,
    pub talent_name: String,
    pub talent_description: String,
    pub awakened: bool,
    pub required_items: Vec<BloodGiftItemRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGiftInfo {
    pub result_code: i64,
    pub message: String,
    pub position: i64,
    pub equipped_index: i64,
    pub options: Vec<BloodGiftOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendNatureCandidate {
    pub spirit_id: i64,
    pub catch_time: i64,
    pub level: i64,
    pub personality: i64,
    pub personality_name: String,
    pub need_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendNatureInfo {
    pub result_code: i64,
    pub message: String,
    pub eligible_spirit_ids: Vec<i64>,
    pub candidates: Vec<AmendNatureCandidate>,
    pub new_personality: i64,
    pub new_personality_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritEquipmentInfo {
    pub server_id: i64,
    pub catch_time: i64,
    pub base_attr: i64,
    pub base_value: i64,
    pub special_attr: i64,
    pub special_value: i64,
    pub spirit_id: RocoOptionalI64,
    pub spirit_catch_time: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritEquipmentBagInfo {
    pub equipment_count: i64,
    pub all_num: i64,
    pub need: i64,
    pub equipments: Vec<SpiritEquipmentInfo>,
}

/// Skill information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub skill_id: i64,
    pub skill_name: String,
    pub pp: i64,
    pub max_pp: i64,
}

pub(super) fn register_extra_rhai_getters(engine: &mut Engine) {
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
