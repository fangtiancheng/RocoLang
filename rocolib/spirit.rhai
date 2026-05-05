fn recover_all_spirits_by_mode(recovery_mode, is_vip) {
    if recovery_mode == "vip" {
        return spirit::try_recover_all_spirits();
    }

    if recovery_mode == "hospital" {
        return recover_all_spirits_hospital();
    }

    if recovery_mode == "money" {
        return recover_all_spirits_money();
    }

    return auto_recover_all_spirits(is_vip);
}

fn auto_recover_all_spirits(is_vip) {
    if is_vip {
        let recover_result = spirit::try_recover_all_spirits();
        if recover_result.ok {
            return recover_result;
        }
        print("vip recovery failed: " + recover_result.message);
    }

    let hospital_result = recover_all_spirits_hospital();
    if hospital_result.ok {
        return hospital_result;
    }
    print("hospital recovery failed: " + hospital_result.message);

    return recover_all_spirits_money();
}

fn recover_all_spirits_hospital() {
    const HOSPITAL_SCENE_ID = 57;
    const MOVE_TIMEOUT_MS = 30000;

    let current_scene = scene::get_current_scene();
    if current_scene == HOSPITAL_SCENE_ID {
        return recover_all_spirits_money();
    }

    let move_result = scene::try_move_to_scene(HOSPITAL_SCENE_ID, MOVE_TIMEOUT_MS);
    if !move_result.ok {
        return move_result;
    }

    let recover_result = recover_all_spirits_money();
    if !recover_result.ok {
        let restore_result = scene::try_move_to_scene(current_scene, MOVE_TIMEOUT_MS);
        if !restore_result.ok {
            print("failed to restore original scene after recovery: " + restore_result.message);
        }
        return recover_result;
    }

    let restore_result = scene::try_move_to_scene(current_scene, MOVE_TIMEOUT_MS);
    if !restore_result.ok {
        return restore_result;
    }

    return recover_result;
}

fn recover_all_spirits_money() {
    let bag = spirit::get_spirit_bag();
    let index = 0;
    while index < len(bag.spirits) {
        let spirit_info = bag.spirits[index];
        if spirit_info.hp < spirit_info.max_hp {
            let restore_result = spirit::restore_spirit(spirit_info.spirit_id, spirit_info.position);
            if !restore_result {
                return #{ ok: false, code: 2, message: "restore_spirit failed on bag position " + spirit_info.position };
            }
        }
        index = index + 1;
    }

    return #{ ok: true, code: 0, message: "" };
}
