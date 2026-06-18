use super::StdlibFunctionDoc;

pub fn docs() -> Vec<StdlibFunctionDoc> {
    vec![
        super::stdlib_doc!(
            "game",
            "get_pause",
            "game::get_pause() -> bool",
            "查询当前游戏暂停状态。",
            params: [],
            returns: "已暂停返回 true。",
            examples: ["let paused = game::get_pause();"]
        ),
        super::stdlib_doc!(
            "game",
            "set_pause",
            "game::set_pause(enabled: bool) -> bool",
            "设置游戏暂停状态。",
            params: ["enabled" => "true 表示暂停，false 表示恢复。"],
            returns: "设置成功返回 true。",
            examples: ["game::set_pause(true);"]
        ),
        super::stdlib_doc!(
            "game",
            "try_set_pause",
            "game::try_set_pause(enabled: bool) -> ActionResult",
            "尝试设置游戏暂停状态，失败时返回结构化结果。",
            params: ["enabled" => "true 表示暂停，false 表示恢复。"],
            returns: "结构化操作结果。",
            examples: ["let result = game::try_set_pause(false);"]
        ),
        super::stdlib_doc!(
            "game",
            "start_mini_game",
            "game::start_mini_game(game_id: int) -> ()",
            "启动指定小游戏。",
            params: ["game_id" => "小游戏 ID。"],
            returns: "无返回值。",
            examples: ["game::start_mini_game(1);"]
        ),
        super::stdlib_doc!(
            "game",
            "submit_mini_game_score",
            "game::submit_mini_game_score(game_id: int, score: int, game_type: int) -> MiniGameSubmitResult",
            "提交小游戏分数。",
            params: ["game_id" => "小游戏 ID。", "score" => "分数。", "game_type" => "小游戏类型。"],
            returns: "提交结果。",
            examples: ["let result = game::submit_mini_game_score(1, 100, 0);"]
        ),
        super::stdlib_doc!(
            "game",
            "try_submit_mini_game_score",
            "game::try_submit_mini_game_score(game_id: int, score: int, game_type: int) -> MiniGameSubmitTryResult",
            "尝试提交小游戏分数，失败时返回结构化结果。",
            params: ["game_id" => "小游戏 ID。", "score" => "分数。", "game_type" => "小游戏类型。"],
            returns: "结构化提交结果。",
            examples: ["let result = game::try_submit_mini_game_score(1, 100, 0);"]
        ),
    ]
}
