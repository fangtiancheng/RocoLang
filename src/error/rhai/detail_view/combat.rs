use super::*;

impl RocoErrorDetailScriptView<'_> {
    pub(in crate::error::rhai) fn combat_action_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::CombatAction(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn combat_action_position(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::CombatAction(error) => error.position(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn combat_action_skill_id(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::CombatAction(error) => error.skill_id(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn combat_action_item_id(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::CombatAction(error) => error.item_id(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn combat_wait_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::CombatWait(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn combat_wait_combat_phase_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::CombatWait(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn combat_wait_elapsed_ms(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::CombatWait(error) => error.elapsed_ms(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn combat_runtime_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::CombatRuntime(error) => error.message(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn combat_command_failure_kind(
        &self,
    ) -> Option<ScriptCombatCommandFailureKind> {
        match self.0 {
            RocoErrorDetail::CombatRuntime(error) => error.command_failure_kind(),
            _ => None,
        }
    }

    pub(in crate::error::rhai) fn combat_command_failure_kind_code(&self) -> String {
        self.combat_command_failure_kind()
            .map(|kind| kind.code().to_string())
            .unwrap_or_default()
    }

    pub(in crate::error::rhai) fn pending_response_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn pending_http_response_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.pending_http_response_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn expected_http_response_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.expected_http_response_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn actual_http_response_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.actual_http_response_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn pending_response_combat_phase_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.combat_phase_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn pending_response_net_target_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.net_response_parse_target_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn pending_response_expected_action_kind(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.expected_action_kind(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn pending_response_expected_spirit_index(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.expected_spirit_index(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn pending_response_actual_action_kind(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.actual_action_kind(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn pending_response_actual_spirit_index(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::PendingResponse(error) => error.actual_spirit_index(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn response_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Response(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn expected_response_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Response(error) => error.expected_response_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn actual_response_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Response(error) => error.actual_response_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_wait_context_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.wait_context_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_system_failure_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.system_failure_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_combat_intent_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.combat_intent_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_combat_validation_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.combat_validation_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_spirit_index(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::Request(error) => error.spirit_index(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn request_value(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::Request(error) => error.value(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn request_combat_protocol_error_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Request(error) => error.combat_protocol_error_kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn request_combat_protocol_error_value(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::Request(error) => error.combat_protocol_error_value(),
            _ => -1,
        }
    }
}
