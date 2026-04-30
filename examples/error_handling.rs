use roco_lang::{
    BattleInfo, BattleResult, CombatActions, Result, RocoEngine, RocoError, RocoStdLib,
    RoundResult, SkillInfo, SpiritBagInfo, SpiritInfo, StaticItemInfo, StaticSkillInfo,
    StaticSpiritInfo,
};
use std::sync::{Arc, Mutex};

/// Mock implementation that intentionally fails in some scenarios.
struct ErrorTestStdLib {
    should_fail: bool,
}

impl ErrorTestStdLib {
    fn new() -> Self {
        Self { should_fail: false }
    }
}

impl RocoStdLib for ErrorTestStdLib {
    fn move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<i64> {
        if self.should_fail {
            Err(RocoError::ServerRejected(
                "move_to_scene rejected".to_string(),
            ))
        } else {
            println!("Moving to scene {} (timeout: {}ms)", scene_id, timeout_ms);
            Ok(scene_id)
        }
    }

    fn get_current_scene(&mut self) -> Result<i64> {
        Ok(1)
    }

    fn fetch_spirit(&mut self, _catch_time: i64) -> Result<bool> {
        Ok(true)
    }

    fn fetch_spirit_by_id(&mut self, _spirit_id: i64) -> Result<bool> {
        Ok(true)
    }

    fn challenge_wild_spirit(&mut self, _spirit_id: i64) -> Result<bool> {
        Ok(true)
    }

