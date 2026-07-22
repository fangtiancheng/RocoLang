use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
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
        _ => return None,
    })
}
