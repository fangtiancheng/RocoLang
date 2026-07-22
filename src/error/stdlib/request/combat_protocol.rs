use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatIntentKind {
    UseSkill,
    ChangeSpirit,
    UseItem,
    Escape,
}

impl ScriptCombatIntentKind {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseSkill => "use_skill",
            Self::ChangeSpirit => "change_spirit",
            Self::UseItem => "use_item",
            Self::Escape => "escape",
        }
    }
}

impl fmt::Display for ScriptCombatIntentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolError {
    InvalidSpiritPosition {
        value: u8,
    },
    TargetSpiritPositionOutOfRange {
        value: u32,
    },
    UnmappedAbnormalState {
        raw_id: u32,
    },
    PropertyStageOutOfRange {
        field: ScriptCombatProtocolPropertyStage,
        value: i16,
    },
    UnknownWeatherEffect {
        raw_weather: u8,
    },
    ParticipantDisplaySideUnresolved {
        role: ScriptCombatParticipantDisplayRole,
        id: u32,
        participant_type: u8,
        position: u8,
    },
    ChangeSpiritPositionOutOfRange {
        skill_id: u32,
    },
    InvalidChangeSpiritPosition {
        position: u8,
    },
}

impl ScriptCombatProtocolError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::InvalidSpiritPosition { .. } => "invalid_spirit_position",
            Self::TargetSpiritPositionOutOfRange { .. } => "target_spirit_position_out_of_range",
            Self::UnmappedAbnormalState { .. } => "unmapped_abnormal_state",
            Self::PropertyStageOutOfRange { .. } => "property_stage_out_of_range",
            Self::UnknownWeatherEffect { .. } => "unknown_weather_effect",
            Self::ParticipantDisplaySideUnresolved { .. } => "participant_display_side_unresolved",
            Self::ChangeSpiritPositionOutOfRange { .. } => "change_spirit_position_out_of_range",
            Self::InvalidChangeSpiritPosition { .. } => "invalid_change_spirit_position",
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidSpiritPosition { value } => *value as i64,
            Self::TargetSpiritPositionOutOfRange { value } => *value as i64,
            Self::UnmappedAbnormalState { raw_id } => *raw_id as i64,
            Self::PropertyStageOutOfRange { value, .. } => *value as i64,
            Self::UnknownWeatherEffect { raw_weather } => *raw_weather as i64,
            Self::ParticipantDisplaySideUnresolved { id, .. } => *id as i64,
            Self::ChangeSpiritPositionOutOfRange { skill_id } => *skill_id as i64,
            Self::InvalidChangeSpiritPosition { position } => *position as i64,
        }
    }

    pub const fn property_stage_code(&self) -> &'static str {
        match self {
            Self::PropertyStageOutOfRange { field, .. } => field.code(),
            _ => "",
        }
    }

    pub const fn participant_role_code(&self) -> &'static str {
        match self {
            Self::ParticipantDisplaySideUnresolved { role, .. } => role.code(),
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatProtocolPropertyStage {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    SpiritPower,
    DefensePierce,
    Critical,
}

impl ScriptCombatProtocolPropertyStage {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "pa",
            Self::PhysicalDefense => "pd",
            Self::MagicAttack => "ma",
            Self::MagicDefense => "md",
            Self::Speed => "ve",
            Self::SpiritPower => "sp",
            Self::DefensePierce => "dp",
            Self::Critical => "crit",
        }
    }
}

impl fmt::Display for ScriptCombatProtocolPropertyStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptCombatParticipantDisplayRole {
    Actor,
    Target,
}

impl ScriptCombatParticipantDisplayRole {
    pub const fn code(self) -> &'static str {
        self.as_str()
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::Target => "target",
        }
    }

    pub const fn identity_str(self) -> &'static str {
        match self {
            Self::Actor => "offense",
            Self::Target => "defense",
        }
    }
}

impl fmt::Display for ScriptCombatParticipantDisplayRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ScriptCombatProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpiritPosition { value } => {
                write!(f, "spirit position must be in 1..=6, got {value}")
            }
            Self::TargetSpiritPositionOutOfRange { value } => {
                write!(f, "target spirit position out of range: {value}")
            }
            Self::UnmappedAbnormalState { raw_id } => {
                write!(f, "unmapped combat abnormal state id: {raw_id}")
            }
            Self::PropertyStageOutOfRange { field, value } => {
                write!(f, "combat property stage {field} out of range: {value}")
            }
            Self::UnknownWeatherEffect { raw_weather } => {
                write!(f, "unknown combat weather effect: {raw_weather}")
            }
            Self::ParticipantDisplaySideUnresolved {
                role,
                id,
                participant_type,
                position,
            } => write!(
                f,
                "combat participant display {role} side unresolved {}_id={id} {}_type={participant_type} {}_index={position}",
                role.identity_str(),
                role.identity_str(),
                role.identity_str()
            ),
            Self::ChangeSpiritPositionOutOfRange { skill_id } => {
                write!(f, "combat change spirit position out of range: {skill_id}")
            }
            Self::InvalidChangeSpiritPosition { position } => {
                write!(f, "combat change spirit position must be in 1..=6, got {position}")
            }
        }
    }
}
