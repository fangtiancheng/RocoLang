use roco_lang::{
    ActionResult, Result, RocoAdventureActivityStdLib, RocoAlchemyActivityStdLib,
    RocoAquariusActivityStdLib, RocoAriesActivityStdLib, RocoCancerActivityStdLib,
    RocoCombatStdLib, RocoDebugBreakpoint, RocoDebugCommand, RocoDebugConfig, RocoDebugEvent,
    RocoDebugHooks, RocoEngine, RocoError, RocoEvolutionActivityStdLib, RocoGeminiActivityStdLib,
    RocoLeoActivityStdLib, RocoLibraActivityStdLib, RocoLookupStdLib,
    RocoMagicPioneerActivityStdLib, RocoManorActivityStdLib, RocoNewsActivityStdLib,
    RocoPiscesActivityStdLib, RocoRuntimeStdLib, RocoSagittariusActivityStdLib,
    RocoScorpioActivityStdLib, RocoScriptErrorKind, RocoScriptLocation, RocoSpiritBookStdLib,
    RocoSpiritStdLib, RocoSystemStdLib, RocoTaurusActivityStdLib, RocoThreeStartersActivityStdLib,
    RocoTowerActivityStdLib, RocoVirgoActivityStdLib, SceneRoleInfo, SpiritBagInfo,
    SpiritBookEntry, SpiritBookGroup, SpiritBookInfo, SpiritBookStates, SpiritBookSummary,
    SpiritInfo, SpiritSkillInfo, StaticSkillInfo, StorageSpiritInfo,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockStdLib {
    calls: Vec<(i64, i64, i64, i64)>,
    bag_count: i64,
    stored_positions: Vec<i64>,
    restored_positions: Vec<i64>,
    swapped_positions: Vec<(i64, i64)>,
}

impl MockStdLib {
    fn spirit_bag(&self) -> SpiritBagInfo {
        SpiritBagInfo {
            spirits: (1..=self.bag_count)
                .map(|position| SpiritInfo {
                    spirit_id: position,
                    position,
                    catch_time: position,
                    name: format!("Spirit {position}"),
                    level: 1,
                    hp: 0,
                    max_hp: 1,
                    skills: Vec::<SpiritSkillInfo>::new(),
                })
                .collect(),
        }
    }
}

impl RocoRuntimeStdLib for MockStdLib {
    fn query_server_time(&mut self) -> Result<i64> {
        Ok(1234567890)
    }

    fn get_cached_scene_roles(&mut self) -> Result<Vec<SceneRoleInfo>> {
        Ok(vec![SceneRoleInfo {
            uin: 470926678,
            id: 470926678,
            nick_name: "Target".to_string(),
            level: 100,
            loc_x: 10,
            loc_y: 20,
            pk_state: 0,
            is_in_combat: false,
            is_vip: true,
            vip_level: 5,
            trainer_level: 20,
            trainer_exp: 1234,
        }])
    }
}

impl RocoSpiritStdLib for MockStdLib {
    fn start_combat(
        &mut self,
        server_type: i64,
        combat_type: i64,
        rival_id: i64,
        catch_time: i64,
    ) -> Result<bool> {
        self.calls
            .push((server_type, combat_type, rival_id, catch_time));
        Ok(true)
    }

    fn get_spirit_bag(&mut self) -> Result<SpiritBagInfo> {
        Ok(self.spirit_bag())
    }

    fn try_store_spirit(&mut self, position: i64) -> Result<ActionResult> {
        self.stored_positions.push(position);
        if self.bag_count > 0 {
            self.bag_count -= 1;
        }
        Ok(ActionResult::ok())
    }

    fn try_restore_spirit(&mut self, _spirit_id: i64, position: i64) -> Result<ActionResult> {
        self.restored_positions.push(position);
        Ok(ActionResult::ok())
    }

    fn swap_spirits(&mut self, first_position: i64, second_position: i64) -> Result<bool> {
        self.swapped_positions
            .push((first_position, second_position));
        Ok(true)
    }

    fn list_storage_spirits(&mut self) -> Result<Vec<StorageSpiritInfo>> {
        Ok(vec![
            StorageSpiritInfo {
                spirit_id: 49,
                catch_time: 10,
                storage_time: 100,
                level: 99,
                sex: 0,
                skin_flag: 0,
                talent_type: 0,
                talent_level: 0,
            },
            StorageSpiritInfo {
                spirit_id: 49,
                catch_time: 20,
                storage_time: 200,
                level: 100,
                sex: 0,
                skin_flag: 0,
                talent_type: 0,
                talent_level: 0,
            },
            StorageSpiritInfo {
                spirit_id: 49,
                catch_time: 30,
                storage_time: 150,
                level: 100,
                sex: 0,
                skin_flag: 0,
                talent_type: 0,
                talent_level: 0,
            },
            StorageSpiritInfo {
                spirit_id: 2928,
                catch_time: 40,
                storage_time: 300,
                level: 100,
                sex: 0,
                skin_flag: 0,
                talent_type: 0,
                talent_level: 0,
            },
        ])
    }

    fn list_abandoned_storage_spirits(&mut self) -> Result<Vec<StorageSpiritInfo>> {
        Ok(vec![StorageSpiritInfo {
            spirit_id: 2,
            catch_time: 50,
            storage_time: 500,
            level: 1,
            sex: 0,
            skin_flag: 0,
            talent_type: 0,
            talent_level: 0,
        }])
    }

    fn get_storage_spirit_detail(&mut self, spirit_id: i64, catch_time: i64) -> Result<SpiritInfo> {
        let skill_id = if catch_time == 30 { 203 } else { 105 };
        Ok(SpiritInfo {
            spirit_id,
            position: 0,
            catch_time,
            name: format!("Storage Spirit {spirit_id}"),
            level: 100,
            hp: 100,
            max_hp: 100,
            skills: vec![SpiritSkillInfo {
                skill_id,
                pp: 10,
                inherited: false,
            }],
        })
    }
}

impl RocoLookupStdLib for MockStdLib {
    fn list_spirit_book_summaries(&mut self) -> Result<Vec<SpiritBookSummary>> {
        Ok(vec![SpiritBookSummary {
            id: 10,
            name: "All Spirits".to_string(),
            is_new: false,
            has_cover: true,
            background: "book-bg".to_string(),
            page_idx: 0,
            spirit_count: 2,
        }])
    }

    fn get_spirit_book(&mut self, book_id: i64) -> Result<SpiritBookInfo> {
        Ok(SpiritBookInfo {
            id: book_id,
            name: "All Spirits".to_string(),
            is_new: false,
            has_cover: true,
            background: "book-bg".to_string(),
            page_idx: 0,
            groups: vec![SpiritBookGroup {
                template_id: 1,
                spirits: vec![
                    SpiritBookEntry {
                        id: 1,
                        starred: true,
                        unknown: false,
                        newed: false,
                    },
                    SpiritBookEntry {
                        id: 2,
                        starred: false,
                        unknown: false,
                        newed: true,
                    },
                ],
            }],
        })
    }

    fn lookup_skills_info(&mut self, skill_ids: Vec<i64>) -> Result<Vec<StaticSkillInfo>> {
        Ok(skill_ids
            .into_iter()
            .map(|id| StaticSkillInfo {
                id,
                name: format!("Skill {id}"),
                description: String::new(),
                description2: String::new(),
                power: "0".to_string(),
                pp_max: 0,
                property: 0,
                src: String::new(),
                attack_type: 0,
                speed: 0,
                damage_type: 0,
                catch_rate: 0,
                super_form_id: 0,
                super_form_src: String::new(),
            })
            .collect())
    }
}

impl RocoSpiritBookStdLib for MockStdLib {
    fn get_my_spirit_book_states(&mut self) -> Result<SpiritBookStates> {
        Ok(SpiritBookStates {
            uin: 470926678,
            count: 3,
            states: vec![1, 2, 3],
        })
    }

    fn get_role_spirit_book_states(&mut self, uin: i64) -> Result<SpiritBookStates> {
        Ok(SpiritBookStates {
            uin,
            count: 2,
            states: vec![0, 3],
        })
    }
}

#[test]
fn server_time_is_profile_api_not_scene_api() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                let stamp = profile::query_server_time();
                system::assert(stamp == 1234567890, "server time mismatch");

                let result = profile::try_query_server_time();
                system::assert(result.ok, result.message);
                system::assert(result.message == "1234567890", result.message);
            "#,
        )
        .expect("server time should be exposed under profile");

    let error = engine
        .eval("scene::query_server_time();")
        .expect_err("server time should not be exposed under scene");
    assert!(
        error.to_string().contains("query_server_time"),
        "unexpected error: {error}"
    );
}

