use super::super::*;

impl RocoErrorDetail {
    pub fn general_lock_target(&self) -> Option<RocoGeneralLockTarget> {
        match self {
            Self::General(RocoGeneralError::LockPoisoned { target }) => Some(*target),
            _ => None,
        }
    }

    pub fn general_lock_target_code(&self) -> String {
        self.general_lock_target()
            .map(|target| target.code().to_string())
            .unwrap_or_default()
    }

    pub fn invalid_param_kind_code(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.kind_code(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_name(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.name.clone(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_value(&self) -> i64 {
        match self {
            Self::InvalidParam(info) => info.value,
            _ => 0,
        }
    }

    pub fn invalid_param_message(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.message.clone(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_expected(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.expected_text(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_protocol_field(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.protocol_field(),
            _ => String::new(),
        }
    }

    pub fn invalid_param_rejected_source_code(&self) -> String {
        match self {
            Self::InvalidParam(info) => info.rejected_source_code(),
            _ => String::new(),
        }
    }

    pub fn unsupported_module(&self) -> String {
        match self {
            Self::UnsupportedFunction(function) => function.module_code(),
            _ => String::new(),
        }
    }

    pub fn unsupported_function_name(&self) -> String {
        match self {
            Self::UnsupportedFunction(function) => function.function.clone(),
            _ => String::new(),
        }
    }

    pub fn function_context_kind_code(&self) -> String {
        match self {
            Self::FunctionContext(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn function_context_combat_phase_code(&self) -> String {
        match self {
            Self::FunctionContext(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn query_kind_code(&self) -> String {
        match self {
            Self::Query(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn query_skill_index(&self) -> i64 {
        match self {
            Self::Query(error) => error.skill_index(),
            _ => -1,
        }
    }

    pub fn static_data_kind_code(&self) -> String {
        match self {
            Self::StaticData(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn static_data_position(&self) -> i64 {
        match self {
            Self::StaticData(error) => error.position(),
            _ => -1,
        }
    }

    pub fn static_data_function_name(&self) -> String {
        match self {
            Self::StaticData(error) => error.function_name(),
            _ => String::new(),
        }
    }

    pub fn static_data_message(&self) -> String {
        match self {
            Self::StaticData(error) => error.message(),
            _ => String::new(),
        }
    }

    pub fn static_data_active_config_source_code(&self) -> String {
        match self {
            Self::StaticData(error) => error.active_config_source_code(),
            _ => String::new(),
        }
    }

    pub fn system_operation_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn system_source_code(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.source_code(),
            _ => String::new(),
        }
    }

    pub fn system_message(&self) -> String {
        match self {
            Self::SystemFailure(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub fn stdlib_bridge_operation_code(&self) -> String {
        match self {
            Self::StdlibBridge(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub fn stdlib_bridge_message(&self) -> String {
        match self {
            Self::StdlibBridge(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_kind_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_activity_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.activity_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_field_code(&self) -> String {
        match self {
            Self::ActivityOperation(error) => error.field_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn activity_operation_count(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.count(),
            _ => -1,
        }
    }

    pub fn activity_operation_limit(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.limit(),
            _ => -1,
        }
    }

    pub fn activity_operation_value(&self) -> i64 {
        match self {
            Self::ActivityOperation(error) => error.value(),
            _ => -1,
        }
    }

    pub fn combat_action_kind_code(&self) -> String {
        match self {
            Self::CombatAction(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn combat_action_position(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.position(),
            _ => -1,
        }
    }

    pub fn combat_action_skill_id(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.skill_id(),
            _ => -1,
        }
    }

    pub fn combat_action_item_id(&self) -> i64 {
        match self {
            Self::CombatAction(error) => error.item_id(),
            _ => -1,
        }
    }

    pub fn combat_wait_kind_code(&self) -> String {
        match self {
            Self::CombatWait(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn combat_wait_combat_phase_code(&self) -> String {
        match self {
            Self::CombatWait(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn combat_wait_elapsed_ms(&self) -> i64 {
        match self {
            Self::CombatWait(error) => error.elapsed_ms(),
            _ => -1,
        }
    }

    pub fn combat_runtime_message(&self) -> String {
        match self {
            Self::CombatRuntime(error) => error.message(),
            _ => String::new(),
        }
    }

    pub fn combat_command_failure_kind(&self) -> Option<ScriptCombatCommandFailureKind> {
        match self {
            Self::CombatRuntime(error) => error.command_failure_kind(),
            _ => None,
        }
    }

    pub fn combat_command_failure_kind_code(&self) -> String {
        self.combat_command_failure_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }

    pub fn pending_response_kind_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn pending_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.pending_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn expected_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.expected_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn actual_http_response_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.actual_http_response_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_combat_phase_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_net_target_code(&self) -> String {
        match self {
            Self::PendingResponse(error) => error.net_response_parse_target_code(),
            _ => String::new(),
        }
    }

    pub fn pending_response_expected_action_kind(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.expected_action_kind(),
            _ => 0,
        }
    }

    pub fn pending_response_expected_spirit_index(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.expected_spirit_index(),
            _ => 0,
        }
    }

    pub fn pending_response_actual_action_kind(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.actual_action_kind(),
            _ => 0,
        }
    }

    pub fn pending_response_actual_spirit_index(&self) -> i64 {
        match self {
            Self::PendingResponse(error) => error.actual_spirit_index(),
            _ => 0,
        }
    }

    pub fn response_kind_code(&self) -> String {
        match self {
            Self::Response(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn expected_response_code(&self) -> String {
        match self {
            Self::Response(error) => error.expected_response_code(),
            _ => String::new(),
        }
    }

    pub fn actual_response_code(&self) -> String {
        match self {
            Self::Response(error) => error.actual_response_code(),
            _ => String::new(),
        }
    }

    pub fn request_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_wait_context_code(&self) -> String {
        match self {
            Self::Request(error) => error.wait_context_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_system_failure_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.system_failure_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_intent_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_intent_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_validation_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_validation_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_spirit_index(&self) -> i64 {
        match self {
            Self::Request(error) => error.spirit_index(),
            _ => -1,
        }
    }

    pub fn request_value(&self) -> i64 {
        match self {
            Self::Request(error) => error.value(),
            _ => -1,
        }
    }

    pub fn request_combat_protocol_error_kind_code(&self) -> String {
        match self {
            Self::Request(error) => error.combat_protocol_error_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn request_combat_protocol_error_value(&self) -> i64 {
        match self {
            Self::Request(error) => error.combat_protocol_error_value(),
            _ => -1,
        }
    }

    pub fn session_memory_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn session_memory_key(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.key(),
            _ => String::new(),
        }
    }

    pub fn session_memory_expected_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.expected_kind_code(),
            _ => String::new(),
        }
    }

    pub fn session_memory_actual_kind_code(&self) -> String {
        match self {
            Self::SessionMemory(error) => error.actual_kind_code(),
            _ => String::new(),
        }
    }

    pub fn lookup_kind_code(&self) -> String {
        match self {
            Self::Lookup(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn lookup_entity_code(&self) -> String {
        match self {
            Self::Lookup(error) => error.entity_code(),
            _ => String::new(),
        }
    }

    pub fn lookup_key(&self) -> String {
        match self {
            Self::Lookup(error) => error.key(),
            _ => String::new(),
        }
    }

    pub fn spirit_operation_kind_code(&self) -> String {
        match self {
            Self::SpiritOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn spirit_operation_spirit_id(&self) -> i64 {
        match self {
            Self::SpiritOperation(error) => error.spirit_id(),
            _ => -1,
        }
    }

    pub fn spirit_operation_catch_time(&self) -> i64 {
        match self {
            Self::SpiritOperation(error) => error.catch_time(),
            _ => -1,
        }
    }

    pub fn bridge_channel_code(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.channel_code(),
            _ => String::new(),
        }
    }

    pub fn bridge_operation_code(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.operation_code(),
            _ => String::new(),
        }
    }

    pub fn bridge_message(&self) -> String {
        match self {
            Self::Bridge(bridge) => bridge.message.clone(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target(&self) -> String {
        self.net_response_parse_target_code()
    }

    pub fn net_response_parse_target_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.target_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_target_label(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.target_label(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_source_kind_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.source_kind_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_protocol_message(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.protocol_message(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_protocol_error_type(&self) -> String {
        self.net_response_parse_protocol_error_type_code()
    }

    pub fn net_response_parse_protocol_error_type_code(&self) -> String {
        match self {
            Self::NetResponseParse(failure) => failure.protocol_error_type_code(),
            _ => String::new(),
        }
    }

    pub fn net_response_parse_unexpected_cmd_id(&self) -> i64 {
        match self {
            Self::NetResponseParse(failure) => failure.unexpected_cmd_id(),
            _ => 0,
        }
    }

    pub fn return_code_kind_code(&self) -> String {
        match self {
            Self::ReturnCode(rejection) => rejection.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub fn return_code_value(&self) -> i64 {
        match self {
            Self::ReturnCode(rejection) => i64::from(rejection.code()),
            _ => 0,
        }
    }

    pub fn return_code_message(&self) -> String {
        match self {
            Self::ReturnCode(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }

    pub fn http_business_result_code(&self) -> i64 {
        match self {
            Self::HttpBusiness(rejection) => rejection.result_code(),
            _ => 0,
        }
    }

    pub fn http_business_request_context(&self) -> String {
        match self {
            Self::HttpBusiness(rejection) => rejection.request_context(),
            _ => String::new(),
        }
    }

    pub fn http_business_message(&self) -> String {
        match self {
            Self::HttpBusiness(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }
}
