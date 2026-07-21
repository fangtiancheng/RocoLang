use serde::{Deserialize, Serialize};

use super::{to_array, Engine, RocoOptionalI64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerInfo {
    pub result_code: i64,
    pub message: String,
    pub mop: i64,
    pub boss_id: i64,
    pub countdown: i64,
    pub auto_sell: bool,
    pub money: i64,
    pub clips: Vec<i64>,
    pub storeys: Vec<StarTowerStorey>,
    pub top: RocoOptionalStarTowerTop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerStorey {
    pub storey_index: i64,
    pub first: i64,
    pub can_quick_fight: bool,
    pub nodes: Vec<StarTowerNode>,
    pub exchange_items: Vec<StarTowerExchangeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerNode {
    pub node_index: i64,
    pub star: i64,
    pub spirit_id: i64,
    pub fight_id: i64,
    pub item_id: i64,
    pub reward: i64,
    pub equip_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerExchangeItem {
    pub index: i64,
    pub item_id: i64,
    pub item_name: String,
    pub spirit_id: RocoOptionalI64,
    pub spirit_name: String,
    pub owned: i64,
    pub required: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTowerTop {
    pub star: i64,
    pub refresh: i64,
    pub fight_desc: String,
    pub task_desc: String,
    pub fight_id: i64,
    pub tokens: Vec<i64>,
    pub exchanges: Vec<i64>,
    pub missions: Vec<StarTowerTopMission>,
    pub rewards: Vec<StarTowerTopReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RocoOptionalStarTowerTop {
    Missing,
    Present { value: StarTowerTop },
}

impl RocoOptionalStarTowerTop {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn present(value: StarTowerTop) -> Self {
        Self::Present { value }
    }

    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn value(&self) -> Option<StarTowerTop> {
        match self {
            Self::Missing => None,
            Self::Present { value } => Some(value.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerTopMission {
    pub index: i64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTowerTopReward {
    pub index: i64,
    pub threshold: i64,
    pub name: String,
    pub amount: String,
    pub state: i64,
    pub claimed: bool,
    pub claimable: bool,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_optional_getters!(engine, RocoOptionalStarTowerTop);
    engine.register_get("star", |value: &mut RocoOptionalStarTowerTop| {
        value.value().map(|top| top.star).unwrap_or_default()
    });
    engine.register_get("fight_id", |value: &mut RocoOptionalStarTowerTop| {
        value.value().map(|top| top.fight_id).unwrap_or_default()
    });
    engine.register_get("tokens", |value: &mut RocoOptionalStarTowerTop| {
        value
            .value()
            .map(|top| to_array(&top.tokens))
            .unwrap_or_default()
    });
    engine.register_get("exchanges", |value: &mut RocoOptionalStarTowerTop| {
        value
            .value()
            .map(|top| to_array(&top.exchanges))
            .unwrap_or_default()
    });
    register_getters!(
        engine,
        StarTowerNode,
        node_index,
        star,
        spirit_id,
        fight_id,
        item_id,
        reward,
        equip_id
    );
    register_getters!(
        engine,
        StarTowerStorey,
        storey_index,
        first,
        can_quick_fight
    );
    engine.register_get("nodes", |value: &mut StarTowerStorey| {
        to_array(&value.nodes)
    });
    engine.register_get("exchange_items", |value: &mut StarTowerStorey| {
        to_array(&value.exchange_items)
    });
    register_getters!(
        engine,
        StarTowerExchangeItem,
        index,
        item_id,
        item_name,
        spirit_id,
        spirit_name,
        owned,
        required
    );
    register_getters!(
        engine,
        StarTowerTop,
        star,
        refresh,
        fight_desc,
        task_desc,
        fight_id
    );
    engine.register_get("tokens", |value: &mut StarTowerTop| to_array(&value.tokens));
    engine.register_get("exchanges", |value: &mut StarTowerTop| {
        to_array(&value.exchanges)
    });
    engine.register_get("missions", |value: &mut StarTowerTop| {
        to_array(&value.missions)
    });
    engine.register_get("rewards", |value: &mut StarTowerTop| {
        to_array(&value.rewards)
    });
    register_getters!(engine, StarTowerTopMission, index, description, completed);
    register_getters!(
        engine,
        StarTowerTopReward,
        index,
        threshold,
        name,
        amount,
        state,
        claimed,
        claimable
    );
    register_getters!(
        engine,
        StarTowerInfo,
        result_code,
        message,
        mop,
        boss_id,
        countdown,
        auto_sell,
        money
    );
    engine.register_get("has_top", |value: &mut StarTowerInfo| {
        value.top.is_present()
    });
    engine.register_get("top", |value: &mut StarTowerInfo| value.top.clone());
    engine.register_get("clips", |value: &mut StarTowerInfo| to_array(&value.clips));
    engine.register_get("storeys", |value: &mut StarTowerInfo| {
        to_array(&value.storeys)
    });
}
