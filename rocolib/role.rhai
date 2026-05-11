fn find_cached_scene_role(uin) {
    const roles = role::get_cached_scene_roles();
    for role_info in roles {
        if role_info.uin == uin {
            return #{
                found: true,
                role: role_info
            };
        }
    }

    return #{
        found: false,
        role: ()
    };
}
