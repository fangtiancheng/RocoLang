use roco_lang::{
    ActionResult, Result, RocoEngine, RocoStdLib, SpiritBagInfo, SpiritInfo, SpiritSkillInfo,
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockStdLib {
    calls: Vec<(i64, i64, i64, i64)>,
    bag_count: i64,
    stored_positions: Vec<i64>,
    restored_positions: Vec<i64>,
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

impl RocoStdLib for MockStdLib {
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
