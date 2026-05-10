fn format_skill_requirements(required_skill_ids) {
    const skill_infos = lookup::lookup_skills_info(required_skill_ids);
    let text = "";
    let is_first = true;
    for skill_info in skill_infos {
        if is_first {
            is_first = false;
        } else {
            text = text + ",";
        }
        text = text + skill_info.name + "(" + skill_info.id + ")";
    }

    return "[" + text + "]";
}

fn is_spirit_ready(spirit_info, spirit_id, required_skill_ids) {
    if spirit_info.spirit_id != spirit_id {
        return #{
            ok: false,
            code: 1,
            message: "spirit id mismatch: expected " + spirit_id + ", got " + spirit_info.spirit_id
        };
    }

    let missing_skill_ids = [];
    for required_skill_id in required_skill_ids {
        let found = false;
        for skill in spirit_info.skills {
            if skill.skill_id == required_skill_id {
                found = true;
            }
        }

        if !found {
            missing_skill_ids.push(required_skill_id);
        }
    }

    if len(missing_skill_ids) != 0 {
        return #{
            ok: false,
            code: 2,
            message: "spirit " + spirit_id + " missing required skills " + format_skill_requirements(missing_skill_ids)
        };
    }

    return #{
        ok: true,
        code: 0,
        message: "spirit " + spirit_id + " is ready"
    };
}

fn select_best_storage_spirit(spirit_id, required_skill_ids) {
    const storage_spirits = spirit::list_storage_spirits();
    let best = ();
    let found = false;
    for spirit_info in storage_spirits {
        if spirit_info.spirit_id == spirit_id {
            let has_required_skills = len(required_skill_ids) == 0;
            if spirit_info.level == 100 {
                const detail = spirit::get_storage_spirit_detail(spirit_info.spirit_id, spirit_info.catch_time);
                has_required_skills = is_spirit_ready(detail, spirit_id, required_skill_ids).ok;
            }

            if !found {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
                found = true;
            } else if spirit_info.level > best.level {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            } else if spirit_info.level == best.level && spirit_info.level == 100 && has_required_skills && !best.has_required_skills {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            } else if spirit_info.level == best.level && has_required_skills == best.has_required_skills && spirit_info.storage_time > best.storage_time {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            }
        }
    }

    if !found {
        return #{ found: false };
    }

    return #{
        found: true,
        spirit_id: best.spirit_id,
        catch_time: best.catch_time,
        level: best.level,
        storage_time: best.storage_time,
        has_required_skills: best.has_required_skills
    };
}

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
    for spirit_info in bag.spirits {
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