#[test]
fn spirit_book_apis_are_available_to_scripts() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                let summaries = lookup::list_spirit_book_summaries();
                system::assert(len(summaries) == 1, "summary count mismatch");
                system::assert(summaries[0].id == 10, "summary id mismatch");
                system::assert(summaries[0].spirit_count == 2, "summary count field mismatch");

                let book = lookup::get_spirit_book(10);
                system::assert(book.name == "All Spirits", book.name);
                system::assert(len(book.groups) == 1, "group count mismatch");
                system::assert(book.groups[0].template_id == 1, "template id mismatch");

                let entries = lookup::list_spirit_book_entries(10);
                system::assert(len(entries) == 2, "entry count mismatch");
                system::assert(entries[0].id == 1, "entry id mismatch");
                system::assert(entries[0].starred, "entry starred mismatch");
                system::assert(entries[1].newed, "entry newed mismatch");

                let states = spirit_book::get_my_states();
                system::assert(states.uin == 470926678, "state uin mismatch");
                system::assert(states.count == 3, "state count mismatch");
                system::assert(states.states[1] == spirit_book_state::CAUGHT, "state value mismatch");

                let my_state = spirit_book::get_my_spirit_state(2);
                system::assert(my_state.spirit_id == 2, "spirit state id mismatch");
                system::assert(my_state.state == spirit_book_state::CAUGHT, "my state mismatch");

                let role_state = spirit_book::get_role_spirit_state(123, 2);
                system::assert(role_state.state == spirit_book_state::RELEASED, "role state mismatch");
            "#,
        )
        .expect("spirit book APIs should be exposed to scripts");
}

