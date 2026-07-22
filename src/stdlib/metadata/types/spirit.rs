use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "TalentRefreshResult" => (
            "Talent refresh result.",
            vec![
                field("position", "int", "Bag position."),
                field(
                    "pa_ability_old",
                    "int",
                    "Physical attack ability before refresh.",
                ),
                field(
                    "pd_ability_old",
                    "int",
                    "Physical defense ability before refresh.",
                ),
                field(
                    "ma_ability_old",
                    "int",
                    "Magic attack ability before refresh.",
                ),
                field(
                    "md_ability_old",
                    "int",
                    "Magic defense ability before refresh.",
                ),
                field("sp_ability_old", "int", "Speed ability before refresh."),
                field("hp_ability_old", "int", "HP ability before refresh."),
                field(
                    "pa_ability_new",
                    "int",
                    "Physical attack ability after refresh.",
                ),
                field(
                    "pd_ability_new",
                    "int",
                    "Physical defense ability after refresh.",
                ),
                field(
                    "ma_ability_new",
                    "int",
                    "Magic attack ability after refresh.",
                ),
                field(
                    "md_ability_new",
                    "int",
                    "Magic defense ability after refresh.",
                ),
                field("sp_ability_new", "int", "Speed ability after refresh."),
                field("hp_ability_new", "int", "HP ability after refresh."),
                field(
                    "pa_talent_old",
                    "int",
                    "Physical attack talent before refresh.",
                ),
                field(
                    "pd_talent_old",
                    "int",
                    "Physical defense talent before refresh.",
                ),
                field(
                    "ma_talent_old",
                    "int",
                    "Magic attack talent before refresh.",
                ),
                field(
                    "md_talent_old",
                    "int",
                    "Magic defense talent before refresh.",
                ),
                field("sp_talent_old", "int", "Speed talent before refresh."),
                field("hp_talent_old", "int", "HP talent before refresh."),
                field(
                    "pa_talent_new",
                    "int",
                    "Physical attack talent after refresh.",
                ),
                field(
                    "pd_talent_new",
                    "int",
                    "Physical defense talent after refresh.",
                ),
                field("ma_talent_new", "int", "Magic attack talent after refresh."),
                field(
                    "md_talent_new",
                    "int",
                    "Magic defense talent after refresh.",
                ),
                field("sp_talent_new", "int", "Speed talent after refresh."),
                field("hp_talent_new", "int", "HP talent after refresh."),
            ],
        ),
        "BloodGiftInfo" => (
            "Blood gift information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field("position", "int", "Bag position."),
                field(
                    "equipped_index",
                    "int",
                    "Currently equipped blood gift index.",
                ),
                field(
                    "options",
                    "BloodGiftOption[]",
                    "Available blood gift options.",
                ),
            ],
        ),
        "AmendNatureInfo" => (
            "Nature amendment information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field(
                    "eligible_spirit_ids",
                    "int[]",
                    "Spirit IDs eligible for nature amendment.",
                ),
                field(
                    "candidates",
                    "AmendNatureCandidate[]",
                    "Candidate spirits for nature amendment.",
                ),
                field("new_personality", "int", "New personality enum value."),
                field("new_personality_name", "string", "New personality name."),
            ],
        ),
        "SpiritBagInfo" => (
            "Spirit bag information.",
            vec![field(
                "spirits",
                "SpiritInfo[]",
                "Spirits currently in the bag.",
            )],
        ),
        "SpiritEquipmentInfo" => (
            "Spirit equipment instance information.",
            vec![
                field("server_id", "int", "Equipment server ID."),
                field("catch_time", "int", "Equipment catch time."),
                field("base_attr", "int", "Base attribute ID."),
                field("base_value", "int", "Base attribute value."),
                field("special_attr", "int", "Special attribute ID."),
                field("special_value", "int", "Special attribute value."),
                field(
                    "spirit_id",
                    "RocoOptionalI64",
                    "Equipped spirit ID; missing when the equipment has no owner.",
                ),
                field(
                    "spirit_catch_time",
                    "RocoOptionalI64",
                    "Equipped spirit catch time; missing when the equipment has no owner.",
                ),
            ],
        ),
        "SpiritEquipmentBagInfo" => (
            "Spirit equipment bag information.",
            vec![
                field("equipment_count", "int", "Number of equipment items."),
                field("all_num", "int", "Capacity or total count."),
                field("need", "int", "Required value for the operation."),
                field("equipments", "SpiritEquipmentInfo[]", "Equipment list."),
            ],
        ),
        _ => return None,
    })
}
