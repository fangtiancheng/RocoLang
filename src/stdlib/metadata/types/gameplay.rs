use super::{field, try_error_fields, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "ActionResult" => ("操作型 try_* 接口的标准返回结构。", {
            let mut fields = vec![
                field("ok", "bool", "操作是否成功。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示业务失败。"),
                field(
                    "message",
                    "string",
                    "失败原因或服务器返回信息，成功时通常为空。",
                ),
                field(
                    "error",
                    "RocoErrorInfo?",
                    "结构化错误信息；成功时为空，动作不可用或执行失败时通常包含具体错误。可直接读取 kind_code、network_kind_code、code、message 快捷字段。",
                ),
            ];
            fields.extend(try_error_fields());
            fields
        }),
        "CombatActionResult" => ("战斗动作提交和等待结果。", {
            let mut fields = vec![
                field("ok", "bool", "提交和等待流程是否成功完成。"),
                field(
                    "code",
                    "int",
                    "结果码；0 表示成功，非 0 表示动作不可用或执行失败。",
                ),
                field("message", "string", "失败原因或服务器返回信息。"),
                field(
                    "error",
                    "RocoErrorInfo?",
                    "结构化错误信息；成功或普通业务失败时为空。可直接读取 kind_code、network_kind_code、code、message 快捷字段。",
                ),
                field("ack_received", "bool", "是否收到服务器对本次动作的 ACK。"),
                field("combat_finished", "bool", "动作后战斗是否已经结束。"),
                field(
                    "next_action_ready",
                    "bool",
                    "动作后是否已经进入下一次可行动状态。",
                ),
            ];
            fields.extend(try_error_fields());
            fields
        }),
        "CombatActions" => (
            "当前战斗回合可执行动作集合。",
            vec![
                field(
                    "can_submit_action",
                    "bool",
                    "当前是否可以提交任意战斗动作。",
                ),
                field("can_use_skill", "bool", "当前是否可以使用技能。"),
                field("can_capture", "bool", "当前是否可以捕捉。"),
                field("can_use_item", "bool", "当前是否可以使用道具。"),
                field("can_change_spirit", "bool", "当前是否可以换宠。"),
                field("can_escape", "bool", "当前是否可以逃跑。"),
                field("can_use_any_skill", "bool", "当前是否至少有一个技能可用。"),
                field(
                    "can_change_to_any_spirit",
                    "bool",
                    "当前是否至少有一个可切换精灵。",
                ),
            ],
        ),
        "CombatActionSnapshot" => (
            "战斗状态和可行动信息的组合快照。",
            vec![
                field("is_finished", "bool", "战斗是否已经结束。"),
                field(
                    "state",
                    "CombatState",
                    "当前战斗状态；包含回合、天气、我方和敌方状态。",
                ),
                field("actions", "CombatActions", "当前可执行动作集合。"),
            ],
        ),
        "CombatState" => (
            "底层战斗状态快照。",
            vec![
                field("round", "int", "当前回合数。"),
                field("weather", "int", "当前天气或环境 ID。"),
                field("weather_round", "int", "天气或环境剩余回合数。"),
                field("my_side", "CombatSideState", "我方战斗状态。"),
                field("rival_side", "CombatSideState", "敌方战斗状态。"),
            ],
        ),
        "CombatSideState" => (
            "战斗一方的出战状态。",
            vec![
                field("uin", "int", "该方玩家 UIN。"),
                field("active_position", "int", "当前出战精灵位置。"),
                field("spirits", "CombatSpiritState[]", "该方精灵状态列表。"),
            ],
        ),
        "CombatSpiritState" => (
            "战斗中的精灵状态。",
            vec![
                field("position", "int", "背包或战斗位置。"),
                field("spirit_id", "int", "精灵 ID。"),
                field("level", "int", "等级。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "SpiritInfo" => (
            "背包、仓库或战斗中的精灵详细信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置；非背包来源可能为 0。"),
                field(
                    "catch_time",
                    "RocoOptionalI64",
                    "结构化可选捕获时间；战斗来源可能缺失。",
                ),
                field("name", "string", "精灵名称。"),
                field("level", "int", "精灵等级。"),
                field("personality", "int", "性格数字。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "SpiritSkillInfo" => (
            "精灵技能信息。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "当前 PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
            ],
        ),
        "PetEggSpiritInfo" => (
            "宠物培育协议中的宠物或宠物蛋信息。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("level", "int", "宠物等级。"),
                field("exp_to_next_level", "int", "距离下一等级所需经验。"),
                field("personality", "int", "性格数字。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("caught_time", "int", "捕获时间。"),
                field("caught_location", "int", "捕获地点 ID。"),
                field("storage_time", "int", "入库时间。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "PetEggInfo" => (
            "宠物培育室当前状态。",
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("current_egg_count", "int", "当前宠物蛋数量。"),
                field("max_egg_count", "int", "最大宠物蛋容量。"),
                field("vip_count", "int", "VIP 加速相关剩余次数。"),
                field("male", "PetEggSpiritInfo", "当前父方宠物信息。"),
                field("female", "PetEggSpiritInfo", "当前母方宠物信息。"),
                field("egg", "PetEggSpiritInfo", "当前培育的宠物蛋信息。"),
            ],
        ),
        "PetEggSpeedUpResult" => (
            "宠物蛋 VIP 加速结果。",
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("current_egg_count", "int", "当前宠物蛋数量。"),
                field("max_egg_count", "int", "最大宠物蛋容量。"),
                field("vip_count", "int", "VIP 加速相关剩余次数。"),
            ],
        ),
        "PetEggBeginResult" => (
            "开始宠物蛋培育的结果。",
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("max_egg_count", "int", "最大宠物蛋容量。"),
            ],
        ),
        "PetEggCancelResult" => (
            "取消宠物蛋培育的结果。",
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("detail_code", "int", "服务端返回的详细结果码。"),
            ],
        ),
        "PetEggPreviewResult" => (
            "宠物蛋培育预览结果。",
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("egg", "PetEggSpiritInfo", "预览生成的宠物蛋信息。"),
            ],
        ),
        "StorageSpiritInfo" => (
            "仓库精灵摘要。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("catch_time", "int", "捕获时间，用于区分同种精灵个体。"),
                field("storage_time", "int", "入库时间。"),
                field("level", "int", "精灵等级。"),
                field("sex", "int", "性别标识。"),
                field("skin_flag", "int", "皮肤标识。"),
                field("talent_type", "int", "天赋类型。"),
                field("talent_level", "int", "天赋等级。"),
            ],
        ),
        "StorageSpiritDetailInfo" => (
            "仓库精灵详细信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("catch_time", "int", "捕获时间。"),
                field("storage_time", "int", "入库时间。"),
                field("name", "string", "精灵名称。"),
                field("level", "int", "精灵等级。"),
                field("personality", "int", "性格数字。"),
                field("hp", "int", "当前 HP。"),
                field("max_hp", "int", "最大 HP。"),
                field("pa", "int", "真实物攻能力值。"),
                field("pd", "int", "真实物防能力值。"),
                field("ma", "int", "真实魔攻能力值。"),
                field("md", "int", "真实魔防能力值。"),
                field("sp", "int", "真实速度能力值。"),
                field("hp_ability", "int", "真实精力能力值。"),
                field("skills", "SpiritSkillInfo[]", "技能列表。"),
            ],
        ),
        "BagItemInfo" => (
            "背包物品数量。",
            vec![
                field("item_id", "int", "道具 ID。"),
                field("count", "int", "数量。"),
            ],
        ),
        "SkillPoolInfo" => (
            "精灵技能池信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置。"),
                field("skills", "SkillPoolSkillInfo[]", "技能池技能列表。"),
            ],
        ),
        "SkillPoolSkillInfo" => (
            "技能池中的技能信息。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
                field("position", "int", "技能所在槽位。"),
            ],
        ),
        "SkillSwitchResult" => (
            "技能切换结果。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("position", "int", "背包位置。"),
                field("skill_slot", "int", "技能槽位。"),
                field("skill_id", "int", "技能 ID。"),
            ],
        ),
        "SkillStoneResult" => (
            "技能石预览、应用或替换结果。",
            vec![
                field("ok", "bool", "操作是否成功。"),
                field("result_code", "int", "服务器结果码。"),
                field("message", "string", "失败原因或提示信息。"),
                field("item_id", "int", "技能石道具 ID。"),
                field("position", "int", "背包位置。"),
                field("needs_replace", "bool", "是否需要选择旧技能进行替换。"),
                field(
                    "old_skills",
                    "SkillStoneSkillInfo[]",
                    "可被替换的旧技能列表。",
                ),
                field(
                    "new_skills",
                    "SkillStoneSkillInfo[]",
                    "技能石提供的新技能列表。",
                ),
            ],
        ),
        "SkillStoneSkillInfo" => (
            "技能石候选技能。",
            vec![
                field("skill_id", "int", "技能 ID。"),
                field("pp", "int", "PP。"),
                field("inherited", "bool", "是否为遗传技能。"),
            ],
        ),
        "BattleResult" => (
            "战斗结算结果。",
            vec![
                field("winner", "int", "胜负方标识。"),
                field("total_rounds", "int", "战斗总回合数。"),
                field("finish_code", "int", "战斗结束原因码。"),
                field("trainer_exp", "int", "获得的训练师经验。"),
                field(
                    "next_level_trainer_exp",
                    "int",
                    "距离下一训练师等级所需经验。",
                ),
                field("honour_point", "int", "获得的荣誉点。"),
                field("exp_add_bits", "int", "经验加成位标记。"),
                field("obtained_items", "BagItemInfo[]", "获得的物品列表。"),
                field(
                    "spirit_results",
                    "BattleSpiritResult[]",
                    "参战精灵经验/等级变化列表。",
                ),
                field(
                    "captured_spirits",
                    "BattleCapturedSpirit[]",
                    "本场捕获精灵列表。",
                ),
            ],
        ),
        "BattleResultQueryResult" => (
            "不会因战斗结果暂不可用而抛错的查询结果。",
            {
                let mut fields = vec![
                    field("ok", "bool", "是否成功取得战斗结果。"),
                    field(
                        "code",
                        "int",
                        "结果码；0 表示成功，非 0 表示暂不可用或失败。",
                    ),
                    field("message", "string", "失败原因，成功时通常为空。"),
                    field(
                        "error",
                        "RocoErrorInfo?",
                        "结构化错误信息；成功或普通暂不可用时为空。可直接读取 kind_code、network_kind_code、code、message 快捷字段。",
                    ),
                    field(
                        "result",
                        "BattleResult",
                        "战斗结算结果；ok 为 false 时为默认空结果。",
                    ),
                ];
                fields.extend(try_error_fields());
                fields
            },
        ),
        "MiniGameSubmitResult" => (
            "小游戏分数提交结果。",
            vec![
                field("ok", "bool", "提交是否成功。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示失败。"),
                field("message", "string", "失败原因或服务器返回信息。"),
                field("game_id", "int", "小游戏 ID。"),
                field("score", "int", "提交的分数。"),
                field("game_type", "int", "小游戏类型。"),
                field("items", "MiniGameRewardItem[]", "获得的奖励物品。"),
                field(
                    "extra_fields",
                    "MiniGameExtraField[]",
                    "协议返回的附加字段。",
                ),
            ],
        ),
        "MiniGameSubmitTryResult" => (
            "不会因网络或业务失败抛错的小游戏提交结果。",
            {
                let mut fields = vec![
                    field("ok", "bool", "是否成功提交。"),
                    field(
                        "code",
                        "int",
                        "结果码；0 表示成功，1001 表示网络错误，其它非 0 表示业务失败。",
                    ),
                    field("message", "string", "失败原因，成功时通常为空。"),
                    field(
                        "error",
                        "RocoErrorInfo?",
                        "结构化错误信息；成功或普通业务失败时为空。可直接读取 kind_code、network_kind_code、code、message 快捷字段。",
                    ),
                    field(
                        "result",
                        "MiniGameSubmitResult",
                        "小游戏提交结果；失败时为默认失败结果。",
                    ),
                ];
                fields.extend(try_error_fields());
                fields
            },
        ),
        _ => return None,
    })
}
