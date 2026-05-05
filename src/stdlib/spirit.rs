use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2,
    register_stdlib_fn_4, register_stdlib_fn_5, register_stdlib_fn_7, to_array, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_2!(
        module,
        stdlib,
        "fetch_spirit",
        fetch_spirit,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "recover_all_spirits", recover_all_spirits);
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_recover_all_spirits", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_recover_all_spirits().map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_4!(
        module,
        stdlib,
        "use_spirit_item",
        use_spirit_item,
        spirit_id: i64,
        position: i64,
        item_id: i64,
        count: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "restore_spirit",
        restore_spirit,
        spirit_id: i64,
        position: i64
    );
    register_stdlib_fn_4!(
        module,
        stdlib,
        "use_talent_refresh_item",
        use_talent_refresh_item,
        spirit_id: i64,
        position: i64,
        item_id: i64,
        count: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "allocate_exp",
        allocate_exp,
        position: i64,
        exp: i64
    );
    register_stdlib_fn_7!(
        module,
        stdlib,
        "save_strive_add",
        save_strive_add,
        position: i64,
        pa: i64,
        pd: i64,
        ma: i64,
        md: i64,
        sp: i64,
        hp: i64
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("clear_lineup", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.clear_lineup().map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_1!(
        module,
        stdlib,
        "store_spirit",
        store_spirit,
        position: i64
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_spirit_bag", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_spirit_bag().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_bag_items", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_bag_items()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("take_pushed_drops", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.take_pushed_drops()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_2!(
        module,
        stdlib,
        "learn_skill",
        learn_skill,
        position: i64,
        skill_id: i64
    );
    register_stdlib_fn_1!(module, stdlib, "get_skills", get_skills, position: i64);
    register_stdlib_fn_5!(
        module,
        stdlib,
        "equip_item",
        equip_item,
        position: i64,
        equipment_server_id: i64,
        equipment_catch_time: i64,
        spirit_id: i64,
        spirit_catch_time: i64
    );
}