    fn clear_lineup(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn store_spirit(&mut self, _position: i64) -> Result<bool> {
        Ok(true)
    }

    fn get_spirit_bag(&mut self) -> Result<SpiritBagInfo> {
        Ok(SpiritBagInfo { spirits: vec![] })
    }

    fn get_combat_lineup(&mut self) -> Result<Vec<SpiritInfo>> {
        Ok(vec![])
    }

    fn learn_skill(&mut self, _position: i64, _skill_id: i64) -> Result<bool> {
        Ok(true)
    }

    fn get_skills(&mut self, _position: i64) -> Result<[Option<SkillInfo>; 4]> {
        Ok([None, None, None, None])
    }

    fn equip_item(&mut self, _position: i64, _item_name: &str) -> Result<bool> {
        Ok(true)
    }

    fn lookup_item_info(&mut self, item_id: i64) -> Result<StaticItemInfo> {
        Err(RocoError::StdLibError(format!(
            "item info not found: {item_id}"
        )))
    }

    fn lookup_skill_info(&mut self, skill_id: i64) -> Result<StaticSkillInfo> {
        Err(RocoError::StdLibError(format!(
            "skill info not found: {skill_id}"
        )))
    }

    fn lookup_spirit_info(&mut self, spirit_id: i64) -> Result<StaticSpiritInfo> {
        Err(RocoError::StdLibError(format!(
            "spirit info not found: {spirit_id}"
        )))
    }

    fn invite_pk(&mut self, target_uin: i64) -> Result<BattleInfo> {
        Ok(BattleInfo {
            battle_id: "test".to_string(),
            my_uin: 12345,
            rival_uin: target_uin,
            started: true,
        })
    }

    fn accept_pk(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn reject_pk(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn use_skill(&mut self, _skill_id: i64) -> Result<bool> {
        Ok(true)
    }

    fn use_item(&mut self, _item_id: i64) -> Result<bool> {
        Ok(true)
    }

    fn change_spirit(&mut self, _position: i64) -> Result<bool> {
        Ok(true)
    }

    fn defend(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn escape(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn wait_round_end(&mut self) -> Result<RoundResult> {
        Ok(RoundResult {
            round: 1,
            my_hp: 100,
            rival_hp: 100,
            finished: false,
        })
    }

    fn get_battle_result(&mut self) -> Result<BattleResult> {
        Ok(BattleResult {
            winner: None,
            total_rounds: 0,
        })
    }

    fn get_combat_actions(&mut self) -> Result<CombatActions> {
        Ok(CombatActions {
            can_submit_action: true,
            can_use_skill: true,
            can_capture: true,
            can_use_item: true,
            can_change_spirit: false,
            can_escape: true,
            can_use_any_skill: false,
            can_change_to_any_spirit: false,
            can_combat_mask: 31,
        })
    }

    fn can_use_skill(&mut self, _skill_id: i64) -> Result<bool> {
        Ok(false)
    }

    fn can_use_item(&mut self, _item_id: i64) -> Result<bool> {
        Ok(false)
    }

    fn can_change_to_spirit(&mut self, _position: i64) -> Result<bool> {
        Ok(false)
    }

    fn can_capture(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn get_battle_history(&mut self) -> Result<String> {
        Ok("{}".to_string())
    }

    fn get_my_hp(&mut self) -> Result<i64> {
        if self.should_fail {
            Err(RocoError::StdLibError("HP query failed".to_string()))
        } else {
            Ok(100)
        }
    }

    fn get_my_max_hp(&mut self) -> Result<i64> {
        Ok(100)
    }

    fn get_rival_hp(&mut self) -> Result<i64> {
        Ok(100)
    }

    fn get_rival_max_hp(&mut self) -> Result<i64> {
        Ok(100)
    }

    fn get_my_pp(&mut self, _slot: i64) -> Result<i64> {
        Ok(10)
    }

    fn get_my_power_skill(&mut self) -> Result<i64> {
        Ok(101)
    }

    fn get_my_spirit_info(&mut self, _position: i64) -> Result<SpiritInfo> {
        Ok(SpiritInfo {
            spirit_id: 1,
            position: _position,
            catch_time: 0,
            name: "Test".to_string(),
            level: 1,
            hp: 100,
            max_hp: 100,
            skills: Vec::new(),
        })
    }

    fn get_rival_spirit_info(&mut self) -> Result<SpiritInfo> {
        Ok(SpiritInfo {
            spirit_id: 2,
            position: 1,
            catch_time: 0,
            name: "Rival".to_string(),
            level: 1,
            hp: 100,
            max_hp: 100,
            skills: Vec::new(),
        })
    }

    fn is_combat_finished(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn get_current_round(&mut self) -> Result<i64> {
        Ok(0)
    }

    fn sleep(&mut self, _ms: i64) -> Result<()> {
        Ok(())
    }

    fn log(&mut self, message: &str) -> Result<()> {
        println!("[LOG] {}", message);
        Ok(())
    }

    fn assert(&mut self, condition: bool, message: &str) -> Result<()> {
        if !condition {
            Err(RocoError::AssertionError(message.to_string()))
        } else {
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let stdlib = Arc::new(Mutex::new(ErrorTestStdLib::new()));
    let mut engine = RocoEngine::new(stdlib.clone());

    println!("=== Test 1: Normal execution ===");
    let script1 = r#"
        log("Test normal execution");
        move_to_scene(42, 5000);
        let hp = get_my_hp();
        log("HP: " + hp);
        true
    "#;

    match engine.eval(script1) {
        Ok(result) => println!("Script succeeded: {:?}\n", result),
        Err(e) => println!("Script failed: {}\n", e),
    }

    println!("=== Test 2: Error handling with try-catch ===");

    // Enable the failure flag.
    stdlib.lock().unwrap().should_fail = true;

    let script2 = r#"
        log("Test error handling");

        try {
            move_to_scene(99, 5000);
            log("This should not print");
        } catch (err) {
            log("Caught error: " + err);
        }

        log("Script continues after error");
        true
    "#;

    match engine.eval(script2) {
        Ok(result) => println!("Script succeeded with error handling: {:?}\n", result),
        Err(e) => println!("Script failed: {}\n", e),
    }

    println!("=== Test 3: Unhandled error ===");
    let script3 = r#"
        log("Test unhandled error");
        get_my_hp();  // This will fail.
        log("This should not print");
    "#;

    match engine.eval(script3) {
        Ok(result) => println!("Script should have failed but succeeded: {:?}\n", result),
        Err(e) => println!("Script failed as expected: {}\n", e),
    }

    println!("=== Test 4: Assert failure ===");
    stdlib.lock().unwrap().should_fail = false;

    let script4 = r#"
        log("Test assert");
        assert(1 + 1 == 2, "Math works");
        log("First assert passed");

        try {
            assert(1 + 1 == 3, "Math is broken!");
        } catch (err) {
            log("Caught assertion error: " + err);
        }

        true
    "#;

    match engine.eval(script4) {
        Ok(result) => println!("Assert test succeeded: {:?}\n", result),
        Err(e) => println!("Assert test failed: {}\n", e),
    }

    Ok(())
}
