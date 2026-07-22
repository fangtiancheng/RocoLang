use super::*;

impl RocoEngine {
    pub(super) fn register_stdlib<T: RocoStdLib + 'static>(
        engine: &mut Engine,
        stdlib: Arc<Mutex<T>>,
    ) {
        Self::register_static_info_types(engine);
        crate::stdlib::register_modules(engine, stdlib);
    }
    pub(super) fn register_builtin_helpers(engine: &mut Engine) {
        engine.register_fn("len", |array: &mut Array| array.len() as i64);
    }

    fn to_array<T: Clone + Send + Sync + 'static>(items: &[T]) -> Array {
        items.iter().cloned().map(Dynamic::from).collect()
    }

    fn register_static_info_types(engine: &mut Engine) {
        macro_rules! register_getters {
            ($type:ty, $($field:ident),+ $(,)?) => {
                $(
                    engine.register_get(stringify!($field), |value: &mut $type| {
                        value.$field.clone()
                    });
                )+
            };
        }
        macro_rules! register_roco_type_basics {
            ($type:ty, $name:literal) => {
                engine.register_type_with_name::<$type>($name);
                engine.register_fn(
                    "to_string",
                    |value: &mut $type| -> std::result::Result<String, Box<rhai::EvalAltResult>> {
                        serde_json::to_string_pretty(value).map_err(|error| {
                            Box::<rhai::EvalAltResult>::from(rhai::EvalAltResult::ErrorRuntime(
                                Dynamic::from(
                                    RocoScriptRuntimeErrorValue::roco_type_json_serialize::<$type>(
                                        &error,
                                    ),
                                ),
                                rhai::Position::NONE,
                            ))
                        })
                    },
                );
            };
        }
        for_each_roco_type!(register_roco_type_basics);
        crate::types::register_rhai_getters(engine);
        engine.register_get("raw", |value: &mut RocoRequestContext| value.raw.clone());
        engine.register_get("domain", |value: &mut RocoRequestContext| {
            value.domain().to_string()
        });
        engine.register_get("action", |value: &mut RocoRequestContext| {
            value.action().to_string()
        });
        engine.register_get("code", |value: &mut RocoRewardKind| {
            value.code().to_string()
        });
        register_getters!(
            IncubativeMachineEggInfo,
            egg_id,
            incubate_time,
            property,
            catch_time,
            egg_uin,
            egg_name,
            roco_name
        );
        register_getters!(
            IncubativeMachineIncubationInfo,
            egg_type,
            spirit_id,
            egg_id,
            percentile,
            remainder_time,
            stage,
            egg_name,
            property
        );
        register_getters!(
            IncubativeMachineEggListResult,
            result_code,
            message,
            egg_type,
            total_pages,
            current_page
        );
        engine.register_get("eggs", |value: &mut IncubativeMachineEggListResult| {
            Self::to_array(&value.eggs)
        });
        register_getters!(
            IncubativeMachineInfo,
            result_code,
            message,
            guide,
            incubator_type,
            total_pages,
            current_page,
            incubation
        );
        engine.register_get("eggs", |value: &mut IncubativeMachineInfo| {
            Self::to_array(&value.eggs)
        });
        register_getters!(
            IncubativeMachineIncubationResult,
            result_code,
            message,
            incubation
        );
        register_getters!(IncubativeMachineActionResult, result_code, message);
        register_getters!(
            IncubativeMachineGetSpiritResult,
            result_code,
            message,
            spirit_id,
            spirit_level
        );
        register_getters!(
            PetEggSpiritInfo,
            spirit_id,
            level,
            exp_to_next_level,
            personality,
            hp,
            max_hp,
            caught_time,
            caught_location,
            storage_time
        );
        engine.register_get("skills", |value: &mut PetEggSpiritInfo| {
            Self::to_array(&value.skills)
        });
        register_getters!(
            PetEggInfo,
            result_code,
            message,
            current_egg_count,
            max_egg_count,
            vip_count,
            male,
            female,
            egg
        );
        register_getters!(
            PetEggSpeedUpResult,
            result_code,
            message,
            current_egg_count,
            max_egg_count,
            vip_count
        );
        register_getters!(PetEggBeginResult, result_code, message, max_egg_count);
        register_getters!(PetEggCancelResult, result_code, message, detail_code);
        register_getters!(PetEggPreviewResult, result_code, message, egg);
        register_getters!(RemoteSceneData, scene_id, values);
        register_getters!(TaskProgressResult, result_code, message);
        crate::error::register_rhai_types(engine);
    }
}
