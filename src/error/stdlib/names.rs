use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptModuleName {
    AlchemyFurnace,
    Aquarius,
    Aries,
    Cancer,
    Capricorn,
    Combat,
    DarkCity,
    DiamondTear,
    FourSeasons,
    Game,
    Gemini,
    IceCrystal,
    Ladder,
    Leo,
    Libra,
    Lookup,
    MagicPioneer,
    Manor,
    MountainSea,
    MultiEvolution,
    MysteryFusion,
    News,
    NewsTimes,
    PetTraining,
    Pisces,
    PlayGuide,
    Profile,
    Role,
    Sagittarius,
    Scene,
    Scorpio,
    SentinelIntelligence,
    Spirit,
    SpiritBook,
    StarTower,
    Summon,
    System,
    Taurus,
    ThreeStarters,
    TreasureRealm,
    TypeLadder,
    Unicorn,
    Virgo,
    Custom { name: String },
}

impl ScriptModuleName {
    pub fn parse(name: &str) -> Self {
        match name {
            "alchemy_furnace" => Self::AlchemyFurnace,
            "aquarius" => Self::Aquarius,
            "aries" => Self::Aries,
            "cancer" => Self::Cancer,
            "capricorn" => Self::Capricorn,
            "combat" => Self::Combat,
            "dark_city" => Self::DarkCity,
            "diamond_tear" => Self::DiamondTear,
            "four_seasons" => Self::FourSeasons,
            "game" => Self::Game,
            "gemini" => Self::Gemini,
            "ice_crystal" => Self::IceCrystal,
            "ladder" => Self::Ladder,
            "leo" => Self::Leo,
            "libra" => Self::Libra,
            "lookup" => Self::Lookup,
            "magic_pioneer" => Self::MagicPioneer,
            "manor" => Self::Manor,
            "mountain_sea" => Self::MountainSea,
            "multi_evolution" => Self::MultiEvolution,
            "mystery_fusion" => Self::MysteryFusion,
            "news" => Self::News,
            "news_times" => Self::NewsTimes,
            "pet_training" => Self::PetTraining,
            "pisces" => Self::Pisces,
            "play_guide" => Self::PlayGuide,
            "profile" => Self::Profile,
            "role" => Self::Role,
            "sagittarius" => Self::Sagittarius,
            "scene" => Self::Scene,
            "scorpio" => Self::Scorpio,
            "sentinel_intelligence" => Self::SentinelIntelligence,
            "spirit" => Self::Spirit,
            "spirit_book" => Self::SpiritBook,
            "star_tower" => Self::StarTower,
            "summon" => Self::Summon,
            "system" => Self::System,
            "taurus" => Self::Taurus,
            "three_starters" => Self::ThreeStarters,
            "treasure_realm" => Self::TreasureRealm,
            "type_ladder" => Self::TypeLadder,
            "unicorn" => Self::Unicorn,
            "virgo" => Self::Virgo,
            _ => Self::Custom {
                name: name.to_string(),
            },
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::AlchemyFurnace => "alchemy_furnace",
            Self::Aquarius => "aquarius",
            Self::Aries => "aries",
            Self::Cancer => "cancer",
            Self::Capricorn => "capricorn",
            Self::Combat => "combat",
            Self::DarkCity => "dark_city",
            Self::DiamondTear => "diamond_tear",
            Self::FourSeasons => "four_seasons",
            Self::Game => "game",
            Self::Gemini => "gemini",
            Self::IceCrystal => "ice_crystal",
            Self::Ladder => "ladder",
            Self::Leo => "leo",
            Self::Libra => "libra",
            Self::Lookup => "lookup",
            Self::MagicPioneer => "magic_pioneer",
            Self::Manor => "manor",
            Self::MountainSea => "mountain_sea",
            Self::MultiEvolution => "multi_evolution",
            Self::MysteryFusion => "mystery_fusion",
            Self::News => "news",
            Self::NewsTimes => "news_times",
            Self::PetTraining => "pet_training",
            Self::Pisces => "pisces",
            Self::PlayGuide => "play_guide",
            Self::Profile => "profile",
            Self::Role => "role",
            Self::Sagittarius => "sagittarius",
            Self::Scene => "scene",
            Self::Scorpio => "scorpio",
            Self::SentinelIntelligence => "sentinel_intelligence",
            Self::Spirit => "spirit",
            Self::SpiritBook => "spirit_book",
            Self::StarTower => "star_tower",
            Self::Summon => "summon",
            Self::System => "system",
            Self::Taurus => "taurus",
            Self::ThreeStarters => "three_starters",
            Self::TreasureRealm => "treasure_realm",
            Self::TypeLadder => "type_ladder",
            Self::Unicorn => "unicorn",
            Self::Virgo => "virgo",
            Self::Custom { name } => name.as_str(),
        }
    }
}

impl fmt::Display for ScriptModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptFunctionName {
    pub module: ScriptModuleName,
    pub function: String,
}

impl ScriptFunctionName {
    pub fn parse(name: &str) -> Self {
        match name.split_once("::") {
            Some((module, function)) => Self {
                module: ScriptModuleName::parse(module),
                function: function.to_string(),
            },
            None => Self {
                module: ScriptModuleName::Custom {
                    name: String::new(),
                },
                function: name.to_string(),
            },
        }
    }

    pub fn module_code(&self) -> String {
        self.module.code().to_string()
    }

    pub fn qualified_name(&self) -> String {
        let module = self.module.code();
        if module.is_empty() {
            self.function.clone()
        } else {
            format!("{}::{}", module, self.function)
        }
    }
}

impl fmt::Display for ScriptFunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.qualified_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptUnsupportedError {
    Function { name: ScriptFunctionName },
}

impl ScriptUnsupportedError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Function { .. } => "stdlib.unsupported.function",
        }
    }
}

impl fmt::Display for ScriptUnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function { name } => write!(f, "{name} unsupported by this runtime"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptHttpResponseName {
    pub code: String,
}

impl ScriptHttpResponseName {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl fmt::Display for ScriptHttpResponseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code)
    }
}
