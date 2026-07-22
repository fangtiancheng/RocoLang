use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookEntry {
    pub id: i64,
    pub starred: bool,
    pub unknown: bool,
    pub newed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookGroup {
    pub template_id: i64,
    pub spirits: Vec<SpiritBookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookSummary {
    pub id: i64,
    pub name: String,
    pub is_new: bool,
    pub has_cover: bool,
    pub background: String,
    pub page_idx: i64,
    pub spirit_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookInfo {
    pub id: i64,
    pub name: String,
    pub is_new: bool,
    pub has_cover: bool,
    pub background: String,
    pub page_idx: i64,
    pub groups: Vec<SpiritBookGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookStates {
    pub uin: i64,
    pub count: i64,
    pub states: Vec<i64>,
}

impl SpiritBookStates {
    pub fn spirit_state(&self, spirit_id: i64) -> SpiritBookState {
        let state_code = spirit_id
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| self.states.get(index).copied())
            .unwrap_or(0);
        SpiritBookState::from_code(state_code)
    }

    pub fn spirit_owned(&self, spirit_id: i64) -> bool {
        self.spirit_state(spirit_id).is_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritBookSpiritState {
    pub spirit_id: i64,
    pub state: i64,
    pub owned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiritBookState {
    Unknown,
    Found,
    Caught,
    Released,
}

impl SpiritBookState {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::Found,
            2 => Self::Caught,
            3 => Self::Released,
            _ => Self::Unknown,
        }
    }

    pub const fn code(self) -> i64 {
        match self {
            Self::Unknown => 0,
            Self::Found => 1,
            Self::Caught => 2,
            Self::Released => 3,
        }
    }

    pub const fn is_owned(self) -> bool {
        matches!(self, Self::Caught | Self::Released)
    }
}
