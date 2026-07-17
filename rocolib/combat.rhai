fn challenge_wild_spirit(spirit_id) {
    combat::start_combat(0, 1, spirit_id, 0)
}

fn challenge_wild_spirit_with_catch_time(spirit_id, catch_time) {
    combat::start_combat(0, 1, spirit_id, catch_time)
}

fn challenge_boss(boss_id) {
    combat::start_combat(0, 2, boss_id, 0)
}

fn challenge_boss_with_catch_time(boss_id, catch_time) {
    combat::start_combat(0, 2, boss_id, catch_time)
}

fn remaining_spirit_count(side) {
    let count = 0;
    for spirit in side.spirits {
        if spirit.hp > 0 {
            count = count + 1;
        }
    }
    count
}

fn rival_remaining_spirit_count(state) {
    remaining_spirit_count(state.rival_side)
}

fn has_available_action(actions) {
    actions.can_use_skill
        || actions.can_capture
        || actions.can_use_item
        || actions.can_change_spirit
        || actions.can_escape
}

fn wait_until_action_ready() {
    loop {
        let snapshot = combat::get_action_snapshot();
        if snapshot.is_finished {
            return false;
        }
        if has_available_action(snapshot.actions) {
            return true;
        }
        combat::wait_next_action();
    }
}

fn wait_until_escape_ready() {
    loop {
        let snapshot = combat::get_action_snapshot();
        if snapshot.is_finished {
            return false;
        }
        if snapshot.actions.can_escape {
            return true;
        }
        combat::wait_next_action();
    }
}

fn assert_action_ready(action_result, label) {
    system::assert(action_result.ok, label + " failed: " + action_result.message);
    system::assert(
        action_result.combat_finished || action_result.next_action_ready,
        label + " did not reach next action"
    );
}

fn escape_if_possible_and_wait(label) {
    if combat::is_combat_finished() || !wait_until_escape_ready() {
        return true;
    }

    let escape_result = combat::try_combat_escape_and_wait();
    assert_action_ready(escape_result, label);
    return escape_result.combat_finished;
}
