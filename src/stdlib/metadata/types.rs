use super::{StdlibFieldDoc, StdlibReturnDoc};

pub fn return_doc_for(type_name: &str) -> Option<StdlibReturnDoc> {
    let normalized = normalize_type_name(type_name);
    let (description, fields) = match normalized.as_str() {
        "RocoRequestContext" => (
            "结构化请求上下文。",
            vec![
                field("raw", "string", "原始请求上下文。"),
                field("domain", "string", "第一个点号前的业务域。"),
                field(
                    "action",
                    "string",
                    "第一个点号后的动作名；旧单段上下文为空。",
                ),
            ],
        ),
        "RocoRewardKind" => (
            "结构化奖励类型。",
            vec![field(
                "code",
                "string",
                "稳定脚本代码，例如 item、money、assignable_exp、spirit_equipment。",
            )],
        ),
        "RocoOptionalI64" => (
            "结构化可选整数。",
            vec![
                field("present", "bool", "是否存在服务器返回值。"),
                field("value", "int", "存在时为服务器返回值；缺失时为 0。"),
                field(
                    "value_or_missing",
                    "int",
                    "存在时为服务器返回值；缺失时为 -1。",
                ),
            ],
        ),
        "RocoDisplayItem" => (
            "展示用奖励物品。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("item_count", "int", "数量。"),
                field("item_type", "int", "物品类型。"),
            ],
        ),
        "RocoOptionalDisplayItem" => (
            "结构化可选展示物品。",
            vec![
                field("present", "bool", "是否存在展示物品。"),
                field("item", "RocoDisplayItem", "展示物品；缺失时为 0 占位。"),
            ],
        ),
        "RocoOptionalIceCrystalBattleInfo" => (
            "结构化可选冰晶战斗信息。",
            vec![
                field("present", "bool", "是否存在当前战斗。"),
                field(
                    "battle",
                    "IceCrystalBattleInfo",
                    "当前战斗；缺失时为 0 占位。",
                ),
            ],
        ),
        "RocoOptionalCapricornSecondTask" => (
            "结构化可选摩羯宫二阶任务。",
            vec![
                field("present", "bool", "是否存在二阶任务。"),
                field("task", "CapricornSecondTask", "二阶任务；缺失时为 0 占位。"),
            ],
        ),
        "RocoOptionalStarTowerTop" => (
            "结构化可选星辰塔顶层信息。",
            vec![
                field("present", "bool", "是否存在顶层信息。"),
                field("top", "StarTowerTop", "顶层信息；缺失时为 0 占位。"),
                field("star", "int", "顶层星数；缺失时为 0。"),
                field("refresh", "int", "刷新状态；缺失时为 0。"),
                field("fight_desc", "string", "战斗描述；缺失时为空字符串。"),
                field("task_desc", "string", "任务描述；缺失时为空字符串。"),
                field("fight_id", "int", "战斗 ID；缺失时为 0。"),
                field("tokens", "int[]", "代币列表；缺失时为空列表。"),
                field("exchanges", "int[]", "兑换列表；缺失时为空列表。"),
                field(
                    "missions",
                    "StarTowerTopMission[]",
                    "任务列表；缺失时为空列表。",
                ),
                field(
                    "rewards",
                    "StarTowerTopReward[]",
                    "奖励列表；缺失时为空列表。",
                ),
            ],
        ),
        "RocoOptionalCapricornTeamSnapshot" => (
            "结构化可选摩羯宫队伍快照。",
            vec![
                field("present", "bool", "是否存在队伍快照。"),
                field(
                    "team",
                    "CapricornTeamSnapshot",
                    "队伍快照；缺失时为空队伍占位。",
                ),
            ],
        ),
        "RocoOptionalTypeLadderRankUser" => (
            "结构化可选系别天梯个人排行信息。",
            vec![
                field("present", "bool", "是否存在个人排行信息。"),
                field(
                    "user",
                    "TypeLadderRankUser",
                    "个人排行信息；缺失时为空用户占位。",
                ),
            ],
        ),
        "RocoNetworkErrorKind" => (
            "结构化网络错误子类型。",
            vec![field("code", "string", "稳定网络错误子类型代码。")],
        ),
        "RocoBridgeErrorChannel" => (
            "结构化 bridge 错误通道。",
            vec![field("code", "string", "稳定 bridge 通道代码。")],
        ),
        "RocoHttpBridgeErrorKind" => (
            "结构化 HTTP bridge 错误大类。",
            vec![field("code", "string", "稳定 HTTP bridge 错误大类代码。")],
        ),
        "RocoNetBridgeErrorKind" => (
            "结构化 Net bridge 错误大类。",
            vec![field("code", "string", "稳定 Net bridge 错误大类代码。")],
        ),
        "RocoBridgeErrorKind" => (
            "结构化 bridge 错误大类。",
            vec![field("code", "string", "稳定 bridge 错误大类代码。")],
        ),
        "RocoBridgeErrorInfo" => (
            "结构化 bridge 错误详情。",
            vec![
                field("channel", "RocoBridgeErrorChannel", "bridge 通道。"),
                field("channel_code", "string", "稳定 bridge 通道代码。"),
                field("kind", "RocoBridgeErrorKind", "结构化 bridge 错误大类。"),
                field("kind_code", "string", "稳定 bridge 错误大类代码。"),
                field("operation_code", "string", "稳定 bridge 操作代码。"),
                field("message", "string", "bridge 错误说明。"),
                field("description", "string", "格式化后的 bridge 错误说明。"),
            ],
        ),
        "RocoNetResponseParseSource" => (
            "结构化网络响应解析失败来源。",
            vec![
                field("kind_code", "string", "稳定解析失败来源代码。"),
                field(
                    "protocol_kind_code",
                    "string",
                    "协议解析失败分类代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_message",
                    "string",
                    "协议解析错误说明；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason",
                    "RocoProtocolParseReason?",
                    "结构化协议解析失败原因；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason_code",
                    "string",
                    "稳定协议解析失败原因代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_error_type",
                    "string",
                    "协议解析错误来源类型代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer",
                    "RocoProtocolParseLayer?",
                    "协议解析来源层级；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer_code",
                    "string",
                    "稳定协议解析来源层级代码；非 protocol 来源为空。",
                ),
                field(
                    "unexpected_cmd_id",
                    "int",
                    "意外命令号；非 unexpected_command 来源为 0。",
                ),
            ],
        ),
        "RocoNetResponseParseFailure" => (
            "结构化网络响应解析失败详情。",
            vec![
                field("target", "RocoNetResponseParseTarget", "结构化解析目标。"),
                field("target_code", "string", "稳定解析目标代码。"),
                field("target_label", "string", "解析目标显示名。"),
                field("source", "RocoNetResponseParseSource", "解析失败来源。"),
                field("source_kind_code", "string", "稳定解析失败来源代码。"),
                field(
                    "protocol_kind_code",
                    "string",
                    "协议解析失败分类代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_message",
                    "string",
                    "协议解析错误说明；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason",
                    "RocoProtocolParseReason?",
                    "结构化协议解析失败原因；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason_code",
                    "string",
                    "稳定协议解析失败原因代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_error_type",
                    "string",
                    "协议解析错误来源类型代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer",
                    "RocoProtocolParseLayer?",
                    "协议解析来源层级；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer_code",
                    "string",
                    "稳定协议解析来源层级代码；非 protocol 来源为空。",
                ),
                field(
                    "unexpected_cmd_id",
                    "int",
                    "意外命令号；非 unexpected_command 来源为 0。",
                ),
                field("description", "string", "格式化后的解析失败说明。"),
            ],
        ),
        "RocoNetResponseParseTarget" => (
            "网络响应解析目标。",
            vec![
                field("code", "string", "稳定解析目标代码。"),
                field("label", "string", "解析目标显示名。"),
            ],
        ),
        "RocoProtocolParseFailureKind" => (
            "协议解析失败分类。",
            vec![field("code", "string", "稳定协议解析失败分类代码。")],
        ),
        "RocoProtocolParseReason" => (
            "结构化协议解析失败原因。",
            vec![
                field("code", "string", "稳定原因代码。"),
                field("message", "string", "格式化失败说明。"),
                field(
                    "field_name",
                    "RocoProtocolFieldName?",
                    "结构化协议字段名；仅 missing_field 和 missing_indexed_field 原因为非空。",
                ),
                field("field_code", "string", "稳定协议字段代码；无字段时为空。"),
                field(
                    "spirit_storage_field",
                    "RocoSpiritStorageProtoField?",
                    "结构化仓库精灵字段；仅 spirit_storage_missing_field 原因为非空。",
                ),
                field(
                    "spirit_storage_field_code",
                    "string",
                    "稳定仓库精灵字段代码；无字段时为空。",
                ),
            ],
        ),
        "RocoSpiritStorageProtoField" => (
            "仓库精灵 protobuf 字段。",
            vec![
                field("code", "string", "稳定字段代码。"),
                field("label", "string", "字段显示名。"),
            ],
        ),
        "RocoProtocolParseErrorType" => (
            "协议解析错误来源类型。",
            vec![field("code", "string", "稳定协议解析错误来源类型代码。")],
        ),
        "RocoProtocolParseLayer" => (
            "协议解析来源层级。",
            vec![field("code", "string", "稳定协议解析来源层级代码。")],
        ),
        "ScriptBridgeOperation" => (
            "脚本 stdlib bridge 操作类型。",
            vec![field("code", "string", "稳定脚本 bridge 操作代码。")],
        ),
        "ScriptBridgeFailureReason" => (
            "脚本 stdlib bridge 失败原因。",
            vec![
                field("code", "string", "稳定 bridge 失败原因代码。"),
                field("message", "string", "bridge 失败原因说明。"),
            ],
        ),
        "ScriptBridgeFailure" => (
            "脚本 stdlib bridge 失败详情。",
            vec![
                field("operation", "ScriptBridgeOperation", "失败的 bridge 操作。"),
                field("operation_code", "string", "稳定 bridge 操作代码。"),
                field("reason", "ScriptBridgeFailureReason", "bridge 失败原因。"),
                field("reason_code", "string", "稳定 bridge 失败原因代码。"),
                field("message", "string", "bridge 失败说明。"),
                field("description", "string", "格式化后的 bridge 失败说明。"),
            ],
        ),
        "ScriptSystemOperation" => (
            "脚本 system 操作类型。",
            vec![field("code", "string", "稳定 system 操作代码。")],
        ),
        "ScriptSystemFailure" => (
            "脚本 system 操作失败详情。",
            vec![
                field("operation", "ScriptSystemOperation", "失败的 system 操作。"),
                field("operation_code", "string", "稳定 system 操作代码。"),
                field(
                    "source",
                    "ScriptSystemFailureSource",
                    "结构化 system 失败来源。",
                ),
                field("source_code", "string", "稳定 system 失败来源代码。"),
                field("message", "string", "由结构化来源派生的 system 失败说明。"),
                field("description", "string", "格式化后的 system 失败说明。"),
            ],
        ),
        "ScriptSystemFailureSource" => (
            "脚本 system 操作失败来源。",
            vec![field("code", "string", "稳定 system 失败来源代码。")],
        ),
        "RocoErrorKind" => (
            "结构化错误类型。",
            vec![
                field(
                    "code",
                    "string",
                    "稳定错误类型代码；网络错误包含 network.<subkind>。",
                ),
                field("family_code", "string", "顶层错误家族代码。"),
                field(
                    "network_kind",
                    "RocoNetworkErrorKind?",
                    "网络错误子类型；非网络错误为空。",
                ),
            ],
        ),
        "RocoGeneralLockTarget" => (
            "结构化通用锁目标。",
            vec![field("code", "string", "稳定锁目标代码。")],
        ),
        "RocoErrorInfo" => (
            "结构化错误信息。",
            vec![
                field("kind", "RocoErrorKind", "结构化错误类型。"),
                field("detail", "RocoErrorDetail", "结构化错误详情。"),
                field(
                    "kind_code",
                    "string",
                    "稳定错误类型代码；网络错误包含 network.<subkind>。",
                ),
                field(
                    "detail_kind_code",
                    "string",
                    "稳定错误详情代码；无详情时为空。",
                ),
                field(
                    "network_kind_code",
                    "string",
                    "稳定网络错误子类型代码；非网络错误时为空。",
                ),
                field("code", "string", "面向脚本的具体错误代码。"),
                field("message", "string", "错误说明。"),
            ],
        ),
        "RocoErrorDetail" => (
            "结构化错误详情。",
            vec![
                field(
                    "kind_code",
                    "string",
                    "稳定错误详情代码，例如 invalid_param、bridge、return_code。",
                ),
                field(
                    "general_lock_target",
                    "RocoGeneralLockTarget?",
                    "通用锁错误的锁目标；非 general 详情为空。",
                ),
                field(
                    "general_lock_target_code",
                    "string",
                    "通用锁错误的稳定锁目标代码；非 general 详情为空。",
                ),
                field(
                    "invalid_param_kind_code",
                    "string",
                    "非法参数类型代码；非 invalid_param 详情为空。",
                ),
                field(
                    "invalid_param_name",
                    "string",
                    "非法参数名；非 invalid_param 详情为空。",
                ),
                field(
                    "invalid_param_value",
                    "int",
                    "非法参数数值；非 invalid_param 详情为 0。",
                ),
                field(
                    "invalid_param_message",
                    "string",
                    "非法参数说明；非 invalid_param 详情为空。",
                ),
                field(
                    "invalid_param_expected",
                    "string",
                    "非法参数期望范围；非 invalid_param 详情为空。",
                ),
                field(
                    "invalid_param_protocol_field",
                    "string",
                    "协议层拒绝的字段名；非 protocol_rejected 详情为空。",
                ),
                field(
                    "unsupported_module",
                    "string",
                    "未支持函数所在模块；非 unsupported_function 详情为空。",
                ),
                field(
                    "unsupported_function_name",
                    "string",
                    "未支持函数名；非 unsupported_function 详情为空。",
                ),
                field(
                    "function_context_kind_code",
                    "string",
                    "函数上下文错误类型代码；非 function_context 详情为空。",
                ),
                field(
                    "function_context_combat_phase_code",
                    "string",
                    "函数上下文错误涉及的战斗阶段代码；仅 cannot_wait_for_combat 非空。",
                ),
                field(
                    "query_kind_code",
                    "string",
                    "脚本查询错误类型代码；非 query 详情为空。",
                ),
                field(
                    "query_skill_index",
                    "int",
                    "脚本查询涉及的技能索引；仅 no_skill_at_index 非 -1。",
                ),
                field(
                    "static_data_kind_code",
                    "string",
                    "脚本静态数据错误类型代码；非 static_data 详情为空。",
                ),
                field(
                    "static_data_position",
                    "int",
                    "脚本静态数据错误涉及的背包位置；无对应位置时为 -1。",
                ),
                field(
                    "static_data_function_name",
                    "string",
                    "脚本静态数据未实现函数名；仅 not_implemented 非空。",
                ),
                field(
                    "static_data_message",
                    "string",
                    "脚本静态数据错误说明；仅 active_config_not_available 非空。",
                ),
                field(
                    "static_data_active_config_source_code",
                    "string",
                    "ActiveConfig 不可用的结构化来源代码；非 active_config_not_available 为空。",
                ),
                field(
                    "system_operation_code",
                    "string",
                    "脚本 system 操作代码；非 system_failure 详情为空。",
                ),
                field(
                    "system_source_code",
                    "string",
                    "脚本 system 失败来源代码；非 system_failure 详情为空。",
                ),
                field(
                    "system_message",
                    "string",
                    "脚本 system 失败说明；非 system_failure 详情为空。",
                ),
                field(
                    "stdlib_bridge_operation_code",
                    "string",
                    "脚本 stdlib bridge 操作代码；非 stdlib_bridge 详情为空。",
                ),
                field(
                    "stdlib_bridge_message",
                    "string",
                    "脚本 stdlib bridge 失败说明；非 stdlib_bridge 详情为空。",
                ),
                field(
                    "activity_operation_kind_code",
                    "string",
                    "脚本活动操作错误类型代码；非 activity_operation 详情为空。",
                ),
                field(
                    "activity_operation_activity_code",
                    "string",
                    "脚本活动操作涉及的活动代码；非 activity_operation 详情为空。",
                ),
                field(
                    "activity_operation_field_code",
                    "string",
                    "脚本活动操作涉及的选项字段代码；仅 invalid_option 非空。",
                ),
                field(
                    "activity_operation_count",
                    "int",
                    "脚本活动操作涉及的数量；无对应数量时为 -1。",
                ),
                field(
                    "activity_operation_limit",
                    "int",
                    "脚本活动操作涉及的上限；无对应上限时为 -1。",
                ),
                field(
                    "activity_operation_value",
                    "int",
                    "脚本活动操作涉及的原始选项值；仅 invalid_option 非 -1。",
                ),
                field(
                    "combat_action_kind_code",
                    "string",
                    "脚本战斗动作错误类型代码；非 combat_action 详情为空。",
                ),
                field(
                    "combat_action_position",
                    "int",
                    "脚本战斗动作涉及的精灵位置；无对应位置时为 -1。",
                ),
                field(
                    "combat_action_skill_id",
                    "int",
                    "脚本战斗动作涉及的技能 ID；无对应技能 ID 时为 -1。",
                ),
                field(
                    "combat_action_item_id",
                    "int",
                    "脚本战斗动作涉及的道具 ID；无对应道具 ID 时为 -1。",
                ),
                field(
                    "combat_wait_kind_code",
                    "string",
                    "脚本战斗等待错误类型代码；非 combat_wait 详情为空。",
                ),
                field(
                    "combat_wait_combat_phase_code",
                    "string",
                    "脚本战斗等待错误涉及的战斗阶段代码；非 combat_wait 详情为空。",
                ),
                field(
                    "combat_wait_elapsed_ms",
                    "int",
                    "脚本战斗等待已耗时毫秒；非 combat_wait 详情为 -1。",
                ),
                field(
                    "combat_runtime_message",
                    "string",
                    "脚本战斗运行时错误说明；非 combat_runtime 详情为空。",
                ),
                field(
                    "combat_command_failure_kind",
                    "ScriptCombatCommandFailureKind?",
                    "战斗命令失败类型；仅自动收尾或前端加载标记失败时非空。",
                ),
                field(
                    "combat_command_failure_kind_code",
                    "string",
                    "稳定战斗命令失败类型代码；非对应 combat_runtime 详情为空。",
                ),
                field(
                    "pending_response_kind_code",
                    "string",
                    "待响应处理错误类型代码；非 pending_response 详情为空。",
                ),
                field(
                    "pending_http_response_code",
                    "string",
                    "待处理 HTTP 响应代码；仅 unexpected_http_response 非空。",
                ),
                field(
                    "expected_http_response_code",
                    "string",
                    "期望 HTTP 响应代码；仅 unexpected_http_response 非空。",
                ),
                field(
                    "actual_http_response_code",
                    "string",
                    "实际 HTTP 响应代码；仅 unexpected_http_response 非空。",
                ),
                field(
                    "pending_response_combat_phase_code",
                    "string",
                    "待响应处理中的战斗阶段代码；仅 combat_loaded_unexpected_phase 非空。",
                ),
                field(
                    "pending_response_net_target_code",
                    "string",
                    "待响应处理中的网络响应目标代码；仅 missing_net_response_payload 非空。",
                ),
                field(
                    "pending_response_expected_action_kind",
                    "int",
                    "期望战斗动作类型；仅 combat_action_ack_mismatch 非 0。",
                ),
                field(
                    "pending_response_expected_spirit_index",
                    "int",
                    "期望战斗精灵位置；仅 combat_action_ack_mismatch 非 0。",
                ),
                field(
                    "pending_response_actual_action_kind",
                    "int",
                    "实际战斗动作类型；仅 combat_action_ack_mismatch 非 0。",
                ),
                field(
                    "pending_response_actual_spirit_index",
                    "int",
                    "实际战斗精灵位置；仅 combat_action_ack_mismatch 非 0。",
                ),
                field(
                    "response_kind_code",
                    "string",
                    "脚本响应转换错误类型代码；非 response 详情为空。",
                ),
                field(
                    "expected_response_code",
                    "string",
                    "期望脚本响应类型代码；仅 response.type_mismatch 非空。",
                ),
                field(
                    "actual_response_code",
                    "string",
                    "实际脚本响应类型代码；仅 response.type_mismatch 非空。",
                ),
                field(
                    "request_kind_code",
                    "string",
                    "脚本请求错误类型代码；非 request 详情为空。",
                ),
                field(
                    "request_wait_context_code",
                    "string",
                    "脚本请求错误涉及的等待上下文代码；仅等待状态相关错误非空。",
                ),
                field(
                    "request_system_failure_kind_code",
                    "string",
                    "脚本请求系统失败类型代码；仅等待状态相关错误非空。",
                ),
                field(
                    "request_combat_intent_kind_code",
                    "string",
                    "脚本请求战斗意图类型代码；仅 invalid_combat_intent 非空。",
                ),
                field(
                    "request_combat_validation_kind_code",
                    "string",
                    "脚本请求战斗命令校验错误代码；仅 invalid_combat_command 非空。",
                ),
                field(
                    "request_spirit_index",
                    "int",
                    "脚本请求错误涉及的精灵位置；无对应位置时为 -1。",
                ),
                field(
                    "request_value",
                    "int",
                    "脚本请求错误涉及的原始数值；无对应数值时为 -1。",
                ),
                field(
                    "request_combat_protocol_error_kind_code",
                    "string",
                    "脚本请求战斗协议子错误代码；仅 invalid_combat_intent 非空。",
                ),
                field(
                    "request_combat_protocol_error_value",
                    "int",
                    "脚本请求战斗协议子错误的主要数值；无对应数值时为 -1。",
                ),
                field(
                    "session_memory_kind_code",
                    "string",
                    "脚本 session memory 错误类型代码；非 session_memory 详情为空。",
                ),
                field(
                    "session_memory_key",
                    "string",
                    "脚本 session memory 错误涉及的键；非 session_memory 详情为空。",
                ),
                field(
                    "session_memory_expected_kind_code",
                    "string",
                    "期望 session memory 值类型；仅 session_memory.type_mismatch 非空。",
                ),
                field(
                    "session_memory_actual_kind_code",
                    "string",
                    "实际 session memory 值类型；仅 session_memory.type_mismatch 非空。",
                ),
                field(
                    "lookup_kind_code",
                    "string",
                    "脚本 lookup 错误类型代码；非 lookup 详情为空。",
                ),
                field(
                    "lookup_entity_code",
                    "string",
                    "脚本 lookup 涉及的数据实体代码；非 lookup 详情为空。",
                ),
                field(
                    "lookup_key",
                    "string",
                    "脚本 lookup 涉及的查询键；非 lookup 详情为空。",
                ),
                field(
                    "spirit_operation_kind_code",
                    "string",
                    "脚本精灵操作错误类型代码；非 spirit_operation 详情为空。",
                ),
                field(
                    "spirit_operation_spirit_id",
                    "int",
                    "脚本精灵操作涉及的精灵 ID；无对应精灵 ID 时为 -1。",
                ),
                field(
                    "spirit_operation_catch_time",
                    "int",
                    "脚本精灵操作涉及的 catch_time；无对应 catch_time 时为 -1。",
                ),
                field(
                    "bridge_channel_code",
                    "string",
                    "bridge 通道代码；非 bridge 详情为空。",
                ),
                field(
                    "bridge_operation_code",
                    "string",
                    "bridge 操作代码；非 bridge 详情为空。",
                ),
                field(
                    "bridge_message",
                    "string",
                    "bridge 错误说明；非 bridge 详情为空。",
                ),
                field(
                    "net_response_parse_target",
                    "string",
                    "网络响应解析目标代码；非 net_response_parse 详情为空。",
                ),
                field(
                    "net_response_parse_target_code",
                    "string",
                    "网络响应解析目标代码；非 net_response_parse 详情为空。",
                ),
                field(
                    "net_response_parse_target_label",
                    "string",
                    "网络响应解析目标显示名；非 net_response_parse 详情为空。",
                ),
                field(
                    "net_response_parse_source_kind_code",
                    "string",
                    "网络响应解析失败来源代码；非 net_response_parse 详情为空。",
                ),
                field(
                    "net_response_parse_protocol_message",
                    "string",
                    "网络响应协议解析错误说明；非 protocol 来源为空。",
                ),
                field(
                    "net_response_parse_protocol_error_type",
                    "string",
                    "网络响应协议解析错误来源类型代码；非 protocol 来源为空。",
                ),
                field(
                    "net_response_parse_unexpected_cmd_id",
                    "int",
                    "网络响应意外命令号；非 unexpected_command 来源为 0。",
                ),
                field(
                    "return_code_kind_code",
                    "string",
                    "服务器返回码类型代码；非 return_code 详情为空。",
                ),
                field(
                    "return_code_value",
                    "int",
                    "服务器返回码数值；非 return_code 详情为 0。",
                ),
                field(
                    "return_code_message",
                    "string",
                    "服务器返回码消息；非 return_code 详情为空。",
                ),
                field(
                    "http_business_result_code",
                    "int",
                    "HTTP 业务结果码；非 http_business 详情为 0。",
                ),
                field(
                    "http_business_request_context",
                    "string",
                    "HTTP 业务请求上下文；非 http_business 详情为空。",
                ),
                field(
                    "http_business_message",
                    "string",
                    "HTTP 业务拒绝消息；非 http_business 详情为空。",
                ),
            ],
        ),
        "RocoInvalidParamInfo" => (
            "非法参数详情。",
            vec![
                field("kind", "RocoInvalidParamKind", "结构化非法参数类型。"),
                field("kind_code", "string", "稳定非法参数类型代码。"),
                field("detail", "RocoInvalidParamDetail", "结构化非法参数详情。"),
                field(
                    "detail_kind_code",
                    "string",
                    "稳定非法参数详情代码；无详情时为空。",
                ),
                field("name", "string", "参数名。"),
                field("value", "int", "参数数值；非数值型为 0。"),
                field(
                    "expected_text",
                    "string",
                    "期望参数范围文本；无范围信息时为空。",
                ),
                field(
                    "protocol_field",
                    "string",
                    "协议层拒绝的字段名；非 protocol_rejected 错误为空。",
                ),
                field(
                    "protocol_field_name",
                    "RocoProtocolFieldName?",
                    "结构化协议层拒绝字段名；非 protocol_rejected 错误为空。",
                ),
                field(
                    "rejected_source_code",
                    "string",
                    "通用 rejected 参数错误来源代码；非 rejected 错误为空。",
                ),
                field("message", "string", "参数错误说明。"),
                field(
                    "system_operation_code",
                    "string",
                    "关联 system 操作代码；非 system 来源为空。",
                ),
                field(
                    "system_source_code",
                    "string",
                    "关联 system 失败来源代码；非 system 来源为空。",
                ),
            ],
        ),
        "RocoInvalidParamDetail" => (
            "结构化非法参数详情。",
            vec![
                field(
                    "kind_code",
                    "string",
                    "稳定详情代码，例如 expected_range、protocol_rejected、system_failure。",
                ),
                field(
                    "expected_text",
                    "string",
                    "期望参数范围文本；无范围信息时为空。",
                ),
                field(
                    "protocol_field",
                    "string",
                    "协议层拒绝字段名；非 protocol_rejected 详情为空。",
                ),
                field(
                    "protocol_field_name",
                    "RocoProtocolFieldName?",
                    "结构化协议层拒绝字段名；非 protocol_rejected 详情为空。",
                ),
                field(
                    "rejected_source_code",
                    "string",
                    "通用 rejected 参数错误来源代码；非 rejected 详情为空。",
                ),
                field(
                    "system_operation_code",
                    "string",
                    "关联 system 操作代码；非 system_failure 详情为空。",
                ),
                field(
                    "system_source_code",
                    "string",
                    "关联 system 失败来源代码；非 system_failure 详情为空。",
                ),
                field(
                    "system_message",
                    "string",
                    "关联 system 失败说明；非 system_failure 详情为空。",
                ),
            ],
        ),
        "RocoProtocolFieldName" => (
            "协议字段名。",
            vec![field("code", "string", "稳定字段代码。")],
        ),
        "RocoParamRange" => (
            "非法参数期望范围。",
            vec![
                field(
                    "kind_code",
                    "string",
                    "范围类型代码：inclusive、min_inclusive 或 type_bounds。",
                ),
                field("min", "int", "最小值；非数值范围时为 0。"),
                field("max", "int", "最大值；非闭区间范围时为 0。"),
                field(
                    "type_name",
                    "string",
                    "类型边界名称；非 type_bounds 范围时为空。",
                ),
                field("text", "string", "范围文本。"),
            ],
        ),
        "RocoInvalidParamKind" => (
            "结构化非法参数类型。",
            vec![field("code", "string", "稳定非法参数类型代码。")],
        ),
        "RocoHttpBusinessRejection" => (
            "HTTP 业务拒绝详情。",
            vec![
                field("result_code", "int", "业务结果码。"),
                field("message", "string", "业务拒绝消息。"),
                field(
                    "request_context",
                    "RocoRequestContext?",
                    "结构化请求上下文；未知时为空。",
                ),
                field(
                    "request_context_raw",
                    "string",
                    "请求上下文原始代码；未知时为空。",
                ),
                field("description", "string", "格式化后的业务拒绝说明。"),
            ],
        ),
        "RocoReturnCodeKind" => (
            "服务器返回码类型。",
            vec![
                field("code", "int", "返回码数值。"),
                field("kind_code", "string", "稳定返回码类型代码。"),
            ],
        ),
        "RocoReturnCodeRejection" => (
            "服务器返回码拒绝详情。",
            vec![
                field("kind", "RocoReturnCodeKind", "结构化返回码类型。"),
                field("kind_code", "string", "稳定返回码类型代码。"),
                field("code", "int", "返回码数值。"),
                field("message", "string", "服务器返回消息。"),
                field("description", "string", "格式化后的返回码说明。"),
            ],
        ),
        "ScriptModuleName" => (
            "结构化脚本模块名。",
            vec![field("code", "string", "稳定模块代码。")],
        ),
        "ScriptFunctionName" => (
            "结构化脚本函数名。",
            vec![
                field("module", "ScriptModuleName", "函数所在模块。"),
                field("module_code", "string", "稳定模块代码。"),
                field("function", "string", "函数名。"),
                field("qualified_name", "string", "完整函数名。"),
            ],
        ),
        "ScriptHttpResponseName" => (
            "结构化 HTTP 响应名称。",
            vec![field("code", "string", "稳定响应类型代码。")],
        ),
        "ScriptResponseName" => (
            "结构化脚本响应名称。",
            vec![field("code", "string", "稳定响应类型代码。")],
        ),
        "ScriptCombatCommandFailureKind" => (
            "结构化战斗命令失败类型。",
            vec![field("code", "string", "稳定失败类型代码。")],
        ),
        "ScriptBackendCombatRuntimeErrorKind" => (
            "结构化后端战斗运行时错误类型。",
            vec![field("code", "string", "稳定错误类型代码。")],
        ),
        "ScriptCombatRuntimeError" => (
            "结构化脚本战斗运行时错误。",
            vec![
                field("message", "string", "格式化错误消息。"),
                field(
                    "command_failure_kind",
                    "ScriptCombatCommandFailureKind?",
                    "导致自动收尾或前端加载标记失败的命令错误类型；非命令失败时为空。",
                ),
                field(
                    "command_failure_kind_code",
                    "string",
                    "稳定命令失败类型代码；非命令失败时为空字符串。",
                ),
                field(
                    "backend_kind",
                    "ScriptBackendCombatRuntimeErrorKind?",
                    "后端战斗运行时错误类型；非后端运行时错误时为空。",
                ),
                field(
                    "backend_kind_code",
                    "string",
                    "稳定后端战斗运行时错误类型代码；非后端运行时错误时为空字符串。",
                ),
            ],
        ),
        "StarTowerInfo" => ("星辰塔返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field("mop", "int", "魔法值。"),
                field("boss_id", "int", "Boss ID。"),
                field("countdown", "int", "倒计时。"),
                field("auto_sell", "bool", "是否自动出售。"),
                field("money", "int", "洛克贝。"),
                field("clips", "int[]", "碎片列表。"),
                field("storeys", "StarTowerStorey[]", "楼层列表。"),
                field("has_top", "bool", "是否有顶层信息。"),
                field("top", "RocoOptionalStarTowerTop", "结构化可选顶层信息。"),
                field(
                    "top_or_default",
                    "StarTowerTop",
                    "顶层信息；无顶层时返回默认空结构。",
                ),
            ]
        }),
        "StarTowerStorey" => (
            "星辰塔楼层信息。",
            vec![
                field("storey_index", "int", "楼层索引。"),
                field("first", "int", "首通状态。"),
                field("can_quick_fight", "bool", "是否可快速挑战。"),
                field("nodes", "StarTowerNode[]", "Boss 节点列表。"),
                field(
                    "exchange_items",
                    "StarTowerExchangeItem[]",
                    "楼层兑换碎片列表。",
                ),
            ],
        ),
        "StarTowerNode" => (
            "星辰塔 Boss 节点。",
            vec![
                field("node_index", "int", "节点索引。"),
                field("star", "int", "星级。"),
                field("spirit_id", "int", "宠物 ID。"),
                field("fight_id", "int", "战斗 ID。"),
                field("item_id", "int", "奖励物品 ID。"),
                field("reward", "int", "奖励状态。"),
                field("equip_id", "int", "装备 ID。"),
            ],
        ),
        "StarTowerExchangeItem" => (
            "星辰塔兑换碎片。",
            vec![
                field("index", "int", "兑换索引。"),
                field("item_id", "int", "碎片物品 ID。"),
                field("item_name", "string", "碎片物品名称。"),
                field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
                field("spirit_name", "string", "宠物名称。"),
                field("owned", "int", "已拥有碎片数量。"),
                field("required", "int", "所需碎片数量。"),
            ],
        ),
        "CapricornBagCandidate" => ("摩羯宫背包候选宠物。", bag_candidate_fields()),
        "CapricornTeamOperationInfo" => ("摩羯宫队伍操作返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "team",
                    "RocoOptionalCapricornTeamSnapshot",
                    "结构化可选队伍快照。",
                ),
            ]
        }),
        "CapricornSecondInfo" => ("摩羯宫二阶返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("current", "RocoOptionalI64", "当前进度。"),
                field("position", "RocoOptionalI64", "当前位置。"),
                field(
                    "second_task",
                    "RocoOptionalCapricornSecondTask",
                    "结构化可选二阶任务。",
                ),
                field(
                    "bag_candidates",
                    "CapricornBagCandidate[]",
                    "背包候选宠物。",
                ),
            ]
        }),
        "CapricornThirdInfo" => ("摩羯宫三阶返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("current", "RocoOptionalI64", "当前进度。"),
                field("remain", "RocoOptionalI64", "剩余数量。"),
                field("price", "RocoOptionalI64", "价格。"),
                field("limit", "RocoOptionalI64", "上限。"),
                field("progress_percent", "RocoOptionalI64", "进度百分比。"),
                field("reward_num", "RocoOptionalI64", "奖励数量。"),
                field("tips", "RocoOptionalI64", "提示状态。"),
                field(
                    "bag_candidates",
                    "CapricornBagCandidate[]",
                    "背包候选宠物。",
                ),
            ]
        }),
        "PiscesBagCandidate" => ("双鱼宫背包候选宠物。", bag_candidate_fields()),
        "TaurusBagCandidate" => ("金牛宫背包候选宠物。", bag_candidate_fields()),
        "ThreeStartersBagCandidate" => ("三主宠活动背包候选宠物。", bag_candidate_fields()),
        "AlchemyFurnaceBagCandidate" => ("炼丹炉背包候选宠物。", bag_candidate_fields()),
        "UnicornBagCandidate" => ("独角兽活动背包候选宠物。", bag_candidate_fields()),
        "IceCrystalBagCandidate" => ("冰晶活动背包候选宠物。", bag_candidate_fields()),
        "GeminiBagCandidate" => ("双子宫背包候选宠物。", bag_candidate_fields()),
        "SagittariusBagCandidate" => ("射手宫背包候选宠物。", bag_candidate_fields()),
        "ScorpioBagCandidate" => ("天蝎宫背包候选宠物。", bag_candidate_fields()),
        "AriesBagCandidate" => ("白羊宫背包候选宠物。", bag_candidate_fields()),
        "LibraBagCandidate" => ("天秤宫背包候选宠物。", bag_candidate_fields()),
        "LeoBagCandidate" => ("狮子宫背包候选宠物。", bag_candidate_fields()),
        "VirgoBellFoxExchangeInfo" => ("处女宫铃狐兑换结果。", exchange_display_item_fields()),
        "AquariusSecondExchangeInfo" => ("水瓶宫二阶兑换结果。", exchange_display_item_fields()),
        "AriesThirdExchangeInfo" => ("白羊宫三阶兑换结果。", exchange_display_item_fields()),
        "LibraThirdExchangeInfo" => ("天秤宫三阶兑换结果。", exchange_display_item_fields()),
        "LeoFirstExchangeInfo" => ("狮子宫一阶兑换结果。", exchange_display_item_fields()),
        "MonkeyCultivationInfo" => ("灵猴修炼返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("pill_counts", "int[]", "丹药数量。"),
                field("daytimes", "RocoOptionalI64", "当日次数。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("progress", "RocoOptionalI64", "进度。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "MonkeyEvoInfo" => ("灵猴进化返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("pill_counts", "int[]", "丹药数量。"),
                field("branch_type", "RocoOptionalI64", "进化分支类型。"),
                field("done", "RocoOptionalI64", "完成状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field(
                    "bag_candidates",
                    "AlchemyFurnaceBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "RagingFireInfo" => ("烈火活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("vip", "RocoOptionalI64", "VIP 状态。"),
                field("daytimes", "RocoOptionalI64", "当日次数。"),
                field("required_stone_indexes", "int[]", "所需石头索引。"),
                field("progress", "int[]", "三段进度。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("fusion", "RocoOptionalI64", "融合状态。"),
                field("add_progress", "RocoOptionalI64", "新增进度。"),
                field(
                    "bag_candidates",
                    "AlchemyFurnaceBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "AlchemyFurnaceRewardItem[]", "奖励列表。"),
            ]
        }),
        "UnicornBossInfo" => ("独角兽 Boss 信息。", {
            vec![
                field("slot", "int", "槽位。"),
                field("npc_index", "int", "NPC 索引。"),
                field("spirit_id", "RocoOptionalI64", "Boss 宠物 ID。"),
                field("fight_id", "RocoOptionalI64", "战斗 ID。"),
            ]
        }),
        "UnicornInfo" => ("独角兽活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("bosses", "UnicornBossInfo[]", "Boss 列表。"),
                field("finish", "RocoOptionalI64", "完成状态。"),
                field("start", "RocoOptionalI64", "开始状态。"),
                field("total", "RocoOptionalI64", "总数。"),
                field("book", "RocoOptionalI64", "图鉴状态。"),
                field("cultivation_times", "int[]", "培养次数。"),
                field("evolution_energy_costs", "int[]", "进化能量消耗。"),
                field("one_key_diamond_costs", "int[]", "一键钻石消耗。"),
                field("purple_vine_count", "RocoOptionalI64", "紫藤数量。"),
                field("energy", "RocoOptionalI64", "能量。"),
                field("fruit_count", "RocoOptionalI64", "果实数量。"),
                field("increase", "RocoOptionalI64", "增量。"),
                field("bag_candidates", "UnicornBagCandidate[]", "背包候选宠物。"),
                field("rewards", "UnicornRewardItem[]", "奖励列表。"),
            ]
        }),
        "WaterSourceInfo" => ("水源活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("battle", "RocoOptionalI64", "战斗状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("time", "RocoOptionalI64", "时间。"),
                field("increase", "RocoOptionalI64", "增量。"),
                field("water", "int[]", "水源状态数组。"),
            ]
        }),
        "FiresWillInfo" => ("火焰意志返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("num", "RocoOptionalI64", "数量。"),
                field("fire", "int[]", "火焰状态数组。"),
            ]
        }),
        "BatheSunInfo" => ("日光浴返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("fields", "ThreeStartersField[]", "字段列表。"),
                field("counters", "ThreeStartersCounter[]", "计数器列表。"),
                field("rewards", "ThreeStartersRewardItem[]", "奖励列表。"),
                field(
                    "bag_candidates",
                    "ThreeStartersBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("battle", "RocoOptionalI64", "战斗状态。"),
                field("schedule", "RocoOptionalI64", "进度计划。"),
                field("time", "RocoOptionalI64", "时间。"),
                field("num", "RocoOptionalI64", "数量。"),
                field("act", "RocoOptionalI64", "动作状态。"),
                field("times", "RocoOptionalI64", "次数。"),
                field("sun", "RocoOptionalI64", "日光值。"),
                field("add", "RocoOptionalI64", "增量。"),
            ]
        }),
        "FourSeasonsInfo" => ("四季活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("month", "RocoOptionalI64", "月份。"),
                field("map", "RocoOptionalI64", "地图。"),
                field("position_1based", "RocoOptionalI64", "1 基位置。"),
                field("times", "RocoOptionalI64", "次数。"),
                field("ticket", "RocoOptionalI64", "票数。"),
                field("used_tool_index", "RocoOptionalI64", "已使用工具索引。"),
                field("need_item_index", "RocoOptionalI64", "所需道具索引。"),
                field("add", "RocoOptionalI64", "增量。"),
                field("point", "RocoOptionalI64", "点数。"),
                field("boxes", "int[]", "格子状态。"),
                field("tools", "int[]", "工具状态。"),
                field("tool_shop_indexes", "int[]", "工具商店索引。"),
                field("tool_shop_flags", "int[]", "工具商店标记。"),
                field("pass_boxes", "int[]", "已通过格子。"),
                field("tool_costs", "int[]", "工具消耗。"),
                field("event_item_counts", "int[]", "事件道具数量。"),
                field("shop_rewards", "FourSeasonsShopRewardInfo[]", "商店奖励。"),
                field(
                    "monthly_spirit_rewards",
                    "FourSeasonsMonthlySpiritRewardInfo[]",
                    "月度宠物奖励。",
                ),
                field("rewards", "FourSeasonsRewardItem[]", "奖励列表。"),
            ]
        }),
        "DiamondTearInfo" => ("钻石泪活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("buy", "RocoOptionalI64", "购买状态。"),
                field("level", "RocoOptionalI64", "等级。"),
                field("count_down", "RocoOptionalI64", "倒计时。"),
                field("tear_state", "RocoOptionalI64", "钻石泪状态。"),
                field("rewards", "DiamondTearRewardItem[]", "奖励列表。"),
            ]
        }),
        "IceCrystalInfo" => ("冰晶活动返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("progress", "RocoOptionalI64", "进度。"),
                field("battle_times", "RocoOptionalI64", "战斗次数。"),
                field("battle_index", "RocoOptionalI64", "战斗索引。"),
                field("get_times", "RocoOptionalI64", "领取次数。"),
                field("add", "RocoOptionalI64", "增量。"),
                field("item_counts", "int[]", "道具数量。"),
                field("crystal_counts", "int[]", "冰晶数量。"),
                field("item_costs", "int[]", "道具消耗。"),
                field("one_key_diamond_costs", "int[]", "一键钻石消耗。"),
                field(
                    "current_battle",
                    "RocoOptionalIceCrystalBattleInfo",
                    "结构化可选当前战斗。",
                ),
                field(
                    "bag_candidates",
                    "IceCrystalBagCandidate[]",
                    "背包候选宠物。",
                ),
                field("rewards", "IceCrystalRewardItem[]", "奖励列表。"),
            ]
        }),
        "MultiEvolutionInfo" => ("多元进化返回信息。", {
            vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("candidates", "MultiEvolutionCandidate[]", "候选宠物列表。"),
                field("rewards", "MultiEvolutionRewardItem[]", "奖励列表。"),
                field("pet_id", "RocoOptionalI64", "进化结果宠物 ID。"),
                field("result_side", "RocoOptionalI64", "水系进化结果侧。"),
                field("item_id", "RocoOptionalI64", "助燃道具 ID。"),
                field("count", "int", "道具数量或服务器返回计数。"),
                field("available", "bool", "奖励是否可领取。"),
            ]
        }),
        "CancerSharpScorpionInfo" => ("巨蟹宫尖角蜘蛛状态。", {
            let fields = vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
                field("light_num", "int", "光数量。"),
                field("tail_num", "int", "尾巴数量。"),
                field("boss_left_hp", "int", "Boss 剩余 HP。"),
                field("boss_full_hp", "int", "Boss 总 HP。"),
                field("left_fight_count", "int", "剩余战斗次数。"),
                field("add_hit_level", "int", "追加命中等级。"),
                field("today_sum_hit", "int", "今日累计命中。"),
                field("exchange_count0", "int", "兑换计数 0。"),
                field("exchange_count1", "int", "兑换计数 1。"),
                field(
                    "display_item",
                    "RocoOptionalDisplayItem",
                    "结构化可选展示物品。",
                ),
            ];
            fields
        }),
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
                field("has_my_info", "bool", "是否有个人排行信息。"),
                field(
                    "my_info_or_default",
                    "TypeLadderRankUser",
                    "个人排行信息；缺失时为空用户占位。",
                ),
                field("users", "TypeLadderRankUser[]", "排行用户列表。"),
            ],
        ),
        "TalentRefreshResult" => (
            "Talent refresh result.",
            vec![
                field("position", "int", "Bag position."),
                field(
                    "pa_ability_old",
                    "int",
                    "Physical attack ability before refresh.",
                ),
                field(
                    "pd_ability_old",
                    "int",
                    "Physical defense ability before refresh.",
                ),
                field(
                    "ma_ability_old",
                    "int",
                    "Magic attack ability before refresh.",
                ),
                field(
                    "md_ability_old",
                    "int",
                    "Magic defense ability before refresh.",
                ),
                field("sp_ability_old", "int", "Speed ability before refresh."),
                field("hp_ability_old", "int", "HP ability before refresh."),
                field(
                    "pa_ability_new",
                    "int",
                    "Physical attack ability after refresh.",
                ),
                field(
                    "pd_ability_new",
                    "int",
                    "Physical defense ability after refresh.",
                ),
                field(
                    "ma_ability_new",
                    "int",
                    "Magic attack ability after refresh.",
                ),
                field(
                    "md_ability_new",
                    "int",
                    "Magic defense ability after refresh.",
                ),
                field("sp_ability_new", "int", "Speed ability after refresh."),
                field("hp_ability_new", "int", "HP ability after refresh."),
                field(
                    "pa_talent_old",
                    "int",
                    "Physical attack talent before refresh.",
                ),
                field(
                    "pd_talent_old",
                    "int",
                    "Physical defense talent before refresh.",
                ),
                field(
                    "ma_talent_old",
                    "int",
                    "Magic attack talent before refresh.",
                ),
                field(
                    "md_talent_old",
                    "int",
                    "Magic defense talent before refresh.",
                ),
                field("sp_talent_old", "int", "Speed talent before refresh."),
                field("hp_talent_old", "int", "HP talent before refresh."),
                field(
                    "pa_talent_new",
                    "int",
                    "Physical attack talent after refresh.",
                ),
                field(
                    "pd_talent_new",
                    "int",
                    "Physical defense talent after refresh.",
                ),
                field("ma_talent_new", "int", "Magic attack talent after refresh."),
                field(
                    "md_talent_new",
                    "int",
                    "Magic defense talent after refresh.",
                ),
                field("sp_talent_new", "int", "Speed talent after refresh."),
                field("hp_talent_new", "int", "HP talent after refresh."),
            ],
        ),
        "BloodGiftInfo" => (
            "Blood gift information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field("position", "int", "Bag position."),
                field(
                    "equipped_index",
                    "int",
                    "Currently equipped blood gift index.",
                ),
                field(
                    "options",
                    "BloodGiftOption[]",
                    "Available blood gift options.",
                ),
            ],
        ),
        "AmendNatureInfo" => (
            "Nature amendment information.",
            vec![
                field("result_code", "int", "Server result code."),
                field("message", "string", "Message returned by the server."),
                field(
                    "eligible_spirit_ids",
                    "int[]",
                    "Spirit IDs eligible for nature amendment.",
                ),
                field(
                    "candidates",
                    "AmendNatureCandidate[]",
                    "Candidate spirits for nature amendment.",
                ),
                field("new_personality", "int", "New personality enum value."),
                field("new_personality_name", "string", "New personality name."),
            ],
        ),
        "SpiritBagInfo" => (
            "Spirit bag information.",
            vec![field(
                "spirits",
                "SpiritInfo[]",
                "Spirits currently in the bag.",
            )],
        ),
        "SpiritEquipmentInfo" => (
            "Spirit equipment instance information.",
            vec![
                field("server_id", "int", "Equipment server ID."),
                field("catch_time", "int", "Equipment catch time."),
                field("base_attr", "int", "Base attribute ID."),
                field("base_value", "int", "Base attribute value."),
                field("special_attr", "int", "Special attribute ID."),
                field("special_value", "int", "Special attribute value."),
                field(
                    "spirit_id",
                    "RocoOptionalI64",
                    "Equipped spirit ID; missing when the equipment has no owner.",
                ),
                field(
                    "spirit_catch_time",
                    "RocoOptionalI64",
                    "Equipped spirit catch time; missing when the equipment has no owner.",
                ),
            ],
        ),
        "SpiritEquipmentBagInfo" => (
            "Spirit equipment bag information.",
            vec![
                field("equipment_count", "int", "Number of equipment items."),
                field("all_num", "int", "Capacity or total count."),
                field("need", "int", "Required value for the operation."),
                field("equipments", "SpiritEquipmentInfo[]", "Equipment list."),
            ],
        ),
        "ManorGroundInfo" => (
            "Manor ground information.",
            vec![
                field("ground_id", "int", "Ground ID."),
                field("ground_status", "int", "Ground status."),
                field("seed", "int", "Seed ID."),
                field("plant_status", "int", "Plant status."),
                field("current_time", "int", "Current growth time."),
                field("total_time", "int", "Total growth time."),
                field("total_produce", "int", "Total produce count."),
                field("left_produce", "int", "Remaining produce count."),
                field("has_grass", "bool", "Whether grass is present."),
                field("has_insect", "bool", "Whether insects are present."),
                field("has_fruit", "bool", "Whether fruit can be harvested."),
                field("season", "int", "Season ID."),
                field("left_row_times", "int", "Remaining row action count."),
            ],
        ),
        "ManorSeedInfo" => (
            "Manor seed information.",
            vec![
                field("item_id", "int", "Seed item ID."),
                field("item_count", "int", "Seed count."),
            ],
        ),
        "StaticItemInfo" => (
            "Static item information.",
            vec![
                field("id", "int", "Item ID."),
                field("name", "string", "Item name."),
                field("description", "string", "Item description."),
                field("unique", "bool", "Whether the item is unique."),
                field("item_type", "int", "Item type."),
                field("subtype", "int", "Item subtype."),
                field("price", "int", "Price."),
                field("expire_time", "string", "Expiration time."),
            ],
        ),
        "StaticStriveItemInfo" => (
            "Static effort item information.",
            vec![
                field("id", "int", "Item ID."),
                field("name", "string", "Item name."),
                field("item_type", "int", "Item type."),
                field("ghp", "int", "HP effort gain."),
                field("gpa", "int", "Physical attack effort gain."),
                field("gpd", "int", "Physical defense effort gain."),
                field("gma", "int", "Magic attack effort gain."),
                field("gmd", "int", "Magic defense effort gain."),
                field("gsp", "int", "Speed effort gain."),
                field("src", "string", "Resource path."),
            ],
        ),
        "AquariusRewardItem" => (
            "水瓶宫奖励项。",
            vec![
                field("item_index", "int", "奖励项索引。"),
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "数量。"),
                field("item_type", "RocoOptionalI64", "结构化可选物品类型。"),
            ],
        ),
        "AquariusBagCandidate" => (
            "水瓶宫背包候选宠物。",
            vec![
                field("candidate_index", "int", "候选项索引。"),
                field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
                field("bag_index", "RocoOptionalI64", "结构化可选背包位置。"),
                field("catch_time", "RocoOptionalI64", "结构化可选捕获时间。"),
                field("level", "RocoOptionalI64", "结构化可选等级。"),
                field("need_money", "RocoOptionalI64", "结构化可选所需洛克贝。"),
            ],
        ),
        "StaticGuardianPetPropertyInfo" => (
            "Static guardian pet property information.",
            vec![
                field("level", "int", "Level."),
                field("phase", "int", "Phase."),
                field("energy", "int", "Energy."),
                field("attack", "int", "Attack."),
                field("defend", "int", "Defense."),
                field("magic_attack", "int", "Magic attack."),
                field("magic_defend", "int", "Magic defense."),
                field(
                    "need_level_to_next_phase",
                    "int",
                    "Level required for next phase.",
                ),
            ],
        ),
        "StaticTitleInfo" => (
            "Static title information.",
            vec![
                field("id", "int", "Title ID."),
                field("title_name", "string", "Title name."),
            ],
        ),
        "StaticMagicInfo" => (
            "Static magic information.",
            vec![
                field("id", "int", "Magic ID."),
                field("name", "string", "Magic name."),
                field("item_id", "int", "Related item ID."),
                field("target", "int", "Target type."),
                field("magic_type", "int", "Magic type."),
                field("duration", "int", "Duration."),
                field("action_type", "int", "Action type."),
                field("app", "string", "Application identifier."),
                field("description", "string", "Description."),
            ],
        ),
        "StaticPluginInfo" => (
            "Static plugin information.",
            vec![
                field("name", "string", "Plugin name."),
                field("label", "string", "Display label."),
                field("domain", "string", "Domain."),
                field("version", "string", "Version."),
                field("command_type", "string", "Command type."),
                field("plugin_class", "string", "Plugin class name."),
                field("plugin_src", "string", "Plugin resource path."),
                field("plugin_url", "string", "Plugin URL."),
            ],
        ),
        "LadderMatchConfig" => (
            "Ladder match configuration.",
            vec![
                field(
                    "error",
                    "string",
                    "Configuration parse error; empty when valid.",
                ),
                field("match_rewards", "string[]", "Match reward configuration."),
                field("win_rewards", "string[]", "Win reward configuration."),
                field("season_rewards", "string[]", "Season reward configuration."),
                field(
                    "task0_descriptions",
                    "LadderQuestConfigEntry[]",
                    "Task 0 description configuration.",
                ),
                field(
                    "task1_descriptions",
                    "LadderQuestConfigEntry[]",
                    "Task 1 description configuration.",
                ),
                field(
                    "spirit_costs",
                    "LadderSpiritCostEntry[]",
                    "Spirit cost configuration.",
                ),
                field("limit_spirits", "int[]", "Limited spirit ID list."),
            ],
        ),
        "StaticTalentInfo" => (
            "Static talent information.",
            vec![
                field("id", "int", "Talent type ID."),
                field("name", "string", "Talent name."),
                field("description", "string", "Talent description."),
            ],
        ),
        "StaticSkillInfo" => (
            "Static skill information.",
            vec![
                field("id", "int", "Skill ID."),
                field("name", "string", "Skill name."),
                field("description", "string", "Skill description."),
                field("description2", "string", "Extended skill description."),
                field("power", "string", "Power."),
                field("pp_max", "int", "Maximum PP."),
                field("property", "int", "Property ID."),
                field("src", "string", "Resource path."),
                field("attack_type", "int", "Attack type."),
                field("speed", "int", "Priority speed."),
                field("damage_type", "int", "Damage type."),
                field("catch_rate", "int", "Capture-related rate."),
                field("super_form_id", "int", "Super form ID."),
                field("super_form_src", "string", "Super form resource path."),
            ],
        ),
        "SpiritBookStates" => (
            "图鉴拥有状态集合。",
            vec![
                field("uin", "int", "状态所属角色 UIN。"),
                field("count", "int", "状态条目数量。"),
                field("states", "Map", "宠物 ID 到图鉴状态的映射。"),
            ],
        ),
        "SpiritBookSpiritState" => (
            "单个宠物的图鉴拥有状态。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("state", "int", "图鉴状态枚举值。"),
                field("owned", "bool", "是否已拥有。"),
            ],
        ),
        "SpiritBookSummary" => (
            "图鉴册摘要。",
            vec![
                field("id", "int", "图鉴册 ID。"),
                field("name", "string", "图鉴册名称。"),
                field("count", "int", "条目总数。"),
            ],
        ),
        "SpiritBookEntry" => (
            "图鉴宠物条目。",
            vec![
                field("id", "int", "宠物 ID。"),
                field("starred", "bool", "是否星标。"),
                field("unknown", "bool", "是否未知条目。"),
                field("newed", "bool", "是否新条目。"),
            ],
        ),
        "SpiritBookInfo" => (
            "图鉴册详情。",
            vec![
                field("id", "int", "图鉴册 ID。"),
                field("name", "string", "图鉴册名称。"),
                field("groups", "SpiritBookGroup[]", "图鉴分组列表。"),
            ],
        ),
        "StaticSpiritInfoLookupResult" => (
            "静态宠物资料查询结果。",
            vec![
                field("ok", "bool", "是否找到宠物资料。"),
                field("code", "int", "结果码；0 表示成功，非 0 表示不存在或失败。"),
                field("message", "string", "失败原因，成功时通常为空。"),
                field(
                    "result",
                    "StaticSpiritInfo",
                    "宠物静态资料；失败时为 unknown 占位。",
                ),
            ],
        ),
        "StaticSpiritInfo" => (
            "宠物静态资料。",
            vec![
                field("id", "int", "宠物 ID。"),
                field("name", "string", "宠物名称。"),
                field("first_id", "int", "普通进化链首个形态 ID。"),
                field(
                    "evolution_edges",
                    "StaticSpiritEvolutionEdge[]",
                    "进化边列表。",
                ),
            ],
        ),
        _ => return fallback_return_doc_for(type_name, &normalized),
    };

    Some(StdlibReturnDoc {
        type_name: type_name.to_string(),
        description: description.to_string(),
        fields,
    })
}

