use super::*;

impl RocoErrorDetailScriptView<'_> {
    pub(in crate::error::rhai) fn general_lock_target(&self) -> Option<RocoGeneralLockTarget> {
        match self.0 {
            RocoErrorDetail::General(RocoGeneralError::LockPoisoned { target }) => Some(*target),
            _ => None,
        }
    }

    pub(in crate::error::rhai) fn general_lock_target_code(&self) -> String {
        self.general_lock_target()
            .map(|target| target.code().to_string())
            .unwrap_or_default()
    }

    pub(in crate::error::rhai) fn invalid_param_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.kind_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn invalid_param_name(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.name.clone(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn invalid_param_value(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.value,
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn invalid_param_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.message.clone(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn invalid_param_expected(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.expected_text(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn invalid_param_protocol_field(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.protocol_field(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn invalid_param_rejected_source_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::InvalidParam(info) => info.rejected_source_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn unsupported_module(&self) -> String {
        match self.0 {
            RocoErrorDetail::UnsupportedFunction(function) => function.module_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn unsupported_function_name(&self) -> String {
        match self.0 {
            RocoErrorDetail::UnsupportedFunction(function) => function.function.clone(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn function_context_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::FunctionContext(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn function_context_combat_phase_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::FunctionContext(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn query_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Query(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn query_skill_index(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::Query(error) => error.skill_index(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn static_data_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::StaticData(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn static_data_position(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::StaticData(error) => error.position(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn static_data_function_name(&self) -> String {
        match self.0 {
            RocoErrorDetail::StaticData(error) => error.function_name(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn static_data_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::StaticData(error) => error.message(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn static_data_active_config_source_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::StaticData(error) => error.active_config_source_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn system_operation_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SystemFailure(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn system_source_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SystemFailure(failure) => failure.source_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn system_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::SystemFailure(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn stdlib_bridge_operation_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::StdlibBridge(failure) => failure.operation_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn stdlib_bridge_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::StdlibBridge(failure) => failure.message(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn activity_operation_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn activity_operation_activity_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.activity_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn activity_operation_field_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.field_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn activity_operation_count(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.count(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn activity_operation_limit(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.limit(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn activity_operation_value(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::ActivityOperation(error) => error.value(),
            _ => -1,
        }
    }
}
