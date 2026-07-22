use crate::types::*;
use ::rhai::Engine;

pub(super) fn register_rhai_getters(engine: &mut Engine) {
    register_getters!(
        engine,
        SceneSpiritInfo,
        spirit_id,
        count,
        area_index,
        is_rare,
        is_boss,
        is_npc_boss
    );
    register_getters!(
        engine,
        SceneRoleInfo,
        uin,
        id,
        nick_name,
        level,
        loc_x,
        loc_y,
        pk_state,
        is_in_combat,
        is_vip,
        vip_level,
        trainer_level,
        trainer_exp
    );
}