pub fn infer_return_type(signature: &str) -> Option<String> {
    let return_type = signature.split("->").nth(1)?.trim();
    if return_type.is_empty() || return_type == "()" {
        return None;
    }
    Some(return_type.to_string())
}

fn normalize_type_name(type_name: &str) -> String {
    type_name
        .trim()
        .trim_end_matches("[]")
        .trim_start_matches("Vec<")
        .trim_end_matches('>')
        .to_string()
}

fn field(name: &str, type_name: &str, description: &str) -> StdlibFieldDoc {
    StdlibFieldDoc {
        name: name.to_string(),
        type_name: type_name.to_string(),
        description: description.to_string(),
    }
}

fn bag_candidate_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field("candidate_index", "int", "候选项索引。"),
        field("spirit_id", "RocoOptionalI64", "结构化可选宠物 ID。"),
        field("bag_index", "RocoOptionalI64", "结构化可选背包位置。"),
        field("catch_time", "RocoOptionalI64", "结构化可选捕获时间。"),
        field("level", "RocoOptionalI64", "结构化可选等级。"),
        field("need_money", "RocoOptionalI64", "结构化可选所需洛克贝。"),
    ]
}

fn exchange_display_item_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field("result_code", "int", "服务器返回结果码。"),
        field("message", "string", "服务器返回信息。"),
        field("item", "RocoOptionalDisplayItem", "结构化可选展示物品。"),
        field("light_num", "int", "光数量。"),
        field("tail_num", "int", "尾巴数量。"),
        field("exchange_count0", "int", "兑换计数 0。"),
        field("exchange_count1", "int", "兑换计数 1。"),
    ]
}

