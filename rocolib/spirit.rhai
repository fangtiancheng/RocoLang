fn try_recover_all_spirits_by_mode(recovery_mode, is_vip) {
    if recovery_mode == "vip" {
        return spirit::try_recover_all_spirits();
    }

    if recovery_mode == "hospital" {
        return try_recover_all_spirits_hospital();
    }

    if recovery_mode == "money" {
        return try_recover_all_spirits_money();
    }

    return try_auto_recover_all_spirits(is_vip);
}

fn try_auto_recover_all_spirits(is_vip) {
    if is_vip {
        let recover_result = spirit::try_recover_all_spirits();
        if recover_result.ok {
            return recover_result;
        }
        print("vip recovery failed: " + recover_result.message);
    }

    let hospital_result = try_recover_all_spirits_hospital();
    if hospital_result.ok {
        return hospital_result;
    }
    print("hospital recovery failed: " + hospital_result.message);

    return try_recover_all_spirits_money();
}

fn try_recover_all_spirits_hospital() {
    const HOSPITAL_SCENE_ID = 57;
    const MOVE_TIMEOUT_MS = 30000;

    let current_scene = scene::get_current_scene();
    if current_scene == HOSPITAL_SCENE_ID {
        return try_recover_all_spirits_money();
    }

    let move_result = scene::try_move_to_scene(HOSPITAL_SCENE_ID, MOVE_TIMEOUT_MS);
    if !move_result.ok {
        return move_result;
    }

    let recover_result = try_recover_all_spirits_money();
    let restore_result = scene::try_move_to_scene(current_scene, MOVE_TIMEOUT_MS);
    if !restore_result.ok {
        return #{
            ok: false,
            code: restore_result.code,
            message: "failed to restore original scene after hospital recovery: " + restore_result.message
        };
    }

    return recover_result;
}

fn try_recover_all_spirits_money() {
    let bag = spirit::get_spirit_bag();
    let restored_count = 0;
    let index = 0;
    while index < len(bag.spirits) {
        let spirit_info = bag.spirits[index];
        if spirit_info.hp < spirit_info.max_hp {
            let restore_result = spirit::try_restore_spirit(spirit_info.spirit_id, spirit_info.position);
            if !restore_result.ok {
                return #{
                    ok: false,
                    code: restore_result.code,
                    message: "failed to restore spirit at bag position " + spirit_info.position + ": " + restore_result.message + ", restored_count=" + restored_count
                };
            }
            restored_count = restored_count + 1;
        }
        index = index + 1;
    }

    return #{
        ok: true,
        code: 0,
        message: "restored " + restored_count + " spirits"
    };
}

fn try_store_all_spirits() {
    const bag = spirit::get_spirit_bag();
    let position = len(bag.spirits);
    let stored_count = 0;

    while position >= 1 {
        const result = spirit::try_store_spirit(position);
        if !result.ok {
            return #{
                ok: false,
                code: result.code,
                message: "failed to store spirit at position " + position + ": " + result.message + ", stored_count=" + stored_count
            };
        }
        stored_count = stored_count + 1;
        position = position - 1;
    }

    const verify_bag = spirit::get_spirit_bag();
    if len(verify_bag.spirits) != 0 {
        return #{
            ok: false,
            code: 1,
            message: "bag still has " + len(verify_bag.spirits) + " spirits after storing " + stored_count
        };
    }

    return #{
        ok: true,
        code: 0,
        message: "stored " + stored_count + " spirits"
    };
}
