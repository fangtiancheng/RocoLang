use serde::{Deserialize, Serialize};

use super::super::{to_array, Engine};
use super::super::{RocoOptionalI64, RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBagCandidate {
    pub candidate_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub bag_index: RocoOptionalI64,
    pub catch_time: RocoOptionalI64,
    pub level: RocoOptionalI64,
    pub need_money: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornBossInfo {
    pub slot: i64,
    pub npc_index: i64,
    pub spirit_id: RocoOptionalI64,
    pub fight_id: RocoOptionalI64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicornInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub bosses: Vec<UnicornBossInfo>,
    pub finish: RocoOptionalI64,
    pub start: RocoOptionalI64,
    pub total: RocoOptionalI64,
    pub book: RocoOptionalI64,
    pub cultivation_times: Vec<i64>,
    pub evolution_energy_costs: Vec<i64>,
    pub one_key_diamond_costs: Vec<i64>,
    pub purple_vine_count: RocoOptionalI64,
    pub energy: RocoOptionalI64,
    pub fruit_count: RocoOptionalI64,
    pub increase: RocoOptionalI64,
    pub bag_candidates: Vec<UnicornBagCandidate>,
    pub rewards: Vec<UnicornRewardItem>,
}

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        UnicornRewardItem,
        reward_id,
        reward_kind,
        raw_reward_type,
        count
    );
    register_getters!(
        engine,
        UnicornBagCandidate,
        candidate_index,
        spirit_id,
        bag_index,
        catch_time,
        level,
        need_money,
    );
    register_getters!(
        engine,
        UnicornBossInfo,
        slot,
        npc_index,
        spirit_id,
        fight_id
    );
    register_getters!(
        engine,
        UnicornInfo,
        result_code,
        message,
        request_context,
        finish,
        start,
        total,
        book,
        purple_vine_count,
        energy,
        fruit_count,
        increase
    );
    engine.register_get("bosses", |value: &mut UnicornInfo| to_array(&value.bosses));
    engine.register_get("cultivation_times", |value: &mut UnicornInfo| {
        to_array(&value.cultivation_times)
    });
    engine.register_get("evolution_energy_costs", |value: &mut UnicornInfo| {
        to_array(&value.evolution_energy_costs)
    });
    engine.register_get("one_key_diamond_costs", |value: &mut UnicornInfo| {
        to_array(&value.one_key_diamond_costs)
    });
    engine.register_get("bag_candidates", |value: &mut UnicornInfo| {
        to_array(&value.bag_candidates)
    });
    engine.register_get("rewards", |value: &mut UnicornInfo| {
        to_array(&value.rewards)
    });
}
