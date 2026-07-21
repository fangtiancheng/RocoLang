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
