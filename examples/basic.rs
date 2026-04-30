use roco_lang::{
    BattleInfo, BattleResult, Result, RocoEngine, RocoStdLib, RoundResult, SkillInfo,
    SpiritBagInfo, SpiritInfo,
};
use std::sync::{Arc, Mutex};

/// Mock implementation used by examples.
struct MockStdLib {
    scene_id: i64,
    my_hp: i64,
    rival_hp: i64,
    round: i64,
}

impl MockStdLib {
    fn new() -> Self {
        Self {
            scene_id: 1,
            my_hp: 100,
            rival_hp: 100,
            round: 0,
        }
    }
}

impl RocoStdLib for MockStdLib {
    fn move_to_scene(&mut self, scene_id: i64, timeout_ms: i64) -> Result<i64> {
        println!("Moving to scene {} (timeout: {}ms)", scene_id, timeout_ms);
        self.scene_id = scene_id;
        Ok(scene_id)
    }

    fn get_current_scene(&mut self) -> Result<i64> {
        Ok(self.scene_id)
    }

    fn fetch_spirit(&mut self, catch_time: i64) -> Result<bool> {
        println!("Fetching spirit with catch_time {}", catch_time);
        Ok(true)
    }

    fn fetch_spirit_by_id(&mut self, spirit_id: i64) -> Result<bool> {
        println!("Fetching spirit with id {}", spirit_id);
        Ok(true)
    }

    fn challenge_wild_spirit(&mut self, spirit_id: i64) -> Result<bool> {
        println!("Challenging wild spirit {}", spirit_id);
        Ok(true)
    }

    fn clear_lineup(&mut self) -> Result<bool> {
        println!("Clearing lineup");
        Ok(true)
    }

    fn store_spirit(&mut self, position: i64) -> Result<bool> {
        println!("Storing spirit at position {}", position);
        Ok(true)
    }

    fn get_spirit_bag(&mut self) -> Result<SpiritBagInfo> {
        Ok(SpiritBagInfo {
            spirits: vec![SpiritInfo {
                catch_time: 123456,
                name: "Fire Spirit".to_string(),
                level: 50,
                hp: 100,
                max_hp: 100,
            }],
        })
    }

    fn get_lineup(&mut self) -> Result<Vec<SpiritInfo>> {
        Ok(vec![])
    }

    fn get_lineup_count(&mut self) -> Result<i64> {
        Ok(1)
    }

    fn learn_skill(&mut self, position: i64, skill_id: i64) -> Result<bool> {
        println!("Learning skill {} at position {}", skill_id, position);
        Ok(true)
    }

    fn get_skills(&mut self, position: i64) -> Result<[Option<SkillInfo>; 4]> {
        println!("Getting skills at position {}", position);
        Ok([
            Some(SkillInfo {
                skill_id: 101,
                skill_name: "Flame Strike".to_string(),
                pp: 10,
                max_pp: 15,
            }),
            Some(SkillInfo {
                skill_id: 102,
                skill_name: "Flame Storm".to_string(),
                pp: 5,
                max_pp: 10,
            }),
            None,
            None,
        ])
    }

    fn equip_item(&mut self, position: i64, item_name: &str) -> Result<bool> {
        println!("Equipping {} at position {}", item_name, position);
        Ok(true)
    }

    fn invite_pk(&mut self, target_uin: i64) -> Result<BattleInfo> {
        println!("Inviting PK with {}", target_uin);
        Ok(BattleInfo {
            battle_id: "test_battle".to_string(),
            my_uin: 12345,
            rival_uin: target_uin,
            started: true,
        })
    }

    fn accept_pk(&mut self) -> Result<bool> {
        println!("Accepting PK");
        Ok(true)
    }

    fn reject_pk(&mut self) -> Result<bool> {
        println!("Rejecting PK");
        Ok(true)
    }

    fn use_skill(&mut self, skill_id: i64) -> Result<bool> {
        println!("Using skill {}", skill_id);
        self.rival_hp -= 20;
        Ok(true)
    }

