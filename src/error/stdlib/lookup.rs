use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupError {
    NotFound {
        entity: ScriptLookupEntity,
        key: String,
    },
}

impl ScriptLookupError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "not_found",
        }
    }

    pub fn entity_code(&self) -> String {
        match self {
            Self::NotFound { entity, .. } => entity.code().to_string(),
        }
    }

    pub fn key(&self) -> String {
        match self {
            Self::NotFound { key, .. } => key.clone(),
        }
    }
}

impl fmt::Display for ScriptLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { entity, key } => {
                write!(f, "{} not found: {key}", entity.label())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLookupEntity {
    ItemInfo,
    StriveItemInfo,
    GuardianPetPropertyInfo,
    TitleInfo,
    MagicInfo,
    PluginInfo,
    SpiritBook,
    TalentInfo,
    SkillInfo,
    SpiritInfo,
}

impl ScriptLookupEntity {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ItemInfo => "item_info",
            Self::StriveItemInfo => "strive_item_info",
            Self::GuardianPetPropertyInfo => "guardian_pet_property_info",
            Self::TitleInfo => "title_info",
            Self::MagicInfo => "magic_info",
            Self::PluginInfo => "plugin_info",
            Self::SpiritBook => "spirit_book",
            Self::TalentInfo => "talent_info",
            Self::SkillInfo => "skill_info",
            Self::SpiritInfo => "spirit_info",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ItemInfo => "item info",
            Self::StriveItemInfo => "strive item info",
            Self::GuardianPetPropertyInfo => "guardian pet property info",
            Self::TitleInfo => "title info",
            Self::MagicInfo => "magic info",
            Self::PluginInfo => "plugin info",
            Self::SpiritBook => "spirit book",
            Self::TalentInfo => "talent info",
            Self::SkillInfo => "skill info",
            Self::SpiritInfo => "spirit info",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptStaticDataError {
    StaticGameDataNotInitialized,
    ActiveConfigNotAvailable {
        source: ScriptActiveConfigUnavailableSource,
        message: String,
    },
    NotImplemented {
        function_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActiveConfigUnavailableSource {
    Download,
    OptionalResourceParser,
    Other { code: String },
}

impl ScriptActiveConfigUnavailableSource {
    pub fn from_error_code(code: impl Into<String>) -> Self {
        match code.into().as_str() {
            "static_data.download" => Self::Download,
            "static_data.optional_resource_parser" => Self::OptionalResourceParser,
            code => Self::Other {
                code: code.to_string(),
            },
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::Download => "static_data.download",
            Self::OptionalResourceParser => "static_data.optional_resource_parser",
            Self::Other { code } => code.as_str(),
        }
    }
}

impl ScriptStaticDataError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::StaticGameDataNotInitialized => "static_game_data_not_initialized",
            Self::ActiveConfigNotAvailable { .. } => "active_config_not_available",
            Self::NotImplemented { .. } => "not_implemented",
        }
    }

    pub fn function_name(&self) -> String {
        match self {
            Self::NotImplemented { function_name } => function_name.clone(),
            _ => String::new(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ActiveConfigNotAvailable { message, .. } => message.clone(),
            _ => String::new(),
        }
    }

    pub fn active_config_source_code(&self) -> String {
        match self {
            Self::ActiveConfigNotAvailable { source, .. } => source.code().to_string(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for ScriptStaticDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaticGameDataNotInitialized => f.write_str("static game data not initialized"),
            Self::ActiveConfigNotAvailable { message, .. } => {
                write!(f, "active config not available: {message}")
            }
            Self::NotImplemented { function_name } => {
                write!(f, "{function_name} not yet implemented")
            }
        }
    }
}