fn fallback_return_doc_for(type_name: &str, normalized: &str) -> Option<StdlibReturnDoc> {
    if has_structured_reward_kind(normalized) {
        return Some(StdlibReturnDoc {
            type_name: type_name.to_string(),
            description: "奖励条目。".to_string(),
            fields: vec![
                field("reward_id", "int", "奖励 ID。"),
                field("reward_kind", "RocoRewardKind", "结构化奖励类型。"),
                field("raw_reward_type", "int", "协议原始奖励类型。"),
                field("count", "int", "奖励数量。"),
            ],
        });
    }

    if has_request_context(normalized) {
        return Some(StdlibReturnDoc {
            type_name: type_name.to_string(),
            description: format!("{type_name} 返回信息。"),
            fields: vec![
                field("result_code", "int", "服务器返回结果码。"),
                field("message", "string", "服务器返回信息。"),
                field(
                    "request_context",
                    "RocoRequestContext",
                    "结构化请求上下文。",
                ),
            ],
        });
    }

    None
}

fn has_structured_reward_kind(type_name: &str) -> bool {
    matches!(
        type_name,
        "ThreeStartersRewardItem"
            | "MagicPioneerRewardItem"
            | "AlchemyFurnaceRewardItem"
            | "UnicornRewardItem"
            | "FourSeasonsRewardItem"
            | "DiamondTearRewardItem"
            | "IceCrystalRewardItem"
            | "MultiEvolutionRewardItem"
            | "GeminiRewardItem"
            | "SagittariusRewardItem"
    )
}