#[test]
fn imports_built_in_spirit_swap_helpers() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                spirit::swap_spirits(1, 2);
                let result = spirit::try_swap_spirits(2, 3);
                system::assert(result.ok, result.message);
            "#,
        )
        .expect("built-in spirit swap helpers should import and run");

    let stdlib = stdlib.lock().expect("stdlib lock");
    assert_eq!(stdlib.swapped_positions, &[(1, 2), (2, 3)]);
}

#[test]
fn imports_built_in_combat_helpers() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                import "roco/combat" as roco_combat;

                roco_combat::challenge_wild_spirit(577);
                roco_combat::challenge_wild_spirit_with_catch_time(578, 12);
                roco_combat::challenge_boss(9001);
                roco_combat::challenge_boss_with_catch_time(9002, 34);
            "#,
        )
        .expect("built-in combat helpers should import and run");

    let calls = &stdlib.lock().expect("stdlib lock").calls;
    assert_eq!(
        calls,
        &[
            (0, 1, 577, 0),
            (0, 1, 578, 12),
            (0, 2, 9001, 0),
            (0, 2, 9002, 34),
        ]
    );
}

#[test]
fn imports_built_in_role_cache_helpers() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                import "roco/role" as roco_role;

                let roles = role::get_cached_scene_roles();
                system::assert(len(roles) == 1, "cached role count mismatch");
                system::assert(roles[0].uin == 470926678, "cached role uin mismatch");
                system::assert(!roles[0].is_in_combat, "cached role combat state mismatch");

                let found = roco_role::find_cached_scene_role(470926678);
                system::assert(found.found, "target role must be found in cache");
                system::assert(found.role.nick_name == "Target", found.role.nick_name);

                let missing = roco_role::find_cached_scene_role(1);
                system::assert(!missing.found, "missing role must not be found");
            "#,
        )
        .expect("built-in role cache helpers should import and run");
}