    fn use_item(&mut self, item_name: &str) -> Result<bool> {
        println!("Using item {}", item_name);
        self.my_hp += 30;
        if self.my_hp > 100 {
            self.my_hp = 100;
        }
        Ok(true)
    }

    fn change_spirit(&mut self, position: i64) -> Result<bool> {
        println!("Changing to spirit at position {}", position);
        Ok(true)
    }

    fn defend(&mut self) -> Result<bool> {
        println!("Defending");
        Ok(true)
    }

    fn escape(&mut self) -> Result<bool> {
        println!("Escaping");
        Ok(true)
    }

    fn wait_round_end(&mut self) -> Result<RoundResult> {
        self.round += 1;
        println!("Round {} ended", self.round);
        Ok(RoundResult {
            round: self.round,
            my_hp: self.my_hp,
            rival_hp: self.rival_hp,
            finished: self.is_finished().unwrap(),
        })
    }

    fn get_battle_result(&mut self) -> Result<BattleResult> {
        Ok(BattleResult {
            winner: if self.rival_hp <= 0 {
                Some(12345)
            } else if self.my_hp <= 0 {
                Some(67890)
            } else {
                None
            },
            total_rounds: self.round,
        })
    }

    fn get_battle_history(&mut self) -> Result<String> {
        Ok("{}".to_string())
    }

    fn get_my_hp(&mut self) -> Result<i64> {
        Ok(self.my_hp)
    }

    fn get_my_max_hp(&mut self) -> Result<i64> {
        Ok(100)
    }

    fn get_rival_hp(&mut self) -> Result<i64> {
        Ok(self.rival_hp)
    }

    fn get_rival_max_hp(&mut self) -> Result<i64> {
        Ok(100)
    }

    fn get_my_pp(&mut self, slot: i64) -> Result<i64> {
        println!("Getting PP for slot {}", slot);
        Ok(10)
    }

    fn get_my_power_skill(&mut self) -> Result<i64> {
        Ok(101)
    }

    fn get_my_spirit_info(&mut self, position: i64) -> Result<SpiritInfo> {
        println!("Getting spirit info at position {}", position);
        Ok(SpiritInfo {
            catch_time: 123456,
            name: "Fire Spirit".to_string(),
            level: 50,
            hp: self.my_hp,
            max_hp: 100,
        })
    }

    fn get_rival_spirit_info(&mut self) -> Result<SpiritInfo> {
        Ok(SpiritInfo {
            catch_time: 0,
            name: "Rival Spirit".to_string(),
            level: 50,
            hp: self.rival_hp,
            max_hp: 100,
        })
    }

    fn is_finished(&mut self) -> Result<bool> {
        Ok(self.rival_hp <= 0 || self.my_hp <= 0)
    }

    fn get_current_round(&mut self) -> Result<i64> {
        Ok(self.round)
    }

    fn sleep(&mut self, ms: i64) -> Result<()> {
        println!("Sleeping for {}ms", ms);
        Ok(())
    }

    fn log(&mut self, message: &str) -> Result<()> {
        println!("[LOG] {}", message);
        Ok(())
    }

    fn assert(&mut self, condition: bool, message: &str) -> Result<()> {
        if !condition {
            return Err(roco_lang::RocoError::AssertionError(message.to_string()));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let stdlib = Arc::new(Mutex::new(MockStdLib::new()));
    let mut engine = RocoEngine::new(stdlib);

    // Simple battle script.
    let script = r#"
        log("Starting battle script");

        // Move to the battle scene.
        move_to_scene(42, 5000);

        // Battle loop.
        let round = 0;
        while !is_finished() && round < 10 {
            log("Round " + round);

            let my_hp = get_my_hp();
            let rival_hp = get_rival_hp();

            log("My HP: " + my_hp + ", Rival HP: " + rival_hp);

            if my_hp < 50 {
                log("HP low, using item");
                use_item("heal_potion");
            } else {
                log("Using attack skill");
                use_skill(101);
            }

            wait_round_end();
            round += 1;
        }

        log("Battle finished");
        is_finished()
    "#;

    let result = engine.eval(script)?;
    println!("\nScript result: {:?}", result);

    Ok(())
}
