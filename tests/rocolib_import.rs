use roco_lang::{
    ActionResult, AquariusBagCandidate, AquariusFirstInfo, AquariusRewardItem,
    AquariusSecondExchangeInfo, CapricornSecondInfo, CapricornSecondTask,
    CapricornTeamOperationInfo, CapricornTeamPlayer, CapricornTeamSnapshot, CapricornThirdInfo,
    DiamondTearInfo, FourSeasonsInfo, IceCrystalBattleInfo, IceCrystalInfo, MultiEvolutionInfo,
    MultiEvolutionRewardItem, Result, RocoAdventureActivityStdLib, RocoAlchemyActivityStdLib,
    RocoAquariusActivityStdLib, RocoAriesActivityStdLib, RocoCancerActivityStdLib,
    RocoCapricornActivityStdLib, RocoCombatStdLib, RocoDebugBreakpoint, RocoDebugCommand,
    RocoDebugConfig, RocoDebugEvent, RocoDebugHooks, RocoDisplayItem, RocoEngine, RocoError,
    RocoErrorDetail, RocoErrorInfo, RocoEvolutionActivityStdLib, RocoGeminiActivityStdLib,
    RocoHomeActivityStdLib, RocoHttpBridgeErrorKind, RocoHttpBusinessRejection,
    RocoIncubativeMachineStdLib, RocoInvalidParamError, RocoLeoActivityStdLib,
    RocoLibraActivityStdLib, RocoLookupStdLib, RocoMagicPioneerActivityStdLib,
    RocoManorActivityStdLib, RocoNetResponseParseSource, RocoNetResponseParseTarget,
    RocoNetworkError, RocoNewsActivityStdLib, RocoPetEggStdLib, RocoPetTrainingActivityStdLib,
    RocoPiscesActivityStdLib, RocoProtocolParseErrorType, RocoProtocolParseFailureKind,
    RocoRemoteStateStdLib, RocoRequestContext, RocoReturnCodeKind, RocoReturnCodeRejection,
    RocoRewardKind, RocoRuntimeStdLib, RocoSagittariusActivityStdLib, RocoScorpioActivityStdLib,
    RocoScriptErrorKind, RocoScriptLocation, RocoServerRejectedError, RocoSpiritBookStdLib,
    RocoSpiritStdLib, RocoSystemStdLib, RocoTaskStdLib, RocoTaurusActivityStdLib,
    RocoThreeStartersActivityStdLib, RocoTowerActivityStdLib, RocoVirgoActivityStdLib,
    SceneRoleInfo, ScriptActivityName, ScriptActivityOperationError, ScriptActivityOptionField,
    ScriptBridgeError, ScriptBridgeFailure, ScriptCombatActionError,
    ScriptCombatCommandFailureKind, ScriptCombatIntentKind, ScriptCombatPhase,
    ScriptCombatProtocolError, ScriptCombatRuntimeError, ScriptCombatWaitError,
    ScriptFunctionContextError, ScriptHttpResponseName, ScriptLookupEntity, ScriptLookupError,
    ScriptModuleName, ScriptPendingResponseError, ScriptQueryError, ScriptRequestError,
    ScriptRequestSystemFailureKind, ScriptResponseError, ScriptResponseName,
    ScriptSessionMemoryError, ScriptSessionValueKind, ScriptStaticDataError, ScriptSystemError,
    ScriptSystemFailure, ScriptSystemFailureSource, ScriptSystemOperation, ScriptWaitContext,
    SpiritBagInfo, SpiritBookEntry, SpiritBookGroup, SpiritBookInfo, SpiritBookStates,
    SpiritBookSummary, SpiritInfo, SpiritSkillInfo, StarTowerInfo, StarTowerTop, StaticSkillInfo,
    StorageSpiritDetailInfo, StorageSpiritInfo, TaskInfo, TaskInfoList, UnicornBossInfo,
    UnicornInfo,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockStdLib {
    calls: Vec<(i64, i64, i64, i64)>,
    bag_count: i64,
    stored_positions: Vec<i64>,
    restored_positions: Vec<i64>,
    swapped_positions: Vec<(i64, i64)>,
    fail_server_time: bool,
}

impl MockStdLib {
    fn spirit_bag(&self) -> SpiritBagInfo {
        SpiritBagInfo {
            spirits: (1..=self.bag_count)
                .map(|position| SpiritInfo {
                    spirit_id: position,
                    position,
                    catch_time: roco_lang::RocoOptionalI64::present(position),
                    name: format!("Spirit {position}"),
                    level: 1,
                    personality: 0,
                    hp: 0,
                    max_hp: 1,
                    skills: Vec::<SpiritSkillInfo>::new(),
                })
                .collect(),
        }
    }
}

impl RocoRuntimeStdLib for MockStdLib {
    fn memory_today(&mut self) -> Result<String> {
        Ok("2026-07-18".to_string())
    }

    fn memory_daily_get_int(&mut self, _key: &str, default_value: i64) -> Result<i64> {
        Ok(default_value + 1)
    }

    fn memory_daily_set_int(&mut self, _key: &str, _value: i64) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_increment_int(&mut self, _key: &str, delta: i64) -> Result<i64> {
        Ok(delta + 10)
    }

    fn memory_daily_get_string(&mut self, _key: &str, default_value: &str) -> Result<String> {
        Ok(format!("{default_value}-stored"))
    }

    fn memory_daily_set_string(&mut self, _key: &str, _value: &str) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_get_bool(&mut self, _key: &str, default_value: bool) -> Result<bool> {
        Ok(!default_value)
    }

    fn memory_daily_set_bool(&mut self, _key: &str, _value: bool) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_delete(&mut self, _key: &str) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_clear(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_list_keys(&mut self) -> Result<Vec<(String, String)>> {
        Ok(vec![("task.count".to_string(), "integer".to_string())])
    }

    fn memory_daily_battle_observed_started(&mut self) -> Result<i64> {
        Ok(11)
    }

    fn memory_daily_battle_observed_completed(&mut self) -> Result<i64> {
        Ok(9)
    }

    fn memory_daily_battle_tracking_since(&mut self) -> Result<i64> {
        Ok(123_456)
    }

    fn memory_daily_battle_limit_reached(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn memory_daily_battle_limit(&mut self) -> Result<i64> {
        Ok(2_000)
    }

    fn memory_daily_battle_limit_return_code(&mut self) -> Result<i64> {
        Ok(41)
    }

    fn memory_daily_battle_limit_message(&mut self) -> Result<String> {
        Ok("limit".to_string())
    }

    fn query_server_time(&mut self) -> Result<roco_lang::ServerTimeInfo> {
        if self.fail_server_time {
            return Err(RocoError::NetworkError(
                roco_lang::RocoNetworkError::HttpBridgeRequestFailed {
                    bridge: roco_lang::RocoBridgeErrorInfo::http(
                        RocoHttpBridgeErrorKind::Timeout,
                        "query_server_time",
                        "bridge timed out",
                    ),
                },
            ));
        }

        Ok(roco_lang::ServerTimeInfo {
            stamp: 1234567890,
            full_year: 2026,
            month: 6,
            date: 19,
            hours: 12,
            minutes: 34,
            seconds: 56,
            day: 5,
            day_of_year: 170,
        })
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

    fn get_storage_spirit_detail(
        &mut self,
        spirit_id: i64,
        catch_time: i64,
    ) -> Result<StorageSpiritDetailInfo> {
        let skill_id = if catch_time == 30 { 203 } else { 105 };
        Ok(StorageSpiritDetailInfo {
            spirit_id,
            catch_time,
            storage_time: 0,
            name: format!("Storage Spirit {spirit_id}"),
            level: 100,
            personality: 0,
            hp: 100,
            max_hp: 100,
            pa: 100,
            pd: 100,
            ma: 100,
            md: 100,
            sp: 100,
            hp_ability: 100,
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
fn persistent_daily_memory_apis_are_available_to_scripts() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                system::assert(memory::today() == "2026-07-18", "date mismatch");
                system::assert(memory::daily_get_int("task.count", 3) == 4, "int get mismatch");
                system::assert(memory::daily_set_int("task.count", 4), "int set mismatch");
                system::assert(memory::daily_increment_int("task.count", 2) == 12, "int increment mismatch");
                system::assert(memory::daily_get_string("task.phase", "new") == "new-stored", "string get mismatch");
                system::assert(memory::daily_set_string("task.phase", "done"), "string set mismatch");
                system::assert(memory::daily_get_bool("task.done", false), "bool get mismatch");
                system::assert(memory::daily_set_bool("task.done", true), "bool set mismatch");
                system::assert(memory::daily_delete("task.done"), "delete mismatch");
                system::assert(memory::daily_clear(), "clear mismatch");
                let keys = memory::daily_list_keys();
                system::assert(keys["task.count"] == "integer", "key list mismatch");
                system::assert(memory::daily_battle_observed_started() == 11, "started mismatch");
                system::assert(memory::daily_battle_observed_completed() == 9, "completed mismatch");
                system::assert(memory::daily_battle_tracking_since() == 123456, "tracking mismatch");
                system::assert(memory::daily_battle_limit_reached(), "limit state mismatch");
                system::assert(memory::daily_battle_limit() == 2000, "limit mismatch");
                system::assert(memory::daily_battle_limit_return_code() == 41, "limit code mismatch");
                system::assert(memory::daily_battle_limit_message() == "limit", "limit message mismatch");
            "#,
        )
        .expect("persistent daily memory APIs should be exposed to scripts");
}

#[test]
fn server_time_is_profile_api_not_scene_api() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib.clone());

    let _ = engine
        .eval(
            r#"
                let server_time = profile::query_server_time();
                system::assert(server_time.stamp == 1234567890, "server time mismatch");
                system::assert(server_time.full_year == 2026, "server year mismatch");
                system::assert(server_time.month == 6, "server month mismatch");
                system::assert(server_time.date == 19, "server date mismatch");

                let result = profile::try_query_server_time();
                system::assert(result.ok, result.message);
                system::assert(result.result.stamp == 1234567890, "try server time mismatch");
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
fn try_result_exposes_structured_error_kind_to_scripts() {
    let stdlib = Arc::new(Mutex::new(MockStdLib {
        fail_server_time: true,
        ..Default::default()
    }));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                let result = profile::try_query_server_time();
                system::assert(!result.ok, "try call should fail");
                system::assert(result.error_kind_code == "network.http_bridge_request_failed", result.error_kind_code);
                system::assert(result.error_network_kind_code == "http_bridge_request_failed", result.error_network_kind_code);
                system::assert(result.error.kind_code == result.error_kind_code, result.error.kind_code);
                system::assert(result.error_detail_kind_code == "bridge", result.error_detail_kind_code);
                system::assert(result.error.detail_kind_code == "bridge", result.error.detail_kind_code);
                system::assert(result.error.detail.kind_code == "bridge", result.error.detail.kind_code);
                system::assert(result.error.network_kind_code == result.error_network_kind_code, result.error.network_kind_code);
                system::assert(result.error_code == "network.http_bridge_request_failed", result.error_code);
                system::assert(result.error_detail.kind_code == "bridge", result.error_detail.kind_code);
                system::assert(result.error_detail.bridge_channel_code == "http", result.error_detail.bridge_channel_code);
                system::assert(result.error_detail.bridge_operation_code == "query_server_time", result.error_detail.bridge_operation_code);
                system::assert(result.error.detail.bridge_channel_code == "http", result.error.detail.bridge_channel_code);
                system::assert(result.error.detail.bridge_operation_code == "query_server_time", result.error.detail.bridge_operation_code);
                system::assert(result.error.detail.bridge_message != "", "detail bridge message should be exposed");
                system::assert(result.error_message != "", "error message should be exposed");
            "#,
        )
        .expect("script should inspect structured error kind");
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
fn optional_values_expose_present_and_value() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let _ = engine
        .eval(
            r#"
                let info = aquarius::first_query();
                let present_reward = info.reward_items[0];
                let missing_reward = info.reward_items[1];
                let candidate = info.bag_candidates[0];

                system::assert(present_reward.item_type.present, "item_type should be present");
                system::assert(present_reward.item_type.value == 7, "present optional value mismatch");
                system::assert(!missing_reward.item_type.present, "item_type should be missing");
                system::assert(missing_reward.item_type.value == (), "missing value should be unit");
                system::assert(candidate.bag_index.present, "bag_index should be present");
                system::assert(candidate.bag_index.value == 3, "bag_index value mismatch");
                system::assert(candidate.catch_time.present, "catch_time should be present");
                system::assert(candidate.catch_time.value == 99, "catch_time value mismatch");
                system::assert(!candidate.level.present, "level should be missing");
                system::assert(candidate.need_money.present, "need_money should be present");
                system::assert(candidate.need_money.value == 1200, "need_money value mismatch");

                let exchange = aquarius::second_exchange_item(1);
                system::assert(exchange.item.present, "display item should be present");
                system::assert(exchange.item.value.item_id == 2001, "display item id mismatch");
                system::assert(exchange.item.value.item_count == 4, "display item count mismatch");
                system::assert(exchange.item.value.item_type == 8, "display item type mismatch");

                let evolved = multi_evolution::fire_evolve(1, 3092, 99, 3, 10);
                system::assert(evolved.pet_id.present, "multi evolution pet_id should be present");
                system::assert(evolved.pet_id.value == 3092, "multi evolution pet_id mismatch");
                system::assert(!evolved.result_side.present, "multi evolution result_side should be missing");
                system::assert(!evolved.item_id.present, "multi evolution item_id should be missing");

                let ice = ice_crystal::query();
                system::assert(ice.progress.present, "ice crystal progress should be present");
                system::assert(ice.progress.value == 12, "ice crystal progress mismatch");
                system::assert(!ice.battle_times.present, "ice crystal battle_times should be missing");
                system::assert(ice.battle_index.value == 2, "ice crystal battle_index mismatch");
                system::assert(ice.current_battle.present, "ice crystal current battle should be present");
                system::assert(ice.current_battle.value.battle_index == 2, "ice crystal current battle index mismatch");
                system::assert(ice.current_battle.value.fight_id == 9001, "ice crystal fight id mismatch");

                let cap2 = capricorn::second_query();
                system::assert(cap2.finish.present, "capricorn second finish should be present");
                system::assert(cap2.finish.value == 1, "capricorn second finish mismatch");
                system::assert(!cap2.current.present, "capricorn second current should be missing");
                system::assert(cap2.position.value == 4, "capricorn second position mismatch");
                system::assert(cap2.second_task.present, "capricorn second task should be present");
                system::assert(cap2.second_task.value.task_type == 2, "capricorn second task type mismatch");
                system::assert(cap2.second_task.value.data1 == 11, "capricorn second task data1 mismatch");

                let cap3 = capricorn::third_query();
                system::assert(!cap3.finish.present, "capricorn third finish should be missing");
                system::assert(cap3.current.value == 5, "capricorn third current mismatch");
                system::assert(cap3.progress_percent.value == 80, "capricorn third progress mismatch");
                system::assert(!cap3.reward_num.present, "capricorn third reward_num should be missing");

                let seasons = four_seasons::query();
                system::assert(seasons.month.present, "four seasons month should be present");
                system::assert(seasons.month.value == 6, "four seasons month mismatch");
                system::assert(!seasons.map.present, "four seasons map should be missing");
                system::assert(seasons.position_1based.value == 3, "four seasons position mismatch");
                system::assert(!seasons.ticket.present, "four seasons ticket should be missing");
                system::assert(seasons.point.value == 99, "four seasons point mismatch");

                let tear = diamond_tear::query();
                system::assert(tear.buy.value == 1, "diamond tear buy mismatch");
                system::assert(!tear.level.present, "diamond tear level should be missing");
                system::assert(tear.count_down.value == 30, "diamond tear count_down mismatch");
                system::assert(tear.tear_state.value == 2, "diamond tear state mismatch");

                let unicorn = unicorn::query();
                system::assert(unicorn.finish.value == 1, "unicorn finish mismatch");
                system::assert(!unicorn.start.present, "unicorn start should be missing");
                system::assert(unicorn.total.value == 8, "unicorn total mismatch");
                system::assert(unicorn.purple_vine_count.value == 6, "unicorn purple vine mismatch");
                system::assert(!unicorn.energy.present, "unicorn energy should be missing");
                system::assert(unicorn.fruit_count.value == 7, "unicorn fruit count mismatch");
                system::assert(unicorn.bosses[0].spirit_id.value == 3001, "unicorn boss spirit mismatch");
                system::assert(!unicorn.bosses[0].fight_id.present, "unicorn boss fight should be missing");
                system::assert(!unicorn.bosses[1].spirit_id.present, "unicorn second boss spirit should be missing");
                system::assert(unicorn.bosses[1].fight_id.value == 9002, "unicorn second boss fight mismatch");

                let tower = star_tower::query();
                system::assert(tower.top.present, "star tower top should be present");
                system::assert(tower.top.value.star == 5, "star tower top star mismatch");
                system::assert(tower.top.value.fight_id == 7001, "star tower top fight mismatch");

                let team_op = capricorn::invite_player(470926678);
                system::assert(team_op.team.present, "capricorn team should be present");
                system::assert(team_op.team.value.ticks == 123, "capricorn team ticks mismatch");
                system::assert(team_op.team.value.players[0].uin == 470926678, "capricorn team player mismatch");
            "#,
        )
        .expect("structured optional int should be readable from script");
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
    assert_eq!(error.kind(), RocoScriptErrorKind::Runtime);
    assert!(error.message().contains("profile::get_user_info"));
    let RocoScriptLocation::Anonymous { position } = error.location else {
        panic!("expected anonymous script error location");
    };
    assert_eq!(position.line(), 3);
    assert!(position.column().is_some());
}

#[test]
fn unsupported_stdlib_error_exposes_structured_function_name() {
    let mut stdlib = MockStdLib::default();
    let error = stdlib
        .get_user_info()
        .expect_err("default stdlib implementation should be unsupported");
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.unsupported.function");
    let RocoErrorDetail::UnsupportedFunction(function) = info.detail else {
        panic!("unsupported function should be structured in detail");
    };
    assert_eq!(function.module, ScriptModuleName::Profile);
    assert_eq!(function.module_code(), "profile");
    assert_eq!(function.function, "get_user_info");
    assert_eq!(function.qualified_name(), "profile::get_user_info");
}

#[test]
fn stdlib_bridge_error_exposes_structured_failure() {
    let error = RocoError::from(ScriptBridgeError::SendRequestFailed {
        failure: ScriptBridgeFailure::send_request(
            roco_lang::ScriptBridgeFailureReason::RequestChannelClosed,
        ),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.bridge");
    let RocoErrorDetail::StdlibBridge(failure) = info.detail else {
        panic!("stdlib bridge failure should be structured in detail");
    };
    assert_eq!(failure.operation_code(), "send_request");
    assert_eq!(failure.reason_code(), "request_channel_closed");
    assert_eq!(failure.message(), "request channel closed");
}

#[test]
fn system_error_exposes_structured_failure() {
    let error = RocoError::from(ScriptSystemError::ParseTimeFormatFailed {
        failure: ScriptSystemFailure::new(
            ScriptSystemOperation::ParseTimeFormat,
            ScriptSystemFailureSource::TimeFormatDescription {
                message: "invalid format".to_string(),
            },
        ),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.system");
    let RocoErrorDetail::SystemFailure(failure) = info.detail else {
        panic!("system failure should be structured in detail");
    };
    assert_eq!(failure.operation_code(), "parse_time_format");
    assert_eq!(failure.source_code(), "time_format_description");
    assert_eq!(failure.message(), "invalid format");
}

#[test]
fn combat_runtime_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptCombatRuntimeError::MarkFrontendLoadedFailed {
        kind: ScriptCombatCommandFailureKind::LineupSkill,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.combat_runtime");
    let RocoErrorDetail::CombatRuntime(runtime) = info.detail else {
        panic!("combat runtime error should be structured in detail");
    };
    assert_eq!(runtime.command_failure_kind_code(), "lineup_skill");
    assert!(runtime.to_string().contains("lineup skill"));
}

#[test]
fn invalid_timestamp_exposes_structured_param_info() {
    let error = RocoError::InvalidParam(RocoInvalidParamError::InvalidTimestamp {
        value: -1,
        failure: ScriptSystemFailure::new(
            ScriptSystemOperation::FormatTimestamp,
            ScriptSystemFailureSource::TimeFormat {
                message: "out of range".to_string(),
            },
        ),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "invalid_param");
    let RocoErrorDetail::InvalidParam(invalid) = info.detail else {
        panic!("invalid param info should be structured in detail");
    };
    assert_eq!(invalid.detail_kind_code(), "system_failure");
    assert_eq!(invalid.kind_code(), "invalid_timestamp");
    assert_eq!(invalid.kind.code(), "invalid_timestamp");
    assert_eq!(invalid.system_operation_code(), "format_timestamp");
    assert_eq!(invalid.system_source_code(), "time_format");
    assert_eq!(invalid.detail.system_operation_code(), "format_timestamp");
    assert_eq!(invalid.detail.system_source_code(), "time_format");
    assert_eq!(invalid.detail.system_message(), "out of range");
}

#[test]
fn rejected_invalid_param_exposes_structured_source() {
    let error = RocoError::InvalidParam(RocoInvalidParamError::Rejected {
        name: "activity option".to_string(),
        source_code: "http_protocol.validation".to_string(),
        message: "field rejected".to_string(),
    });
    let info = error.info();

    let RocoErrorDetail::InvalidParam(invalid) = info.detail else {
        panic!("invalid param info should be structured in detail");
    };
    assert_eq!(invalid.kind_code(), "rejected");
    assert_eq!(invalid.detail_kind_code(), "rejected");
    assert_eq!(invalid.rejected_source_code(), "http_protocol.validation");
    assert_eq!(
        invalid.detail.rejected_source_code(),
        "http_protocol.validation"
    );
}

#[test]
fn error_info_exposes_summary_and_detail_without_flattening_detail_fields() {
    let info = RocoErrorInfo {
        kind: roco_lang::RocoErrorKind::Network {
            kind: roco_lang::RocoNetworkErrorKind::HttpBridgeRequestFailed,
        },
        code: "network.test".to_string(),
        message: "test".to_string(),
        detail: RocoErrorDetail::None,
    };

    assert_eq!(info.detail_kind_code(), "");
    assert_eq!(info.network_kind_code(), "http_bridge_request_failed");
}

#[test]
fn net_response_parse_error_exposes_structured_failure() {
    let protocol = RocoError::NetworkError(RocoNetworkError::NetResponseParseFailed {
        target: RocoNetResponseParseTarget::QuerySpiritBag,
        source: RocoNetResponseParseSource::Protocol {
            kind: RocoProtocolParseFailureKind::Decode,
            layer: RocoProtocolParseErrorType::SpiritBag.layer(),
            error_type: RocoProtocolParseErrorType::SpiritBag,
            reason: roco_lang::RocoProtocolParseReason::ByteArrayReadOverflow {
                offset: 0,
                bytes_needed: 1,
                bytes_available: 0,
            },
        },
    });
    let info = protocol.info();

    assert_eq!(info.kind_code(), "network.net_response_parse_failed");
    assert_eq!(info.network_kind_code(), "net_response_parse_failed");
    let RocoErrorDetail::NetResponseParse(failure) = info.detail else {
        panic!("parse failure should be structured in detail");
    };
    assert_eq!(failure.target, RocoNetResponseParseTarget::QuerySpiritBag);
    assert_eq!(failure.source_kind_code(), "protocol");
    assert_eq!(failure.protocol_error_type_code(), "spirit_bag");
    assert_eq!(failure.protocol_layer_code(), "domain_response");

    let unexpected = RocoError::NetworkError(RocoNetworkError::NetResponseParseFailed {
        target: RocoNetResponseParseTarget::CombatStartPacket,
        source: RocoNetResponseParseSource::UnexpectedCommand { cmd_id: 0x1234 },
    });
    let info = unexpected.info();

    let RocoErrorDetail::NetResponseParse(failure) = info.detail else {
        panic!("unexpected command should be structured in detail");
    };
    assert_eq!(
        failure.target,
        RocoNetResponseParseTarget::CombatStartPacket
    );
    assert_eq!(failure.source_kind_code(), "unexpected_command");
    assert_eq!(failure.unexpected_cmd_id(), 0x1234);
    assert_eq!(failure.protocol_message(), "");
}

#[test]
fn server_return_code_rejection_exposes_structured_fields() {
    let error = RocoError::ServerRejected(RocoServerRejectedError::ReturnCode {
        rejection: RocoReturnCodeRejection {
            kind: RocoReturnCodeKind::SafeCodeRequired,
            message: "need safe code".to_string(),
        },
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "server_rejected");
    assert_eq!(info.code, "server_rejected.return_code");
    let RocoErrorDetail::ReturnCode(rejection) = info.detail else {
        panic!("return code rejection should be structured in detail");
    };
    assert_eq!(rejection.kind_code(), "safe_code_required");
    assert_eq!(rejection.code(), -4);
    assert_eq!(rejection.message, "need safe code");
}

#[test]
fn server_http_business_rejection_exposes_structured_fields() {
    let error = RocoError::ServerRejected(RocoServerRejectedError::HttpBusiness {
        rejection: RocoHttpBusinessRejection {
            result_code: -9,
            message: "not enough item".to_string(),
            request_context: Some(RocoRequestContext::from_raw("aquarius.first")),
        },
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "server_rejected");
    assert_eq!(info.code, "server_rejected.http_business");
    let RocoErrorDetail::HttpBusiness(rejection) = info.detail else {
        panic!("http business rejection should be structured in detail");
    };
    assert_eq!(rejection.result_code(), -9);
    assert_eq!(rejection.request_context(), "aquarius.first");
    assert_eq!(rejection.message, "not enough item");
    let context = rejection
        .request_context
        .expect("http business request context should be structured");
    assert_eq!(context.raw, "aquarius.first");
    assert_eq!(context.domain(), "aquarius");
    assert_eq!(context.action(), "first");
}

#[test]
fn unexpected_http_response_error_exposes_structured_names() {
    let error = ScriptPendingResponseError::UnexpectedHttpResponse {
        pending: ScriptHttpResponseName::new("star_tower.query"),
        expected: ScriptHttpResponseName::new("star_tower"),
        actual: ScriptHttpResponseName::new("blood_gift"),
    };

    assert_eq!(
        error.to_string(),
        "unexpected script http response for star_tower.query: expected star_tower, got blood_gift"
    );
    let ScriptPendingResponseError::UnexpectedHttpResponse {
        pending,
        expected,
        actual,
    } = error
    else {
        unreachable!("constructed unexpected http response error");
    };
    assert_eq!(pending.code, "star_tower.query");
    assert_eq!(expected.code, "star_tower");
    assert_eq!(actual.code, "blood_gift");
}

#[test]
fn pending_response_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptPendingResponseError::UnexpectedHttpResponse {
        pending: ScriptHttpResponseName::new("star_tower.query"),
        expected: ScriptHttpResponseName::new("star_tower"),
        actual: ScriptHttpResponseName::new("blood_gift"),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.pending_response");
    let RocoErrorDetail::PendingResponse(pending) = info.detail else {
        panic!("pending response error should be structured in detail");
    };
    assert_eq!(pending.kind_code(), "unexpected_http_response");
    assert_eq!(pending.pending_http_response_code(), "star_tower.query");
    assert_eq!(pending.expected_http_response_code(), "star_tower");
    assert_eq!(pending.actual_http_response_code(), "blood_gift");

    let error = RocoError::from(ScriptPendingResponseError::MissingNetResponsePayload {
        target: RocoNetResponseParseTarget::WakeupSkills,
    });
    let info = error.info();
    let RocoErrorDetail::PendingResponse(pending) = info.detail else {
        panic!("missing net response payload should be structured in detail");
    };
    assert_eq!(pending.kind_code(), "missing_net_response_payload");
    assert_eq!(pending.net_response_parse_target_code(), "wakeup_skills");
}

#[test]
fn script_response_type_mismatch_exposes_structured_names() {
    let error = ScriptResponseError::TypeMismatch {
        expected: ScriptResponseName::new("bool"),
        actual: ScriptResponseName::new("string"),
    };

    assert_eq!(error.to_string(), "Expected bool response, got string");
    let ScriptResponseError::TypeMismatch { expected, actual } = error;
    assert_eq!(expected.code, "bool");
    assert_eq!(actual.code, "string");
}

#[test]
fn script_response_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptResponseError::TypeMismatch {
        expected: ScriptResponseName::new("bool"),
        actual: ScriptResponseName::new("string"),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.response");
    let RocoErrorDetail::Response(response) = info.detail else {
        panic!("script response error should be structured in detail");
    };
    assert_eq!(response.kind_code(), "type_mismatch");
    assert_eq!(response.expected_response_code(), "bool");
    assert_eq!(response.actual_response_code(), "string");
}

#[test]
fn request_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptRequestError::WaitStateRejected {
        context: ScriptWaitContext::TalentRefresh,
        kind: ScriptRequestSystemFailureKind::NoRunningScript,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.request");
    let RocoErrorDetail::Request(request) = info.detail else {
        panic!("request error should be structured in detail");
    };
    assert_eq!(request.kind_code(), "wait_state_rejected");
    assert_eq!(request.wait_context_code(), "talent_refresh");
    assert_eq!(request.system_failure_kind_code(), "no_running_script");
    assert_eq!(request.combat_intent_kind_code(), "");
    assert_eq!(request.spirit_index(), -1);
    assert_eq!(request.value(), -1);
}

#[test]
fn request_combat_protocol_error_exposes_structured_detail_fields() {
    let error = RocoError::from(ScriptRequestError::InvalidCombatIntent {
        kind: ScriptCombatIntentKind::ChangeSpirit,
        spirit_index: 2,
        value: 8,
        error: ScriptCombatProtocolError::TargetSpiritPositionOutOfRange { value: 8 },
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.request");
    let RocoErrorDetail::Request(request) = info.detail else {
        panic!("request error should be structured in detail");
    };
    assert_eq!(request.kind_code(), "invalid_combat_intent");
    assert_eq!(request.combat_intent_kind_code(), "change_spirit");
    assert_eq!(
        request.combat_protocol_error_kind_code(),
        "target_spirit_position_out_of_range"
    );
    assert_eq!(request.combat_protocol_error_value(), 8);
    assert_eq!(request.spirit_index(), 2);
    assert_eq!(request.value(), 8);
}

#[test]
fn session_memory_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptSessionMemoryError::TypeMismatch {
        key: "talent.target".to_string(),
        expected: ScriptSessionValueKind::Integer,
        actual: ScriptSessionValueKind::String,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.session_memory");
    let RocoErrorDetail::SessionMemory(memory) = info.detail else {
        panic!("session memory error should be structured in detail");
    };
    assert_eq!(memory.kind_code(), "type_mismatch");
    assert_eq!(memory.key(), "talent.target");
    assert_eq!(memory.expected_kind_code(), "integer");
    assert_eq!(memory.actual_kind_code(), "string");
}

#[test]
fn lookup_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptLookupError::NotFound {
        entity: ScriptLookupEntity::ItemInfo,
        key: "17367045".to_string(),
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.lookup");
    let RocoErrorDetail::Lookup(lookup) = info.detail else {
        panic!("lookup error should be structured in detail");
    };
    assert_eq!(lookup.kind_code(), "not_found");
    assert_eq!(lookup.entity_code(), "item_info");
    assert_eq!(lookup.key(), "17367045");
}

#[test]
fn function_context_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptFunctionContextError::CannotWaitForCombat {
        phase: ScriptCombatPhase::WaitingPlayerAction,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.function_context");
    let RocoErrorDetail::FunctionContext(context) = info.detail else {
        panic!("function context error should be structured in detail");
    };
    assert_eq!(context.kind_code(), "cannot_wait_for_combat");
    assert_eq!(context.combat_phase_code(), "waiting_player_action");
}

#[test]
fn query_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptQueryError::NoSkillAtIndex { index: 2 });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.query");
    let RocoErrorDetail::Query(query) = info.detail else {
        panic!("query error should be structured in detail");
    };
    assert_eq!(query.kind_code(), "no_skill_at_index");
    assert_eq!(query.skill_index(), 2);
}

#[test]
fn static_data_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptStaticDataError::StaticGameDataNotInitialized);
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.static_data");
    let RocoErrorDetail::StaticData(static_data) = info.detail else {
        panic!("static data error should be structured in detail");
    };
    assert_eq!(static_data.kind_code(), "static_game_data_not_initialized");
    assert_eq!(static_data.function_name(), "");
    assert_eq!(static_data.message(), "");
    assert_eq!(static_data.active_config_source_code(), "");

    let error = RocoError::from(ScriptStaticDataError::ActiveConfigNotAvailable {
        source: roco_lang::ScriptActiveConfigUnavailableSource::Download,
        message: "download failed".to_string(),
    });
    let info = error.info();
    let RocoErrorDetail::StaticData(static_data) = info.detail else {
        panic!("static data error should be structured in detail");
    };
    assert_eq!(static_data.kind_code(), "active_config_not_available");
    assert_eq!(
        static_data.active_config_source_code(),
        "static_data.download"
    );
    assert_eq!(static_data.message(), "download failed");
}

#[test]
fn activity_operation_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptActivityOperationError::InvalidOption {
        activity: ScriptActivityName::Gemini,
        field: ScriptActivityOptionField::Side,
        value: 99,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.activity_operation");
    let RocoErrorDetail::ActivityOperation(operation) = info.detail else {
        panic!("activity operation error should be structured in detail");
    };
    assert_eq!(operation.kind_code(), "invalid_option");
    assert_eq!(operation.activity_code(), "gemini");
    assert_eq!(operation.field_code(), "side");
    assert_eq!(operation.value(), 99);
    assert_eq!(operation.count(), -1);
    assert_eq!(operation.limit(), -1);
}

#[test]
fn combat_action_error_exposes_structured_detail() {
    let error =
        RocoError::from(ScriptCombatActionError::CombatItemUnavailable { item_id: 17367045 });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.combat_action");
    let RocoErrorDetail::CombatAction(action) = info.detail else {
        panic!("combat action error should be structured in detail");
    };
    assert_eq!(action.kind_code(), "combat_item_unavailable");
    assert_eq!(action.item_id(), 17367045);
    assert_eq!(action.position(), -1);
    assert_eq!(action.skill_id(), -1);
}

#[test]
fn combat_wait_error_exposes_structured_detail() {
    let error = RocoError::from(ScriptCombatWaitError::TimeoutWaitingCombatAction {
        phase: ScriptCombatPhase::WaitingPlayerAction,
        elapsed_ms: 12_345,
    });
    let info = error.info();

    assert_eq!(info.kind_code(), "stdlib");
    assert_eq!(info.code, "stdlib.combat_wait");
    let RocoErrorDetail::CombatWait(wait) = info.detail else {
        panic!("combat wait error should be structured in detail");
    };
    assert_eq!(wait.kind_code(), "timeout_waiting_combat_action");
    assert_eq!(wait.combat_phase_code(), "waiting_player_action");
    assert_eq!(wait.elapsed_ms(), 12_345);
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

impl RocoSystemStdLib for MockStdLib {
    fn random_int(&mut self, min_inclusive: i64, max_inclusive: i64) -> Result<i64> {
        assert!(min_inclusive <= max_inclusive);
        Ok(max_inclusive)
    }
}

#[test]
fn system_random_int_is_available_to_scripts_with_inclusive_bounds() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let result = engine
        .eval("system::random_int(-3, 7)")
        .expect("system::random_int should be registered");

    assert_eq!(result.as_int().expect("int result"), 7);
}

impl RocoManorActivityStdLib for MockStdLib {}

impl RocoHomeActivityStdLib for MockStdLib {}

impl RocoPetTrainingActivityStdLib for MockStdLib {}

impl RocoNewsActivityStdLib for MockStdLib {}

impl RocoTaskStdLib for MockStdLib {
    fn task_query_info_list(&mut self) -> Result<TaskInfoList> {
        Ok(TaskInfoList {
            result_code: 0,
            message: String::new(),
            tasks: vec![TaskInfo {
                story_id: 10,
                task_id: 20,
                status: 1,
                task_type: 2,
                task_type_sub: 3,
                theme_id: 4,
            }],
        })
    }
}

#[test]
fn task_results_expose_domain_getters() {
    let stdlib = Arc::new(Mutex::new(MockStdLib::default()));
    let mut engine = RocoEngine::new(stdlib);

    let result = engine
        .eval(
            r#"
                let info = task::query_info_list();
                info.result_code + info.tasks[0].task_id
            "#,
        )
        .expect("task DTO getters should be registered");

    assert_eq!(result.as_int().expect("int result"), 20);
}

impl RocoIncubativeMachineStdLib for MockStdLib {}

impl RocoPetEggStdLib for MockStdLib {}

impl RocoRemoteStateStdLib for MockStdLib {}

impl RocoTowerActivityStdLib for MockStdLib {
    fn star_tower_query(&mut self) -> Result<StarTowerInfo> {
        Ok(StarTowerInfo {
            result_code: 0,
            message: String::new(),
            mop: 1,
            boss_id: 2,
            countdown: 3,
            auto_sell: true,
            money: 4,
            clips: Vec::new(),
            storeys: Vec::new(),
            top: roco_lang::RocoOptionalStarTowerTop::present(StarTowerTop {
                star: 5,
                refresh: 6,
                fight_desc: "fight".to_string(),
                task_desc: "task".to_string(),
                fight_id: 7001,
                tokens: Vec::new(),
                exchanges: Vec::new(),
                missions: Vec::new(),
                rewards: Vec::new(),
            }),
        })
    }
}

impl RocoAlchemyActivityStdLib for MockStdLib {}

impl RocoEvolutionActivityStdLib for MockStdLib {
    fn unicorn_query(&mut self) -> Result<UnicornInfo> {
        Ok(UnicornInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("unicorn.query"),
            bosses: vec![
                UnicornBossInfo {
                    slot: 1,
                    npc_index: 10,
                    spirit_id: roco_lang::RocoOptionalI64::present(3001),
                    fight_id: roco_lang::RocoOptionalI64::missing(),
                },
                UnicornBossInfo {
                    slot: 2,
                    npc_index: 20,
                    spirit_id: roco_lang::RocoOptionalI64::missing(),
                    fight_id: roco_lang::RocoOptionalI64::present(9002),
                },
            ],
            finish: roco_lang::RocoOptionalI64::present(1),
            start: roco_lang::RocoOptionalI64::missing(),
            total: roco_lang::RocoOptionalI64::present(8),
            book: roco_lang::RocoOptionalI64::missing(),
            cultivation_times: Vec::new(),
            evolution_energy_costs: Vec::new(),
            one_key_diamond_costs: Vec::new(),
            purple_vine_count: roco_lang::RocoOptionalI64::present(6),
            energy: roco_lang::RocoOptionalI64::missing(),
            fruit_count: roco_lang::RocoOptionalI64::present(7),
            increase: roco_lang::RocoOptionalI64::present(2),
            bag_candidates: Vec::new(),
            rewards: Vec::new(),
        })
    }

    fn four_seasons_query(&mut self) -> Result<FourSeasonsInfo> {
        Ok(FourSeasonsInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("four_seasons.query"),
            month: roco_lang::RocoOptionalI64::present(6),
            map: roco_lang::RocoOptionalI64::missing(),
            position_1based: roco_lang::RocoOptionalI64::present(3),
            times: roco_lang::RocoOptionalI64::present(2),
            ticket: roco_lang::RocoOptionalI64::missing(),
            used_tool_index: roco_lang::RocoOptionalI64::present(4),
            need_item_index: roco_lang::RocoOptionalI64::missing(),
            add: roco_lang::RocoOptionalI64::present(5),
            point: roco_lang::RocoOptionalI64::present(99),
            boxes: Vec::new(),
            tools: Vec::new(),
            tool_shop_indexes: Vec::new(),
            tool_shop_flags: Vec::new(),
            pass_boxes: Vec::new(),
            tool_costs: Vec::new(),
            event_item_counts: Vec::new(),
            shop_rewards: Vec::new(),
            monthly_spirit_rewards: Vec::new(),
            rewards: Vec::new(),
        })
    }

    fn diamond_tear_query(&mut self) -> Result<DiamondTearInfo> {
        Ok(DiamondTearInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("diamond_tear.query"),
            buy: roco_lang::RocoOptionalI64::present(1),
            level: roco_lang::RocoOptionalI64::missing(),
            count_down: roco_lang::RocoOptionalI64::present(30),
            tear_state: roco_lang::RocoOptionalI64::present(2),
            rewards: Vec::new(),
        })
    }

    fn ice_crystal_query(&mut self) -> Result<IceCrystalInfo> {
        Ok(IceCrystalInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("ice_crystal.query"),
            progress: roco_lang::RocoOptionalI64::present(12),
            battle_times: roco_lang::RocoOptionalI64::missing(),
            battle_index: roco_lang::RocoOptionalI64::present(2),
            get_times: roco_lang::RocoOptionalI64::missing(),
            add: roco_lang::RocoOptionalI64::present(3),
            item_counts: Vec::new(),
            crystal_counts: Vec::new(),
            item_costs: Vec::new(),
            one_key_diamond_costs: Vec::new(),
            current_battle: roco_lang::RocoOptionalIceCrystalBattleInfo::present(
                IceCrystalBattleInfo {
                    battle_index: 2,
                    fight_id: 9001,
                },
            ),
            bag_candidates: Vec::new(),
            rewards: Vec::new(),
        })
    }

    fn multi_evolution_fire_evolve(
        &mut self,
        _slot: i64,
        _spirit_id: i64,
        _catch_time: i64,
        _item_count: i64,
        _fire_score: i64,
    ) -> Result<MultiEvolutionInfo> {
        Ok(MultiEvolutionInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("multi_evolution.fire_evolve"),
            candidates: Vec::new(),
            rewards: vec![MultiEvolutionRewardItem {
                reward_id: 1,
                reward_kind: RocoRewardKind::Item,
                raw_reward_type: 0,
                count: 2,
            }],
            pet_id: roco_lang::RocoOptionalI64::present(3092),
            result_side: roco_lang::RocoOptionalI64::missing(),
            item_id: roco_lang::RocoOptionalI64::missing(),
            count: 0,
            available: false,
        })
    }
}

impl RocoMagicPioneerActivityStdLib for MockStdLib {}

impl RocoAdventureActivityStdLib for MockStdLib {}
impl roco_lang::RocoFriendStdLib for MockStdLib {}

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
impl RocoAquariusActivityStdLib for MockStdLib {
    fn aquarius_first_query(&mut self) -> Result<AquariusFirstInfo> {
        Ok(AquariusFirstInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("aquarius.first_query"),
            fields: Vec::new(),
            counters: Vec::new(),
            item_counts: Vec::new(),
            states: Vec::new(),
            bag_candidates: vec![AquariusBagCandidate {
                candidate_index: 0,
                spirit_id: roco_lang::RocoOptionalI64::present(3092),
                bag_index: roco_lang::RocoOptionalI64::present(3),
                catch_time: roco_lang::RocoOptionalI64::present(99),
                level: roco_lang::RocoOptionalI64::missing(),
                need_money: roco_lang::RocoOptionalI64::present(1200),
            }],
            reward_items: vec![
                AquariusRewardItem {
                    item_index: 1,
                    item_id: 1001,
                    count: 2,
                    item_type: roco_lang::RocoOptionalI64::present(7),
                },
                AquariusRewardItem {
                    item_index: 2,
                    item_id: 1002,
                    count: 3,
                    item_type: roco_lang::RocoOptionalI64::missing(),
                },
            ],
        })
    }

    fn aquarius_second_exchange_item(
        &mut self,
        _exchange_position: i64,
    ) -> Result<AquariusSecondExchangeInfo> {
        Ok(AquariusSecondExchangeInfo {
            result_code: 0,
            message: String::new(),
            item: roco_lang::RocoOptionalDisplayItem::present(RocoDisplayItem {
                item_id: 2001,
                item_count: 4,
                item_type: 8,
            }),
            light_num: 1,
            tail_num: 2,
            exchange_count0: 3,
            exchange_count1: 4,
        })
    }
}
impl RocoCapricornActivityStdLib for MockStdLib {
    fn capricorn_invite_player(&mut self, _uin: i64) -> Result<CapricornTeamOperationInfo> {
        Ok(CapricornTeamOperationInfo {
            result_code: 0,
            message: String::new(),
            team: roco_lang::RocoOptionalCapricornTeamSnapshot::present(CapricornTeamSnapshot {
                players: vec![CapricornTeamPlayer {
                    uin: 470926678,
                    nick: "Target".to_string(),
                }],
                ticks: 123,
            }),
        })
    }

    fn capricorn_second_query(&mut self) -> Result<CapricornSecondInfo> {
        Ok(CapricornSecondInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("capricorn.second_query"),
            finish: roco_lang::RocoOptionalI64::present(1),
            current: roco_lang::RocoOptionalI64::missing(),
            position: roco_lang::RocoOptionalI64::present(4),
            second_task: roco_lang::RocoOptionalCapricornSecondTask::present(CapricornSecondTask {
                task_type: 2,
                data1: 11,
                data2: 12,
                step: 3,
                current: 1,
            }),
            bag_candidates: Vec::new(),
        })
    }

    fn capricorn_third_query(&mut self) -> Result<CapricornThirdInfo> {
        Ok(CapricornThirdInfo {
            result_code: 0,
            message: String::new(),
            request_context: RocoRequestContext::from_raw("capricorn.third_query"),
            finish: roco_lang::RocoOptionalI64::missing(),
            current: roco_lang::RocoOptionalI64::present(5),
            remain: roco_lang::RocoOptionalI64::present(6),
            price: roco_lang::RocoOptionalI64::missing(),
            limit: roco_lang::RocoOptionalI64::present(7),
            progress_percent: roco_lang::RocoOptionalI64::present(80),
            reward_num: roco_lang::RocoOptionalI64::missing(),
            tips: roco_lang::RocoOptionalI64::present(9),
            bag_candidates: Vec::new(),
        })
    }
}