#[test]
fn stdlib_runtime_error_reports_call_location() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);
    let error = engine
        .eval(
            r#"
                let before = 1;
                profile::get_user_info();
            "#,
        )
        .expect_err("unsupported stdlib call should fail");

    let RocoError::ScriptError(error) = error else {
        panic!("expected script error");
    };
    assert_eq!(error.kind, RocoScriptErrorKind::Runtime);
    assert!(error.message.contains("profile::get_user_info"));
    let RocoScriptLocation::Anonymous { position } = error.location else {
        panic!("expected anonymous script error location");
    };
    assert_eq!(position.line(), 3);
    assert!(position.column().is_some());
}

#[test]
fn imports_built_in_spirit_helpers() {
    let stdlib = Arc::new(Mutex::new(MockStdLib {
        bag_count: 3,
        ..Default::default()
    }));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                import "roco/spirit" as roco_spirit;

                let result = roco_spirit::try_store_all_spirits();
                system::assert(result.ok, result.message);
            "#,
        )
        .expect("built-in spirit helpers should import and run");

    let stdlib = stdlib.lock().expect("stdlib lock");
    assert_eq!(stdlib.bag_count, 0);
    assert_eq!(stdlib.stored_positions, &[3, 2, 1]);
}

#[test]
fn built_in_spirit_recovery_helpers_are_try_style() {
    let stdlib = Arc::new(Mutex::new(MockStdLib {
        bag_count: 2,
        ..Default::default()
    }));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                import "roco/spirit" as roco_spirit;

                let result = roco_spirit::try_recover_all_spirits_money();
                system::assert(result.ok, result.message);
            "#,
        )
        .expect("built-in spirit recovery helpers should import and run");

    let stdlib = stdlib.lock().expect("stdlib lock");
    assert_eq!(stdlib.restored_positions, &[1, 2]);
}

#[test]
fn built_in_spirit_ready_helper_returns_try_style_result() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                import "roco/spirit" as roco_spirit;

                let spirit_info = #{
                    spirit_id: 49,
                    position: 1,
                    catch_time: 1,
                    name: "Black Rock",
                    level: 100,
                    hp: 100,
                    max_hp: 100,
                    skills: [
                        #{ skill_id: 203, pp: 10, inherited: false },
                        #{ skill_id: 105, pp: 10, inherited: false }
                    ]
                };

                let ready = roco_spirit::is_spirit_ready(spirit_info, 49, [203, 105]);
                system::assert(ready.ok, ready.message);
                system::assert(ready.code == 0, "ready code mismatch");

                let wrong_id = roco_spirit::is_spirit_ready(spirit_info, 2928, [203]);
                system::assert(!wrong_id.ok, "wrong id must fail");
                system::assert(wrong_id.code == 1, "wrong id code mismatch");

                let missing_skill = roco_spirit::is_spirit_ready(spirit_info, 49, [203, 2647]);
                system::assert(!missing_skill.ok, "missing skill must fail");
                system::assert(missing_skill.code == 2, "missing skill code mismatch");

                let skill_requirements = roco_spirit::format_skill_requirements([203, 2647]);
                system::assert(skill_requirements == "[Skill 203(203),Skill 2647(2647)]", skill_requirements);
            "#,
        )
        .expect("built-in spirit ready helper should import and run");
}

#[test]
fn built_in_select_best_storage_spirit_prefers_ready_level_100_spirit() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                import "roco/spirit" as roco_spirit;

                let best = roco_spirit::select_best_storage_spirit(49, [203]);
                system::assert(best.found, "best storage spirit must be found");
                system::assert(best.spirit_id == 49, "best spirit id mismatch");
                system::assert(best.catch_time == 30, "best catch_time mismatch: " + best.catch_time);
                system::assert(best.level == 100, "best level mismatch");
                system::assert(best.has_required_skills, "best must have required skills");

                let missing = roco_spirit::select_best_storage_spirit(9999, [203]);
                system::assert(!missing.found, "missing spirit must not be found");
            "#,
        )
        .expect("built-in select_best_storage_spirit should import and run");
}

