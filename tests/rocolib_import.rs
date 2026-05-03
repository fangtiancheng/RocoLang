use roco_lang::{Result, RocoEngine, RocoStdLib};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockStdLib {
    calls: Vec<(i64, i64, i64, i64)>,
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
