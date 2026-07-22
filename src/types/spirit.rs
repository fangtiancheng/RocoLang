use serde::{Deserialize, Serialize};

use super::RocoOptionalI64;

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
