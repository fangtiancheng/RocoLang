use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornPalaceNoteItem {
    pub item_index: i64,
    pub item_id: i64,
    pub count: i64,
    pub need: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornPalaceNotesInfo {
    pub items: Vec<CapricornPalaceNoteItem>,
    pub can_summon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornTeamPlayer {
    pub uin: i64,
    pub nick: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapricornTeamSnapshot {
    pub players: Vec<CapricornTeamPlayer>,
    pub ticks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornInviteListInfo {
    pub result_code: i64,
    pub message: String,
    pub players: Vec<CapricornTeamPlayer>,
    pub ticks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornTeamOperationInfo {
    pub result_code: i64,
    pub message: String,
    pub team: RocoOptionalCapricornTeamSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalCapricornTeamSnapshot {
    Missing,
    Present { value: CapricornTeamSnapshot },
}

impl RocoOptionalCapricornTeamSnapshot {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: CapricornTeamSnapshot) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn value(&self) -> Option<CapricornTeamSnapshot> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value.clone()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapricornSecondTask {
    pub task_type: i64,
    pub data1: i64,
    pub data2: i64,
    pub step: i64,
    pub current: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalCapricornSecondTask {
    Missing,
    Present { value: CapricornSecondTask },
}

impl RocoOptionalCapricornSecondTask {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: CapricornSecondTask) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn value(&self) -> Option<CapricornSecondTask> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornStarPalaceInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornSecondInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub finish: RocoOptionalI64,
    pub current: RocoOptionalI64,
    pub position: RocoOptionalI64,
    pub second_task: RocoOptionalCapricornSecondTask,
    pub bag_candidates: Vec<CapricornBagCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapricornThirdInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub finish: RocoOptionalI64,
    pub current: RocoOptionalI64,
    pub remain: RocoOptionalI64,
    pub price: RocoOptionalI64,
    pub limit: RocoOptionalI64,
    pub progress_percent: RocoOptionalI64,
    pub reward_num: RocoOptionalI64,
    pub tips: RocoOptionalI64,
    pub bag_candidates: Vec<CapricornBagCandidate>,
}
