use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RocoRequestContext {
    pub raw: String,
}

impl RocoRequestContext {
    pub fn from_raw(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    pub fn domain(&self) -> &str {
        self.raw
            .split_once('.')
            .map(|(domain, _)| domain)
            .unwrap_or(&self.raw)
    }

    pub fn action(&self) -> &str {
        self.raw
            .split_once('.')
            .map(|(_, action)| action)
            .unwrap_or_default()
    }
}

impl Serialize for RocoRequestContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RocoRequestContext", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("domain", self.domain())?;
        state.serialize_field("action", self.action())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RocoRequestContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireContext {
            #[serde(default)]
            raw: String,
            #[serde(default)]
            domain: String,
            #[serde(default)]
            action: String,
        }

        let wire = WireContext::deserialize(deserializer)?;
        if !wire.raw.is_empty() {
            return Ok(Self::from_raw(wire.raw));
        }
        if wire.action.is_empty() {
            Ok(Self::from_raw(wire.domain))
        } else {
            Ok(Self::from_raw(format!("{}.{}", wire.domain, wire.action)))
        }
    }
}

impl From<String> for RocoRequestContext {
    fn from(raw: String) -> Self {
        Self::from_raw(raw)
    }
}

impl From<&str> for RocoRequestContext {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoRewardKind {
    Item,
    Money,
    AssignableExp,
    Furniture,
    Spirit,
    SpiritEquipment,
    TimedDress,
    Unmapped,
}

impl RocoRewardKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Item => "item",
            Self::Money => "money",
            Self::AssignableExp => "assignable_exp",
            Self::Furniture => "furniture",
            Self::Spirit => "spirit",
            Self::SpiritEquipment => "spirit_equipment",
            Self::TimedDress => "timed_dress",
            Self::Unmapped => "unmapped",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoOptionalI64 {
    Missing,
    Present { value: i64 },
}

impl RocoOptionalI64 {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: i64) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub const fn value(self) -> Option<i64> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value),
        }
    }
}

impl From<Option<i64>> for RocoOptionalI64 {
    fn from(value: Option<i64>) -> Self {
        value.map(Self::present).unwrap_or(Self::Missing)
    }
}

impl From<Option<u32>> for RocoOptionalI64 {
    fn from(value: Option<u32>) -> Self {
        value
            .map(|value| Self::present(i64::from(value)))
            .unwrap_or(Self::Missing)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoDisplayItem {
    pub item_id: i64,
    pub item_count: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoOptionalDisplayItem {
    Missing,
    Present { value: RocoDisplayItem },
}

impl RocoOptionalDisplayItem {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: RocoDisplayItem) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub const fn value(self) -> Option<RocoDisplayItem> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value),
        }
    }
}