#[test]
fn debug_runner_pauses_on_line_breakpoint_and_reports_call_stack() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);
    let events = Arc::new(Mutex::new(Vec::<RocoDebugEvent>::new()));
    let events_for_hook = events.clone();

    let result = engine
        .eval_debug(
            r#"
                fn value() {
                    let x = 1;
                    x + 1
                }

                value()
            "#,
            RocoDebugConfig {
                source: Some("debug_test.rhai".to_string()),
                breakpoints: vec![RocoDebugBreakpoint {
                    source: Some("debug_test.rhai".to_string()),
                    line: 4,
                    column: None,
                    enabled: true,
                }],
            },
            RocoDebugHooks::new(
                move |event| {
                    events_for_hook.lock().expect("events lock").push(event);
                },
                || RocoDebugCommand::Continue,
            ),
        )
        .expect("debug runner should complete");

    assert_eq!(result.as_int().expect("int result"), 2);

    let events = events.lock().expect("events lock");
    assert!(matches!(events.first(), Some(RocoDebugEvent::Started)));
    assert!(
        events
            .iter()
            .any(|event| matches!(event, RocoDebugEvent::Ended)),
        "debug session should report end"
    );

    let paused = events
        .iter()
        .find_map(|event| match event {
            RocoDebugEvent::Paused {
                reason,
                location,
                stack,
                locals,
                ..
            } => {
                let (_, line, _) = location.parts();
                Some((reason, location.source(), line, stack, locals))
            }
            _ => None,
        })
        .expect("line breakpoint should pause");

    assert_eq!(paused.0, "breakpoint:0");
    assert_eq!(paused.1, Some("debug_test.rhai"));
    assert_eq!(paused.2, Some(4));
    assert!(
        paused.3.iter().any(|frame| frame.function_name == "value"),
        "pause event should include function call stack"
    );
    assert!(
        paused
            .4
            .iter()
            .any(|local| local.name == "x" && local.value_preview == "1"),
        "pause event should include local variables"
    );
}

#[test]
fn debug_runner_previews_custom_type_locals_as_json() {
    let stdlib = Arc::new(Mutex::new(MockStdLib {
        bag_count: 1,
        ..Default::default()
    }));
    let mut engine = RocoEngine::new(stdlib);
    let events = Arc::new(Mutex::new(Vec::<RocoDebugEvent>::new()));
    let events_for_hook = events.clone();

    let _ = engine
        .eval_debug(
            r#"
                let bag = spirit::get_spirit_bag();
                let count = len(bag.spirits);
                count
            "#,
            RocoDebugConfig {
                source: Some("debug_custom_type.rhai".to_string()),
                breakpoints: vec![RocoDebugBreakpoint {
                    source: Some("debug_custom_type.rhai".to_string()),
                    line: 3,
                    column: None,
                    enabled: true,
                }],
            },
            RocoDebugHooks::new(
                move |event| {
                    events_for_hook.lock().expect("events lock").push(event);
                },
                || RocoDebugCommand::Continue,
            ),
        )
        .expect("debug runner should complete");

    let bag_preview = events
        .lock()
        .expect("events lock")
        .iter()
        .find_map(|event| match event {
            RocoDebugEvent::Paused { locals, .. } => locals
                .iter()
                .find(|local| local.name == "bag")
                .map(|local| local.value_preview.clone()),
            _ => None,
        })
        .expect("pause event should include bag local");

    assert!(
        bag_preview.contains("\"spirits\""),
        "custom type preview should include serialized fields, got {bag_preview}"
    );
    assert!(
        bag_preview.contains("Spirit 1"),
        "custom type preview should include nested values, got {bag_preview}"
    );
}

