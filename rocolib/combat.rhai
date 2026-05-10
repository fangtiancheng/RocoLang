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