fn has_request_context(type_name: &str) -> bool {
    matches!(
        type_name,
        "CapricornStarPalaceInfo"
            | "CancerMendShapeInfo"
            | "CancerMendShapeBagInfo"
            | "CancerUnsealMemoriesInfo"
            | "CancerUnsealMemoriesBagInfo"
            | "VirgoServeGodInfo"
            | "VirgoFindHalidomInfo"
            | "VirgoBellFoxInfo"
            | "PiscesFirstInfo"
            | "PiscesSecondInfo"
            | "PiscesThirdInfo"
            | "TaurusFirstInfo"
            | "TaurusSecondInfo"
            | "TaurusThirdInfo"
            | "MagicPioneerInfo"
            | "GeminiFirstInfo"
            | "GeminiSecondInfo"
            | "GeminiThirdInfo"
            | "SagittariusFirstInfo"
            | "SagittariusSecondInfo"
            | "SagittariusThirdInfo"
            | "ScorpioFirstInfo"
            | "ScorpioSecondInfo"
            | "ScorpioThirdInfo"
            | "AriesFirstInfo"
            | "AriesSecondInfo"
            | "AriesThirdInfo"
            | "LibraFirstInfo"
            | "LibraSecondInfo"
            | "LibraThirdInfo"
            | "LeoFirstInfo"
            | "LeoSecondInfo"
            | "LeoThirdInfo"
            | "AquariusFirstInfo"
            | "AquariusSecondInfo"
            | "AquariusThirdInfo"
    )
}

