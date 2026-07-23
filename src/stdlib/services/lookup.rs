use std::sync::{Arc, Mutex};

use rhai::{Array, Module, NativeCallContext};

use crate::stdlib::util::{
    lock_stdlib, parse_i64_array_at, to_array, to_rhai_error, to_rhai_error_in_context,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_item_info", move |item_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_item_info(item_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "lookup_items_info",
            move |context: NativeCallContext, item_ids: Array| {
                let item_ids = parse_i64_array_at("item_ids[]", item_ids, context.call_position())?;
                let mut lib = lock_stdlib(&stdlib)?;
                lib.lookup_items_info(item_ids)
                    .map(|infos| to_array(&infos))
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_strive_item_info", move |item_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_strive_item_info(item_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_strive_item_infos", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_strive_item_infos()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_features_name", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_features_name()
                .map(|names| {
                    names
                        .into_iter()
                        .map(rhai::Dynamic::from)
                        .collect::<Array>()
                })
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_guardian_pet_property_info", move |level: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_guardian_pet_property_info(level)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_title_info", move |title_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_title_info(title_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_magic_info", move |magic_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_magic_info(magic_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_plugin_info", move |plugin_name: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_plugin_info(plugin_name).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_plugin_infos", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_plugin_infos()
                .map(|plugins| to_array(&plugins))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_ladder_match_config", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_ladder_match_config().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_talent_info", move |talent_type: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_talent_info(talent_type).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_talent_infos", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_talent_infos()
                .map(|talents| to_array(&talents))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_skill_info", move |skill_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_skill_info(skill_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "lookup_skills_info",
            move |context: NativeCallContext, skill_ids: Array| {
                let skill_ids =
                    parse_i64_array_at("skill_ids[]", skill_ids, context.call_position())?;
                let mut lib = lock_stdlib(&stdlib)?;
                lib.lookup_skills_info(skill_ids)
                    .map(|infos| to_array(&infos))
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_spirit_info", move |spirit_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_spirit_info(spirit_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_lookup_spirit_info", move |spirit_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_lookup_spirit_info(spirit_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "lookup_spirits_info",
            move |context: NativeCallContext, spirit_ids: Array| {
                let spirit_ids =
                    parse_i64_array_at("spirit_ids[]", spirit_ids, context.call_position())?;
                let mut lib = lock_stdlib(&stdlib)?;
                lib.lookup_spirits_info(spirit_ids)
                    .map(|infos| to_array(&infos))
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_spirit_book_summaries", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_spirit_book_summaries()
                .map(|summaries| to_array(&summaries))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_spirit_book", move |book_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_spirit_book(book_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_spirit_book_entries", move |book_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_spirit_book_entries(book_id)
                .map(|entries| to_array(&entries))
                .map_err(to_rhai_error)
        });
    }
}
