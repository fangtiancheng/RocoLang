use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "combat",
            "invite_pk",
            return_type: "BattleInfo",
            "向指定玩家发起 PK 邀请。",
            params: ["target_uin" => "目标玩家 uin。"],
            returns: "战斗邀请信息。",
            examples: ["let battle = combat::invite_pk(123456);"]
        ),
        super::stdlib_doc!(
            "combat",
            "accept_pk",
            return_type: "bool",
            "接受当前 PK 邀请。",
            params: [],
            returns: "接受成功返回 true。",
            examples: ["combat::accept_pk();"]
        ),
        super::stdlib_doc!(
            "combat",
            "reject_pk",
            return_type: "bool",
            "拒绝当前 PK 邀请。",
            params: [],
            returns: "拒绝成功返回 true。",
            examples: ["combat::reject_pk();"]
        ),
        super::stdlib_doc!(
            "combat",
            "start_combat",
            return_type: "bool",
            "发起一场战斗。",
            params: ["server_type" => "服务器类型。", "combat_type" => "战斗类型。", "rival_id" => "对手或 Boss ID。", "catch_time" => "捕获时间，通常为 0。"],
            returns: "发起成功返回 true。",
            examples: ["combat::start_combat(0, 1, 242, 0);"]
        ),
        super::stdlib_doc!(
            "combat",
            "use_skill",
            return_type: "bool",
            "在当前战斗回合提交指定技能。",
            params: ["skill_id" => "技能 ID。"],
            returns: "提交成功返回 true。",
            examples: ["combat::use_skill(497);"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_use_skill",
            return_type: "ActionResult",
            "尝试提交指定技能，失败时返回结构化结果。",
            params: ["skill_id" => "技能 ID。"],
            returns: "结构化操作结果。",
            examples: ["let result = combat::try_use_skill(497);"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_use_skill_and_wait",
            return_type: "CombatActionResult",
            "尝试使用技能，并等待战斗进入下一次可行动状态或结束。",
            params: ["skill_id" => "技能 ID。"],
            returns: "包含战斗是否结束、是否进入下一次行动的结构化结果。",
            examples: ["let result = combat::try_use_skill_and_wait(497);"]
        ),
        super::stdlib_doc!(
            "combat",
            "use_item",
            return_type: "bool",
            "在当前战斗回合使用指定道具。",
            params: ["item_id" => "道具 ID。"],
            returns: "提交成功返回 true。",
            examples: ["combat::use_item(300000);"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_use_item",
            return_type: "ActionResult",
            "尝试使用指定战斗道具，失败时返回结构化结果。",
            params: ["item_id" => "道具 ID。"],
            returns: "结构化操作结果。",
            examples: ["let result = combat::try_use_item(300000);"]
        ),
        super::stdlib_doc!(
            "combat",
            "change_spirit",
            return_type: "bool",
            "切换到指定背包位置的精灵。",
            params: ["position" => "背包精灵位置，从 1 开始。"],
            returns: "提交成功返回 true。",
            examples: ["combat::change_spirit(2);"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_change_spirit",
            return_type: "ActionResult",
            "尝试切换精灵，失败时返回结构化结果。",
            params: ["position" => "背包精灵位置，从 1 开始。"],
            returns: "结构化操作结果。",
            examples: ["let result = combat::try_change_spirit(2);"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_change_spirit_and_wait",
            return_type: "CombatActionResult",
            "尝试切换精灵，并等待战斗进入下一次可行动状态或结束。",
            params: ["position" => "背包精灵位置，从 1 开始。"],
            returns: "包含战斗是否结束、是否进入下一次行动的结构化结果。",
            examples: ["let result = combat::try_change_spirit_and_wait(2);"]
        ),
        super::stdlib_doc!(
            "combat",
            "combat_escape",
            return_type: "bool",
            "从当前战斗逃跑。",
            params: [],
            returns: "提交成功返回 true。",
            examples: ["combat::combat_escape();"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_combat_escape",
            return_type: "ActionResult",
            "尝试逃跑，失败时返回结构化结果。",
            params: [],
            returns: "结构化操作结果。",
            examples: ["let result = combat::try_combat_escape();"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_combat_escape_and_wait",
            return_type: "CombatActionResult",
            "尝试逃跑，并等待战斗进入下一次状态或结束。",
            params: [],
            returns: "包含战斗是否结束、是否进入下一次行动的结构化结果。",
            examples: ["let result = combat::try_combat_escape_and_wait();"]
        ),
        super::stdlib_doc!(
            "combat",
            "wait_round_end",
            return_type: "RoundResult",
            "等待当前回合结算结束。",
            params: [],
            returns: "回合结算结果。",
            examples: ["let round = combat::wait_round_end();"]
        ),
        super::stdlib_doc!(
            "combat",
            "wait_next_action",
            return_type: "bool",
            "等待战斗进入下一次可行动状态，或战斗结束。",
            params: [],
            returns: "进入下一次可行动状态或战斗已结束时返回 true。",
            examples: ["combat::wait_next_action();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_battle_result",
            return_type: "BattleResult",
            "查询最近一次战斗结果。",
            params: [],
            returns: "战斗结果。",
            examples: ["let result = combat::get_battle_result();"]
        ),
        super::stdlib_doc!(
            "combat",
            "try_get_battle_result",
            return_type: "BattleResultQueryResult",
            "尝试查询战斗结果，战斗结果暂不可用时返回结构化结果。",
            params: [],
            returns: "战斗结果查询结果。",
            examples: ["let result = combat::try_get_battle_result();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_combat_actions",
            return_type: "CombatActions",
            "获取当前可用战斗动作。",
            params: [],
            returns: "当前可用战斗动作。",
            examples: ["let actions = combat::get_combat_actions();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_combat_lineup",
            return_type: "SpiritInfo[]",
            "获取当前战斗队伍中已有精灵信息。",
            params: [],
            returns: "战斗队伍精灵数组。",
            examples: ["let lineup = combat::get_combat_lineup();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_combat_state",
            return_type: "CombatState",
            "获取底层战斗状态机快照。",
            params: [],
            returns: "战斗状态快照。",
            examples: ["let state = combat::get_combat_state();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_action_snapshot",
            return_type: "CombatActionSnapshot",
            "一次性获取战斗是否结束、状态机和可用动作。",
            params: [],
            returns: "战斗动作快照。",
            examples: ["let snapshot = combat::get_action_snapshot();"]
        ),
        super::stdlib_doc!(
            "combat",
            "can_use_skill",
            return_type: "bool",
            "判断当前是否可以使用指定技能。",
            params: ["skill_id" => "技能 ID。"],
            returns: "可使用返回 true。",
            examples: ["if combat::can_use_skill(497) { combat::use_skill(497); }"]
        ),
        super::stdlib_doc!(
            "combat",
            "can_use_item",
            return_type: "bool",
            "判断当前是否可以使用指定战斗道具。",
            params: ["item_id" => "道具 ID。"],
            returns: "可使用返回 true。",
            examples: ["let ok = combat::can_use_item(300000);"]
        ),
        super::stdlib_doc!(
            "combat",
            "can_change_to_spirit",
            return_type: "bool",
            "判断当前是否可以切换到指定背包位置的精灵。",
            params: ["position" => "背包精灵位置，从 1 开始。"],
            returns: "可切换返回 true。",
            examples: ["let ok = combat::can_change_to_spirit(2);"]
        ),
        super::stdlib_doc!(
            "combat",
            "can_capture",
            return_type: "bool",
            "判断当前战斗是否可以捕捉。",
            params: [],
            returns: "可捕捉返回 true。",
            examples: ["if combat::can_capture() { system::log(\"can catch\"); }"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_battle_history",
            return_type: "string",
            "获取当前或最近战斗的历史记录文本。",
            params: [],
            returns: "战斗历史文本。",
            examples: ["system::log(combat::get_battle_history());"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_my_hp",
            return_type: "int",
            "获取我方当前出战精灵 HP。",
            params: [],
            returns: "当前 HP。",
            examples: ["let hp = combat::get_my_hp();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_my_max_hp",
            return_type: "int",
            "获取我方当前出战精灵最大 HP。",
            params: [],
            returns: "最大 HP。",
            examples: ["let max_hp = combat::get_my_max_hp();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_rival_hp",
            return_type: "int",
            "获取敌方当前出战精灵 HP。",
            params: [],
            returns: "当前 HP。",
            examples: ["let hp = combat::get_rival_hp();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_rival_max_hp",
            return_type: "int",
            "获取敌方当前出战精灵最大 HP。",
            params: [],
            returns: "最大 HP。",
            examples: ["let max_hp = combat::get_rival_max_hp();"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_my_pp",
            return_type: "int",
            "获取我方当前出战精灵指定技能槽 PP。",
            params: ["slot" => "技能槽位，从 1 开始。"],
            returns: "当前 PP。",
            examples: ["let pp = combat::get_my_pp(1);"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_my_spirit_info",
            return_type: "SpiritInfo",
            "获取我方指定背包位置精灵信息。",
            params: ["position" => "背包精灵位置，从 1 开始。"],
            returns: "精灵信息。",
            examples: ["let spirit = combat::get_my_spirit_info(1);"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_rival_spirit_info",
            return_type: "SpiritInfo",
            "获取敌方当前出战精灵信息。",
            params: [],
            returns: "敌方精灵信息。",
            examples: ["let rival = combat::get_rival_spirit_info();"]
        ),
        super::stdlib_doc!(
            "combat",
            "is_combat_finished",
            return_type: "bool",
            "判断当前战斗是否已经结束。",
            params: [],
            returns: "战斗结束返回 true。",
            examples: ["if combat::is_combat_finished() { break; }"]
        ),
        super::stdlib_doc!(
            "combat",
            "get_current_round",
            return_type: "int",
            "获取当前战斗回合数。",
            params: [],
            returns: "当前回合数。",
            examples: ["let round = combat::get_current_round();"]
        ),
    ]
}
