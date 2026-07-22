use ::rhai::Engine;

use super::super::*;
pub(super) fn register(engine: &mut Engine) {
    register_getters!(engine, RocoErrorInfo, kind, code, message, detail);
    engine.register_get("kind_code", |value: &mut RocoErrorInfo| {
        value.kind.kind_code()
    });
    engine.register_get("detail_kind_code", |value: &mut RocoErrorInfo| {
        value.detail_kind_code()
    });
    engine.register_get("kind_code", |value: &mut RocoErrorDetail| value.kind_code());
    engine.register_get("general_lock_target", |value: &mut RocoErrorDetail| {
        value.general_lock_target()
    });
    engine.register_get("general_lock_target_code", |value: &mut RocoErrorDetail| {
        value.general_lock_target_code()
    });
    engine.register_get("invalid_param_kind_code", |value: &mut RocoErrorDetail| {
        value.invalid_param_kind_code()
    });
    engine.register_get("invalid_param_name", |value: &mut RocoErrorDetail| {
        value.invalid_param_name()
    });
    engine.register_get("invalid_param_value", |value: &mut RocoErrorDetail| {
        value.invalid_param_value()
    });
    engine.register_get("invalid_param_message", |value: &mut RocoErrorDetail| {
        value.invalid_param_message()
    });
    engine.register_get("invalid_param_expected", |value: &mut RocoErrorDetail| {
        value.invalid_param_expected()
    });
    engine.register_get(
        "invalid_param_protocol_field",
        |value: &mut RocoErrorDetail| value.invalid_param_protocol_field(),
    );
    engine.register_get(
        "invalid_param_rejected_source_code",
        |value: &mut RocoErrorDetail| value.invalid_param_rejected_source_code(),
    );
    engine.register_get("bridge_channel_code", |value: &mut RocoErrorDetail| {
        value.bridge_channel_code()
    });
    engine.register_get("bridge_operation_code", |value: &mut RocoErrorDetail| {
        value.bridge_operation_code()
    });
    engine.register_get("bridge_message", |value: &mut RocoErrorDetail| {
        value.bridge_message()
    });
    engine.register_get(
        "net_response_parse_target",
        |value: &mut RocoErrorDetail| value.net_response_parse_target(),
    );
    engine.register_get(
        "net_response_parse_source_kind_code",
        |value: &mut RocoErrorDetail| value.net_response_parse_source_kind_code(),
    );
    engine.register_get(
        "net_response_parse_protocol_message",
        |value: &mut RocoErrorDetail| value.net_response_parse_protocol_message(),
    );
    engine.register_get(
        "net_response_parse_protocol_error_type",
        |value: &mut RocoErrorDetail| value.net_response_parse_protocol_error_type(),
    );
    engine.register_get(
        "net_response_parse_unexpected_cmd_id",
        |value: &mut RocoErrorDetail| value.net_response_parse_unexpected_cmd_id(),
    );
    engine.register_get("unsupported_module", |value: &mut RocoErrorDetail| {
        value.unsupported_module()
    });
    engine.register_get(
        "unsupported_function_name",
        |value: &mut RocoErrorDetail| value.unsupported_function_name(),
    );
    engine.register_get(
        "function_context_kind_code",
        |value: &mut RocoErrorDetail| value.function_context_kind_code(),
    );
    engine.register_get(
        "function_context_combat_phase_code",
        |value: &mut RocoErrorDetail| value.function_context_combat_phase_code(),
    );
    engine.register_get("query_kind_code", |value: &mut RocoErrorDetail| {
        value.query_kind_code()
    });
    engine.register_get("query_skill_index", |value: &mut RocoErrorDetail| {
        value.query_skill_index()
    });
    engine.register_get("static_data_kind_code", |value: &mut RocoErrorDetail| {
        value.static_data_kind_code()
    });
    engine.register_get("static_data_position", |value: &mut RocoErrorDetail| {
        value.static_data_position()
    });
    engine.register_get(
        "static_data_function_name",
        |value: &mut RocoErrorDetail| value.static_data_function_name(),
    );
    engine.register_get("static_data_message", |value: &mut RocoErrorDetail| {
        value.static_data_message()
    });
    engine.register_get(
        "static_data_active_config_source_code",
        |value: &mut RocoErrorDetail| value.static_data_active_config_source_code(),
    );
    engine.register_get("system_operation_code", |value: &mut RocoErrorDetail| {
        value.system_operation_code()
    });
    engine.register_get("system_source_code", |value: &mut RocoErrorDetail| {
        value.system_source_code()
    });
    engine.register_get("system_message", |value: &mut RocoErrorDetail| {
        value.system_message()
    });
    engine.register_get(
        "stdlib_bridge_operation_code",
        |value: &mut RocoErrorDetail| value.stdlib_bridge_operation_code(),
    );
    engine.register_get("stdlib_bridge_message", |value: &mut RocoErrorDetail| {
        value.stdlib_bridge_message()
    });
    engine.register_get(
        "activity_operation_kind_code",
        |value: &mut RocoErrorDetail| value.activity_operation_kind_code(),
    );
    engine.register_get(
        "activity_operation_activity_code",
        |value: &mut RocoErrorDetail| value.activity_operation_activity_code(),
    );
    engine.register_get(
        "activity_operation_field_code",
        |value: &mut RocoErrorDetail| value.activity_operation_field_code(),
    );
    engine.register_get("activity_operation_count", |value: &mut RocoErrorDetail| {
        value.activity_operation_count()
    });
    engine.register_get("activity_operation_limit", |value: &mut RocoErrorDetail| {
        value.activity_operation_limit()
    });
    engine.register_get("activity_operation_value", |value: &mut RocoErrorDetail| {
        value.activity_operation_value()
    });
    engine.register_get("combat_action_kind_code", |value: &mut RocoErrorDetail| {
        value.combat_action_kind_code()
    });
    engine.register_get("combat_action_position", |value: &mut RocoErrorDetail| {
        value.combat_action_position()
    });
    engine.register_get("combat_action_skill_id", |value: &mut RocoErrorDetail| {
        value.combat_action_skill_id()
    });
    engine.register_get("combat_action_item_id", |value: &mut RocoErrorDetail| {
        value.combat_action_item_id()
    });
    engine.register_get("combat_wait_kind_code", |value: &mut RocoErrorDetail| {
        value.combat_wait_kind_code()
    });
    engine.register_get(
        "combat_wait_combat_phase_code",
        |value: &mut RocoErrorDetail| value.combat_wait_combat_phase_code(),
    );
    engine.register_get("combat_wait_elapsed_ms", |value: &mut RocoErrorDetail| {
        value.combat_wait_elapsed_ms()
    });
    engine.register_get("combat_runtime_message", |value: &mut RocoErrorDetail| {
        value.combat_runtime_message()
    });
    engine.register_get(
        "combat_command_failure_kind",
        |value: &mut RocoErrorDetail| value.combat_command_failure_kind(),
    );
    engine.register_get(
        "combat_command_failure_kind_code",
        |value: &mut RocoErrorDetail| value.combat_command_failure_kind_code(),
    );
    engine.register_get(
        "pending_response_kind_code",
        |value: &mut RocoErrorDetail| value.pending_response_kind_code(),
    );
    engine.register_get(
        "pending_http_response_code",
        |value: &mut RocoErrorDetail| value.pending_http_response_code(),
    );
    engine.register_get(
        "expected_http_response_code",
        |value: &mut RocoErrorDetail| value.expected_http_response_code(),
    );
    engine.register_get(
        "actual_http_response_code",
        |value: &mut RocoErrorDetail| value.actual_http_response_code(),
    );
    engine.register_get(
        "pending_response_combat_phase_code",
        |value: &mut RocoErrorDetail| value.pending_response_combat_phase_code(),
    );
    engine.register_get(
        "pending_response_net_target_code",
        |value: &mut RocoErrorDetail| value.pending_response_net_target_code(),
    );
    engine.register_get(
        "pending_response_expected_action_kind",
        |value: &mut RocoErrorDetail| value.pending_response_expected_action_kind(),
    );
    engine.register_get(
        "pending_response_expected_spirit_index",
        |value: &mut RocoErrorDetail| value.pending_response_expected_spirit_index(),
    );
    engine.register_get(
        "pending_response_actual_action_kind",
        |value: &mut RocoErrorDetail| value.pending_response_actual_action_kind(),
    );
    engine.register_get(
        "pending_response_actual_spirit_index",
        |value: &mut RocoErrorDetail| value.pending_response_actual_spirit_index(),
    );
    engine.register_get("response_kind_code", |value: &mut RocoErrorDetail| {
        value.response_kind_code()
    });
    engine.register_get("expected_response_code", |value: &mut RocoErrorDetail| {
        value.expected_response_code()
    });
    engine.register_get("actual_response_code", |value: &mut RocoErrorDetail| {
        value.actual_response_code()
    });
    engine.register_get("request_kind_code", |value: &mut RocoErrorDetail| {
        value.request_kind_code()
    });
    engine.register_get(
        "request_wait_context_code",
        |value: &mut RocoErrorDetail| value.request_wait_context_code(),
    );
    engine.register_get(
        "request_system_failure_kind_code",
        |value: &mut RocoErrorDetail| value.request_system_failure_kind_code(),
    );
    engine.register_get(
        "request_combat_intent_kind_code",
        |value: &mut RocoErrorDetail| value.request_combat_intent_kind_code(),
    );
    engine.register_get(
        "request_combat_validation_kind_code",
        |value: &mut RocoErrorDetail| value.request_combat_validation_kind_code(),
    );
    engine.register_get("request_spirit_index", |value: &mut RocoErrorDetail| {
        value.request_spirit_index()
    });
    engine.register_get("request_value", |value: &mut RocoErrorDetail| {
        value.request_value()
    });
    engine.register_get(
        "request_combat_protocol_error_kind_code",
        |value: &mut RocoErrorDetail| value.request_combat_protocol_error_kind_code(),
    );
    engine.register_get(
        "request_combat_protocol_error_value",
        |value: &mut RocoErrorDetail| value.request_combat_protocol_error_value(),
    );
    engine.register_get("session_memory_kind_code", |value: &mut RocoErrorDetail| {
        value.session_memory_kind_code()
    });
    engine.register_get("session_memory_key", |value: &mut RocoErrorDetail| {
        value.session_memory_key()
    });
    engine.register_get(
        "session_memory_expected_kind_code",
        |value: &mut RocoErrorDetail| value.session_memory_expected_kind_code(),
    );
    engine.register_get(
        "session_memory_actual_kind_code",
        |value: &mut RocoErrorDetail| value.session_memory_actual_kind_code(),
    );
    engine.register_get("lookup_kind_code", |value: &mut RocoErrorDetail| {
        value.lookup_kind_code()
    });
    engine.register_get("lookup_entity_code", |value: &mut RocoErrorDetail| {
        value.lookup_entity_code()
    });
    engine.register_get("lookup_key", |value: &mut RocoErrorDetail| {
        value.lookup_key()
    });
    engine.register_get(
        "spirit_operation_kind_code",
        |value: &mut RocoErrorDetail| value.spirit_operation_kind_code(),
    );
    engine.register_get(
        "spirit_operation_spirit_id",
        |value: &mut RocoErrorDetail| value.spirit_operation_spirit_id(),
    );
    engine.register_get(
        "spirit_operation_catch_time",
        |value: &mut RocoErrorDetail| value.spirit_operation_catch_time(),
    );
    engine.register_get("return_code_kind_code", |value: &mut RocoErrorDetail| {
        value.return_code_kind_code()
    });
    engine.register_get("return_code_value", |value: &mut RocoErrorDetail| {
        value.return_code_value()
    });
    engine.register_get("return_code_message", |value: &mut RocoErrorDetail| {
        value.return_code_message()
    });
    engine.register_get(
        "http_business_result_code",
        |value: &mut RocoErrorDetail| value.http_business_result_code(),
    );
    engine.register_get(
        "http_business_request_context",
        |value: &mut RocoErrorDetail| value.http_business_request_context(),
    );
    engine.register_get("http_business_message", |value: &mut RocoErrorDetail| {
        value.http_business_message()
    });
    engine.register_get("network_kind_code", |value: &mut RocoErrorInfo| {
        value.network_kind_code()
    });
    engine.register_get("kind_code", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(RocoErrorInfo::kind_code)
            .unwrap_or_default()
    });
    engine.register_get("detail_kind_code", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(RocoErrorInfo::detail_kind_code)
            .unwrap_or_default()
    });
    engine.register_get("detail", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(|info| info.detail.clone())
            .unwrap_or(RocoErrorDetail::None)
    });
    engine.register_get("network_kind_code", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(RocoErrorInfo::network_kind_code)
            .unwrap_or_default()
    });
    engine.register_get("code", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(|error| error.code.clone())
            .unwrap_or_default()
    });
    engine.register_get("message", |value: &mut Option<RocoErrorInfo>| {
        value
            .as_ref()
            .map(|error| error.message.clone())
            .unwrap_or_default()
    });
    engine.register_get("code", |value: &mut RocoErrorKind| value.kind_code());
    engine.register_get("family_code", |value: &mut RocoErrorKind| {
        value.family_code().to_string()
    });
    engine.register_get("network_kind", |value: &mut RocoErrorKind| match value {
        RocoErrorKind::Network { kind } => Some(kind.clone()),
        _ => None,
    });
    engine.register_get("code", |value: &mut RocoGeneralLockTarget| {
        value.code().to_string()
    });
}
