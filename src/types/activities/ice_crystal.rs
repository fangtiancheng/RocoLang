use serde::{Deserialize, Serialize};

use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalBattleInfo {
    pub battle_index: i64,
    pub fight_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalIceCrystalBattleInfo {
    Missing,
    Present { value: IceCrystalBattleInfo },
}

impl RocoOptionalIceCrystalBattleInfo {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: IceCrystalBattleInfo) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn value(&self) -> Option<IceCrystalBattleInfo> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCrystalInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub progress: RocoOptionalI64,
    pub battle_times: RocoOptionalI64,
    pub battle_index: RocoOptionalI64,
    pub get_times: RocoOptionalI64,
    pub add: RocoOptionalI64,
    pub item_counts: Vec<i64>,
    pub crystal_counts: Vec<i64>,
    pub item_costs: Vec<i64>,
    pub one_key_diamond_costs: Vec<i64>,
    pub current_battle: RocoOptionalIceCrystalBattleInfo,
    pub bag_candidates: Vec<IceCrystalBagCandidate>,
    pub rewards: Vec<IceCrystalRewardItem>,
}
