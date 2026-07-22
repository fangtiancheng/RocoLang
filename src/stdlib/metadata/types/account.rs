use super::{field, try_error_fields, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "PetTrainingRewardItem" => (
            "家园锻炼奖励道具。",
            vec![
                field("item_id", "int", "奖励道具 ID。"),
                field("count", "int", "奖励数量。"),
            ],
        ),
        "PetTrainingResult" => (
            "家园锻炼 CGI 返回结果。",
            vec![
                field("ok", "bool", "请求是否成功。"),
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("training_type", "int", "锻炼类型。"),
                field("pet_id", "int", "锻炼宠物 ID；结算时传回服务器。"),
                field("rewards", "PetTrainingRewardItem[]", "结算获得的奖励道具。"),
                field("raw_text", "string", "原始 CGI 响应文本。"),
            ],
        ),
        "UserInfo" => (
            "当前登录用户信息。",
            vec![
                field("uin", "int", "用户 UIN。"),
                field("id", "int", "角色 ID。"),
                field("nick_name", "string", "角色昵称。"),
                field("level", "int", "角色等级。"),
                field("is_vip", "bool", "是否为 VIP。"),
                field("vip_level", "int", "VIP 等级。"),
                field("vip_expiring_days", "int", "VIP 剩余天数。"),
                field("vip_lulu", "int", "Lulu VIP 状态。"),
                field("trainer_level", "int", "训练师等级。"),
                field("trainer_exp", "int", "训练师经验。"),
            ],
        ),
        "ServerTimeInfo" => (
            "服务器时间信息。",
            vec![
                field("stamp", "int", "服务器时间戳，单位秒。"),
                field("full_year", "int", "四位年份。"),
                field("month", "int", "月份，1-12。"),
                field("date", "int", "日期，1-31。"),
                field("hours", "int", "小时，0-23。"),
                field("minutes", "int", "分钟，0-59。"),
                field("seconds", "int", "秒，0-59。"),
                field("day", "int", "星期字段，语义与服务器协议一致。"),
                field(
                    "day_of_year",
                    "int",
                    "一年中的第几天，语义与服务器协议一致。",
                ),
            ],
        ),
        "ServerTimeResult" => (
            "不会因网络或业务失败抛错的服务器时间查询结果。",
            {
                let mut fields = vec![
                    field("ok", "bool", "是否成功查询。"),
                    field("code", "int", "结果码；0 表示成功，其它非 0 表示失败。"),
                    field("message", "string", "失败原因，成功时通常为空。"),
                    field(
                    "error",
                    "RocoErrorInfo?",
                    "结构化错误信息；成功或普通业务失败时为空。可直接读取 kind_code、network_kind_code、code、message 快捷字段。",
                ),
                    field("result", "ServerTimeInfo", "服务器时间；失败时为默认空值。"),
                ];
                fields.extend(try_error_fields());
                fields
            },
        ),
        "SceneSpiritInfo" => (
            "场景中的宠物刷新信息。",
            vec![
                field("spirit_id", "int", "精灵 ID。"),
                field("count", "int", "刷新数量。"),
                field("area_index", "int", "区域索引。"),
                field("is_rare", "bool", "是否稀有宠物。"),
                field("is_boss", "bool", "是否 Boss。"),
                field("is_npc_boss", "bool", "是否 NPC Boss。"),
            ],
        ),
        "SceneRoleInfo" => (
            "场景中的角色缓存信息。",
            vec![
                field("uin", "int", "角色 UIN。"),
                field("id", "int", "角色 ID。"),
                field("nick_name", "string", "昵称。"),
                field("level", "int", "等级。"),
                field("loc_x", "int", "场景 X 坐标。"),
                field("loc_y", "int", "场景 Y 坐标。"),
                field("pk_state", "int", "PK 状态。"),
                field("is_in_combat", "bool", "是否处于战斗中。"),
                field("is_vip", "bool", "是否 VIP。"),
                field("vip_level", "int", "VIP 等级。"),
                field("trainer_level", "int", "训练师等级。"),
                field("trainer_exp", "int", "训练师经验。"),
            ],
        ),
        "BattleInfo" => (
            "Combat session information.",
            vec![
                field("battle_id", "string", "Battle ID."),
                field("my_uin", "int", "My UIN."),
                field("rival_uin", "int", "Rival UIN."),
                field("started", "bool", "Whether the battle has started."),
            ],
        ),
        "RoundResult" => (
            "Round wait result.",
            vec![
                field("round", "int", "Current round number."),
                field("my_hp", "int", "Current HP of my active spirit."),
                field("rival_hp", "int", "Current HP of the rival active spirit."),
                field("finished", "bool", "Whether the battle has finished."),
            ],
        ),
        "NewsTimesReportsResult" => (
            "News report query result.",
            vec![
                field("gift_gotten", "int", "Gift claim status."),
                field("reports", "NewsTimesReport[]", "News report entries."),
                field(
                    "player_status_today",
                    "int[]",
                    "Per-player status for today.",
                ),
                field(
                    "player_status_forever",
                    "int[]",
                    "Persistent per-player status.",
                ),
            ],
        ),
        "NewsActiveItem" => (
            "Active news or activity item.",
            vec![
                field("id", "int", "Activity ID."),
                field("scene_id", "int", "Related scene ID."),
                field("npc_x", "int", "NPC X coordinate."),
                field("npc_y", "int", "NPC Y coordinate."),
                field("time", "string", "Activity time text."),
                field("content", "string", "Activity content text."),
                field(
                    "auto_start",
                    "bool",
                    "Whether the item starts automatically.",
                ),
                field("script_url", "string", "Script URL."),
                field("app_url", "string", "Application entry URL."),
            ],
        ),
        "TypeLadderRank" => (
            "系别天梯段位信息。",
            vec![
                field("rank", "int", "大段位。"),
                field("small_rank", "int", "小段位。"),
                field("star", "int", "星数。"),
            ],
        ),
        "TypeLadderRankUser" => (
            "系别天梯排行用户。",
            vec![
                field("uin", "int", "用户 UIN。"),
                field("name", "string", "昵称。"),
                field("win_count", "int", "胜场数。"),
                field("battle_count", "int", "战斗数。"),
                field("rank_num", "int", "排名。"),
                field("score", "TypeLadderRank", "段位信息。"),
            ],
        ),
        "TypeLadderRankInfo" => (
            "系别天梯排行信息。",
            vec![
                field(
                    "my_info",
                    "RocoOptionalTypeLadderRankUser",
                    "结构化可选个人排行信息。",
                ),
                field("users", "TypeLadderRankUser[]", "排行用户列表。"),
            ],
        ),
        _ => return None,
    })
}