#[test]
fn debug_runner_can_update_breakpoints_while_paused() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);
    let events = Arc::new(Mutex::new(Vec::<RocoDebugEvent>::new()));
    let events_for_hook = events.clone();
    let commands = Arc::new(Mutex::new(vec![
        RocoDebugCommand::SetBreakpoints(vec![RocoDebugBreakpoint {
            source: Some("debug_update.rhai".to_string()),
            line: 4,
            column: None,
            enabled: true,
        }]),
        RocoDebugCommand::Continue,
        RocoDebugCommand::Continue,
    ]));
    let commands_for_hook = commands.clone();

    let result = engine
        .eval_debug(
            r#"
                let a = 1;
                let b = 2;
                a + b
            "#,
            RocoDebugConfig {
                source: Some("debug_update.rhai".to_string()),
                breakpoints: vec![RocoDebugBreakpoint {
                    source: Some("debug_update.rhai".to_string()),
                    line: 2,
                    column: None,
                    enabled: true,
                }],
            },
            RocoDebugHooks::new(
                move |event| {
                    events_for_hook.lock().expect("events lock").push(event);
                },
                move || {
                    let mut commands = commands_for_hook.lock().expect("commands lock");
                    if commands.is_empty() {
                        RocoDebugCommand::Continue
                    } else {
                        commands.remove(0)
                    }
                },
            ),
        )
        .expect("debug runner should complete after breakpoint update");

    assert_eq!(result.as_int().expect("int result"), 3);

    let paused_lines = events
        .lock()
        .expect("events lock")
        .iter()
        .filter_map(|event| match event {
            RocoDebugEvent::Paused { location, .. } => location.parts().1,
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(paused_lines.first(), Some(&2));
    assert!(
        paused_lines.contains(&4),
        "updated breakpoint should pause on line 4, got {paused_lines:?}"
    );
}

#[test]
fn imported_rocolib_frames_expose_stable_source_id() {
    let stdlib = Arc::new(Mutex::new(MockStdLib {
        bag_count: 1,
        ..Default::default()
    }));
    let mut engine = RocoEngine::new(stdlib);
    let events = Arc::new(Mutex::new(Vec::<RocoDebugEvent>::new()));
    let events_for_hook = events.clone();

    let _ = engine
        .eval_debug(
            r#"
                import "roco/spirit" as roco_spirit;

                roco_spirit::try_store_all_spirits();
            "#,
            RocoDebugConfig {
                source: Some("main.rhai".to_string()),
                breakpoints: vec![RocoDebugBreakpoint {
                    source: Some("roco/spirit".to_string()),
                    line: 206,
                    column: None,
                    enabled: true,
                }],
            },
            RocoDebugHooks::new(
                move |event| {
                    events_for_hook.lock().expect("events lock").push(event);
                },
                || RocoDebugCommand::Continue,
            ),
        )
        .expect("debug runner should pause inside imported module");

    let paused_sources = events
        .lock()
        .expect("events lock")
        .iter()
        .filter_map(|event| match event {
            RocoDebugEvent::Paused { location, .. } => location.source().map(ToString::to_string),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(
        paused_sources.iter().any(|source| source == "roco/spirit"),
        "expected imported module source id, got {paused_sources:?}"
    );
}

#[test]
fn enum_like_modules_expose_combat_constants() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let result = engine
        .eval(
            r#"
                weather::MIASMA
                    + combat_status::SLEEP
                    + combat_result::WIN
                    + len(weather::name(weather::MIASMA))
                    + len(combat_status::name(combat_status::SLEEP))
                    + len(combat_result::name(combat_result::WIN))
            "#,
        )
        .expect("enum-like constants should be available");

    assert!(result.as_int().expect("int result") > 11);
}

impl RocoCombatStdLib for MockStdLib {}

impl RocoSystemStdLib for MockStdLib {}

impl RocoManorActivityStdLib for MockStdLib {}

impl RocoNewsActivityStdLib for MockStdLib {}

impl RocoTowerActivityStdLib for MockStdLib {}

impl RocoAlchemyActivityStdLib for MockStdLib {}

impl RocoEvolutionActivityStdLib for MockStdLib {}

impl RocoMagicPioneerActivityStdLib for MockStdLib {}

impl RocoAdventureActivityStdLib for MockStdLib {}

impl RocoAriesActivityStdLib for MockStdLib {}

impl RocoLibraActivityStdLib for MockStdLib {}

impl RocoLeoActivityStdLib for MockStdLib {}

impl RocoCancerActivityStdLib for MockStdLib {}

impl RocoVirgoActivityStdLib for MockStdLib {}

impl RocoPiscesActivityStdLib for MockStdLib {}

impl RocoTaurusActivityStdLib for MockStdLib {}

impl RocoThreeStartersActivityStdLib for MockStdLib {}

impl RocoGeminiActivityStdLib for MockStdLib {}

impl RocoSagittariusActivityStdLib for MockStdLib {}

impl RocoScorpioActivityStdLib for MockStdLib {}

impl RocoAquariusActivityStdLib for MockStdLib {}
