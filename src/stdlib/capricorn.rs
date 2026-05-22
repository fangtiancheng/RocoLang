use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - second_query_bag kind: 0=level-up candidates, 1=evolve candidates, matching CGI type.
// - second_answer_quiz answer_index is 0-based, matching AS question=0/1.
// - third_settle_battle boss_index is 0-based, matching ui1848 COMBAT_NPCID[index].
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "query_palace_notes",
        capricorn_query_palace_notes
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "query_invite_list",
        capricorn_query_invite_list
    );
    register_stdlib_fn_1!(module, stdlib, "invite_player", capricorn_invite_player, uin: i64);
    register_stdlib_fn_0!(module, stdlib, "cancel_invite", capricorn_cancel_invite);
    register_stdlib_fn_1!(module, stdlib, "accept_invite", capricorn_accept_invite, uin: i64);
    register_stdlib_fn_1!(module, stdlib, "refuse_invite", capricorn_refuse_invite, uin: i64);
    register_stdlib_fn_0!(module, stdlib, "leave_team", capricorn_leave_team);
    register_stdlib_fn_0!(module, stdlib, "disband_team", capricorn_disband_team);

    register_stdlib_fn_0!(
        module,
        stdlib,
        "star_palace_summon",
        capricorn_star_palace_summon
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "star_palace_quick_summon",
        capricorn_star_palace_quick_summon
    );
    register_stdlib_fn_0!(module, stdlib, "second_query", capricorn_second_query);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_random_task",
        capricorn_second_random_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_settle_battle_task",
        capricorn_second_settle_battle_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_give_up_task",
        capricorn_second_give_up_task
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_accept_task",
        capricorn_second_accept_task
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_answer_quiz",
        capricorn_second_answer_quiz,
        answer_index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_query_bag",
        capricorn_second_query_bag,
        kind: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_level_up",
        capricorn_second_level_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_buy_up",
        capricorn_second_buy_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolve",
        capricorn_second_evolve,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", capricorn_third_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_settle_battle",
        capricorn_third_settle_battle,
        boss_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_buy_star_item",
        capricorn_third_buy_star_item
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_buy_progress",
        capricorn_third_buy_progress,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_add_challenge_count",
        capricorn_third_add_challenge_count
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_buy_star_num",
        capricorn_third_buy_star_num
    );
    register_stdlib_fn_0!(module, stdlib, "third_query_bag", capricorn_third_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_evolve",
        capricorn_third_evolve,
        spirit_id: i64,
        catch_time: i64
    );
}
