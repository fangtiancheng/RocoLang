use ::rhai::Engine;

use super::super::*;
use super::detail_view::{register_error_detail_getters, RocoErrorDetailScriptView};
pub(super) fn register(engine: &mut Engine) {
    register_getters!(engine, RocoErrorInfo, kind, code, message, detail);
    engine.register_get("kind_code", |value: &mut RocoErrorInfo| {
        value.kind.kind_code()
    });
    engine.register_get("detail_kind_code", |value: &mut RocoErrorInfo| {
        value.detail_kind_code()
    });
    engine.register_get("kind_code", |value: &mut RocoErrorDetail| value.kind_code());
    register_error_detail_getters!(
        engine;
        general_lock_target,
        general_lock_target_code,
        invalid_param_kind_code,
        invalid_param_name,
        invalid_param_value,
        invalid_param_message,
        invalid_param_expected,
        invalid_param_protocol_field,
        invalid_param_rejected_source_code,
        bridge_channel_code,
        bridge_operation_code,
        bridge_message,
        net_response_parse_target,
        net_response_parse_source_kind_code,
        net_response_parse_protocol_message,
        net_response_parse_protocol_error_type,
        net_response_parse_unexpected_cmd_id,
        unsupported_module,
        unsupported_function_name,
        function_context_kind_code,
        function_context_combat_phase_code,
        query_kind_code,
        query_skill_index,
        static_data_kind_code,
        static_data_position,
        static_data_function_name,
        static_data_message,
        static_data_active_config_source_code,
        system_operation_code,
        system_source_code,
        system_message,
        stdlib_bridge_operation_code,
        stdlib_bridge_message,
        activity_operation_kind_code,
        activity_operation_activity_code,
        activity_operation_field_code,
        activity_operation_count,
        activity_operation_limit,
        activity_operation_value,
        combat_action_kind_code,
        combat_action_position,
        combat_action_skill_id,
        combat_action_item_id,
        combat_wait_kind_code,
        combat_wait_combat_phase_code,
        combat_wait_elapsed_ms,
        combat_runtime_message,
        combat_command_failure_kind,
        combat_command_failure_kind_code,
        pending_response_kind_code,
        pending_http_response_code,
        expected_http_response_code,
        actual_http_response_code,
        pending_response_combat_phase_code,
        pending_response_net_target_code,
        pending_response_expected_action_kind,
        pending_response_expected_spirit_index,
        pending_response_actual_action_kind,
        pending_response_actual_spirit_index,
        response_kind_code,
        expected_response_code,
        actual_response_code,
        request_kind_code,
        request_wait_context_code,
        request_system_failure_kind_code,
        request_combat_intent_kind_code,
        request_combat_validation_kind_code,
        request_spirit_index,
        request_value,
        request_combat_protocol_error_kind_code,
        request_combat_protocol_error_value,
        session_memory_kind_code,
        session_memory_key,
        session_memory_expected_kind_code,
        session_memory_actual_kind_code,
        lookup_kind_code,
        lookup_entity_code,
        lookup_key,
        spirit_operation_kind_code,
        spirit_operation_spirit_id,
        spirit_operation_catch_time,
        return_code_kind_code,
        return_code_value,
        return_code_message,
        http_business_result_code,
        http_business_request_context,
        http_business_message,
    );
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
