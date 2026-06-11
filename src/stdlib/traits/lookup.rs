use super::*;

/// Static data lookup APIs.
pub trait RocoLookupStdLib: Send {
    fn lookup_item_info(&mut self, _item_id: i64) -> Result<StaticItemInfo> {
        unsupported("lookup::lookup_item_info")
    }

    fn lookup_items_info(&mut self, _item_ids: Vec<i64>) -> Result<Vec<StaticItemInfo>> {
        unsupported("lookup::lookup_items_info")
    }

    fn lookup_strive_item_info(&mut self, _item_id: i64) -> Result<StaticStriveItemInfo> {
        unsupported("lookup::lookup_strive_item_info")
    }

    fn list_strive_item_infos(&mut self) -> Result<Vec<StaticStriveItemInfo>> {
        unsupported("lookup::list_strive_item_infos")
    }

    fn list_features_name(&mut self) -> Result<Vec<String>> {
        unsupported("lookup::list_features_name")
    }

    fn lookup_guardian_pet_property_info(
        &mut self,
        _level: i64,
    ) -> Result<StaticGuardianPetPropertyInfo> {
        unsupported("lookup::lookup_guardian_pet_property_info")
    }

    fn lookup_title_info(&mut self, _title_id: i64) -> Result<StaticTitleInfo> {
        unsupported("lookup::lookup_title_info")
    }

    fn lookup_magic_info(&mut self, _magic_id: i64) -> Result<StaticMagicInfo> {
        unsupported("lookup::lookup_magic_info")
    }

    fn lookup_plugin_info(&mut self, _plugin_name: &str) -> Result<StaticPluginInfo> {
        unsupported("lookup::lookup_plugin_info")
    }

    fn list_plugin_infos(&mut self) -> Result<Vec<StaticPluginInfo>> {
        unsupported("lookup::list_plugin_infos")
    }

    fn get_ladder_match_config(&mut self) -> Result<LadderMatchConfig> {
        unsupported("lookup::get_ladder_match_config")
    }

    fn lookup_talent_info(&mut self, _talent_type: i64) -> Result<StaticTalentInfo> {
        unsupported("lookup::lookup_talent_info")
    }

    fn list_talent_infos(&mut self) -> Result<Vec<StaticTalentInfo>> {
        unsupported("lookup::list_talent_infos")
    }

    fn lookup_skill_info(&mut self, _skill_id: i64) -> Result<StaticSkillInfo> {
        unsupported("lookup::lookup_skill_info")
    }

    fn lookup_skills_info(&mut self, _skill_ids: Vec<i64>) -> Result<Vec<StaticSkillInfo>> {
        unsupported("lookup::lookup_skills_info")
    }

    fn lookup_spirit_info(&mut self, _spirit_id: i64) -> Result<StaticSpiritInfo> {
        unsupported("lookup::lookup_spirit_info")
    }

    fn lookup_spirits_info(&mut self, _spirit_ids: Vec<i64>) -> Result<Vec<StaticSpiritInfo>> {
        unsupported("lookup::lookup_spirits_info")
    }
}
