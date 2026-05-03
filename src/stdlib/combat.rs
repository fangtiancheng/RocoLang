use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_4, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(module, stdlib, "invite_pk", invite_pk, target_uin: i64);
    register_stdlib_fn_0!(module, stdlib, "accept_pk", accept_pk);
    register_stdlib_fn_0!(module, stdlib, "reject_pk", reject_pk);
    register_stdlib_fn_4!(
        module,
        stdlib,
        "start_combat",
        start_combat,
        server_type: i64,
        combat_type: i64,
        rival_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(module, stdlib, "use_skill", use_skill, skill_id: i64);
    register_stdlib_fn_1!(module, stdlib, "try_use_skill", try_use_skill, skill_id: i64);
    register_stdlib_fn_1!(module, stdlib, "use_item", use_item, item_id: i64);
    register_stdlib_fn_1!(module, stdlib, "try_use_item", try_use_item, item_id: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "change_spirit",
        change_spirit,
        position: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "try_change_spirit",
        try_change_spirit,
        position: i64
    );
    register_stdlib_fn_0!(module, stdlib, "combat_escape", combat_escape);
    register_stdlib_fn_0!(module, stdlib, "try_combat_escape", try_combat_escape);
    register_stdlib_fn_0!(module, stdlib, "wait_round_end", wait_round_end);
    register_stdlib_fn_0!(module, stdlib, "get_battle_result", get_battle_result);
    register_stdlib_fn_0!(module, stdlib, "get_combat_actions", get_combat_actions);
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_combat_lineup", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_combat_lineup()
                .map(|spirits| {
                    let mut lineup = rhai::Array::new();
                    for spirit in spirits.into_iter() {
                        match spirit {
                            Some(spirit) => lineup.push(rhai::Dynamic::from(spirit)),
                            None => break,
                        }
                    }
                    lineup
                })
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_1!(module, stdlib, "can_use_skill", can_use_skill, skill_id: i64);
    register_stdlib_fn_1!(module, stdlib, "can_use_item", can_use_item, item_id: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "can_change_to_spirit",
        can_change_to_spirit,
        position: i64
    );
    register_stdlib_fn_0!(module, stdlib, "can_capture", can_capture);
    register_stdlib_fn_0!(module, stdlib, "get_battle_history", get_battle_history);
    register_stdlib_fn_0!(module, stdlib, "get_my_hp", get_my_hp);
    register_stdlib_fn_0!(module, stdlib, "get_my_max_hp", get_my_max_hp);
    register_stdlib_fn_0!(module, stdlib, "get_rival_hp", get_rival_hp);
    register_stdlib_fn_0!(module, stdlib, "get_rival_max_hp", get_rival_max_hp);
    register_stdlib_fn_1!(module, stdlib, "get_my_pp", get_my_pp, slot: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "get_my_spirit_info",
        get_my_spirit_info,
        position: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "get_rival_spirit_info",
        get_rival_spirit_info
    );
    register_stdlib_fn_0!(module, stdlib, "is_combat_finished", is_combat_finished);
    register_stdlib_fn_0!(module, stdlib, "get_current_round", get_current_round);
}
