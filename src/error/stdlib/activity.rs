use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityOperationError {
    MysteryFusionMaterialCountExceedsLimit {
        count: usize,
        limit: usize,
    },
    SummonDrawCountInvalid {
        count: i64,
    },
    InvalidOption {
        activity: ScriptActivityName,
        field: ScriptActivityOptionField,
        value: u32,
    },
}

impl ScriptActivityOperationError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { .. } => {
                "mystery_fusion_material_count_exceeds_limit"
            }
            Self::SummonDrawCountInvalid { .. } => "summon_draw_count_invalid",
            Self::InvalidOption { .. } => "invalid_option",
        }
    }

    pub const fn activity_code(&self) -> &'static str {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { .. } => "mystery_fusion",
            Self::SummonDrawCountInvalid { .. } => "summon",
            Self::InvalidOption { activity, .. } => activity.code(),
        }
    }

    pub const fn field_code(&self) -> &'static str {
        match self {
            Self::InvalidOption { field, .. } => field.code(),
            _ => "",
        }
    }

    pub const fn count(&self) -> i64 {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { count, .. } => *count as i64,
            Self::SummonDrawCountInvalid { count } => *count,
            _ => -1,
        }
    }

    pub const fn limit(&self) -> i64 {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { limit, .. } => *limit as i64,
            _ => -1,
        }
    }

    pub const fn value(&self) -> i64 {
        match self {
            Self::InvalidOption { value, .. } => *value as i64,
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityName {
    Gemini,
}

impl ScriptActivityName {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
        }
    }
}

impl fmt::Display for ScriptActivityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptActivityOptionField {
    Kind,
    Side,
}

impl ScriptActivityOptionField {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Kind => "kind",
            Self::Side => "side",
        }
    }
}

impl fmt::Display for ScriptActivityOptionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl fmt::Display for ScriptActivityOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MysteryFusionMaterialCountExceedsLimit { count, limit } => write!(
                f,
                "mystery_fusion::fuse material count {count} exceeds AS limit {limit}"
            ),
            Self::SummonDrawCountInvalid { count } => {
                write!(f, "summon::draw draw_count must be 1 or 10, got {count}")
            }
            Self::InvalidOption {
                activity,
                field,
                value,
            } => write!(f, "{activity} invalid {field}: {value}"),
        }
    }
}