fn try_error_fields() -> Vec<StdlibFieldDoc> {
    vec![
        field(
            "error_kind_code",
            "string",
            "稳定错误类型代码；无结构化错误时为空。",
        ),
        field(
            "error_detail_kind_code",
            "string",
            "稳定错误详情代码；无结构化错误或无详情时为空。",
        ),
        field(
            "error_network_kind_code",
            "string",
            "稳定网络错误子类型代码；非网络错误或无结构化错误时为空。",
        ),
        field("error_code", "string", "具体错误代码；无结构化错误时为空。"),
        field(
            "error_message",
            "string",
            "结构化错误说明；无结构化错误时为空。",
        ),
        field(
            "error_detail",
            "RocoErrorDetail",
            "结构化错误详情；无结构化错误时为空详情。",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::return_doc_for;

    #[test]
    fn fallback_info_doc_exposes_structured_request_context() {
        let doc = return_doc_for("MagicPioneerInfo").expect("return doc");

        assert!(doc.fields.iter().any(
            |field| field.name == "request_context" && field.type_name == "RocoRequestContext"
        ));
    }

    #[test]
    fn fallback_reward_doc_exposes_structured_reward_kind() {
        let doc = return_doc_for("MagicPioneerRewardItem").expect("return doc");

        assert!(doc
            .fields
            .iter()
            .any(|field| field.name == "reward_kind" && field.type_name == "RocoRewardKind"));
    }

    #[test]
    fn fallback_info_doc_does_not_invent_request_context() {
        assert!(return_doc_for("VirgoBellFoxStatusInfo").is_none());
    }

    #[test]
    fn fallback_reward_doc_does_not_invent_reward_kind() {
        assert!(return_doc_for("MiniGameRewardItem").is_none());
    }

    #[test]
    fn action_result_doc_exposes_error_summary_and_detail() {
        let doc = return_doc_for("ActionResult").expect("return doc");
        let field_names = doc
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<std::collections::HashSet<_>>();

        for name in [
            "error_kind_code",
            "error_detail_kind_code",
            "error_network_kind_code",
            "error_code",
            "error_message",
            "error_detail",
        ] {
            assert!(field_names.contains(name), "missing metadata field {name}");
        }

        for name in [
            "error_invalid_param_kind_code",
            "error_bridge_code",
            "error_bridge_operation_code",
            "error_net_response_parse_target",
            "error_return_code_kind_code",
            "error_http_business_result_code",
        ] {
            assert!(
                !field_names.contains(name),
                "stale flattened detail metadata field {name}"
            );
        }
    }
}
