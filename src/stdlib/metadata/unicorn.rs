use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("unicorn", "claim_cultivation_reward", return_type: "UnicornInfo", "领取独角兽培养奖励。", params: [], returns: "返回领取后的独角兽活动状态。", examples: ["let info = unicorn::claim_cultivation_reward();"]),
        super::stdlib_doc!("unicorn", "evolve", return_type: "UnicornInfo", "执行独角兽指定阶段进化。", params: ["stage" => "进化阶段。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回进化后的独角兽活动状态。", examples: ["let info = unicorn::evolve(1, 0);"]),
        super::stdlib_doc!("unicorn", "exchange_skill_stone", return_type: "UnicornInfo", "兑换独角兽技能石。", params: ["skill" => "技能或技能石 ID。", "cost_kind" => "消耗类型。"], returns: "返回兑换后的独角兽活动状态。", examples: ["let info = unicorn::exchange_skill_stone(100, 0);"]),
        super::stdlib_doc!("unicorn", "harvest", return_type: "UnicornInfo", "收获独角兽培养产物。", params: [], returns: "返回收获后的独角兽活动状态。", examples: ["let info = unicorn::harvest();"]),
        super::stdlib_doc!("unicorn", "one_key_evolve", return_type: "UnicornInfo", "一键执行独角兽指定阶段进化。", params: ["stage" => "进化阶段。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回一键进化后的独角兽活动状态。", examples: ["let info = unicorn::one_key_evolve(1, 0);"]),
        super::stdlib_doc!("unicorn", "query", return_type: "UnicornInfo", "查询独角兽活动状态。", params: [], returns: "返回培养、召唤、进化和奖励信息。", examples: ["let info = unicorn::query();"]),
        super::stdlib_doc!("unicorn", "query_evolution_bag", return_type: "UnicornInfo", "查询独角兽指定阶段可进化宠物。", params: ["stage" => "进化阶段。"], returns: "返回可进化宠物和活动状态。", examples: ["let info = unicorn::query_evolution_bag(1);"]),
        super::stdlib_doc!("unicorn", "settle_summon_fight", return_type: "UnicornInfo", "提交独角兽召唤战斗结算。", params: ["slot" => "召唤槽位。"], returns: "返回结算后的独角兽活动状态。", examples: ["let info = unicorn::settle_summon_fight(1);"]),
        super::stdlib_doc!("unicorn", "start_cultivation", return_type: "UnicornInfo", "开始独角兽培养。", params: [], returns: "返回开始培养后的独角兽活动状态。", examples: ["let info = unicorn::start_cultivation();"]),
        super::stdlib_doc!("unicorn", "submit_cultivation_task", return_type: "UnicornInfo", "提交独角兽培养任务。", params: ["task" => "培养任务 ID。"], returns: "返回提交后的独角兽活动状态。", examples: ["let info = unicorn::submit_cultivation_task(1);"]),
        super::stdlib_doc!("unicorn", "submit_minigame", return_type: "UnicornInfo", "提交独角兽小游戏结果。", params: [], returns: "返回提交后的独角兽活动状态。", examples: ["let info = unicorn::submit_minigame();"]),
        super::stdlib_doc!("unicorn", "summon", return_type: "UnicornInfo", "执行独角兽召唤。", params: [], returns: "返回召唤后的独角兽活动状态。", examples: ["let info = unicorn::summon();"]),
    ]
}
