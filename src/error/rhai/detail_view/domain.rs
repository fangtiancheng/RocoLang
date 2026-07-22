use super::*;

impl RocoErrorDetailScriptView<'_> {
    pub(in crate::error::rhai) fn session_memory_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SessionMemory(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn session_memory_key(&self) -> String {
        match self.0 {
            RocoErrorDetail::SessionMemory(error) => error.key(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn session_memory_expected_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SessionMemory(error) => error.expected_kind_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn session_memory_actual_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SessionMemory(error) => error.actual_kind_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn lookup_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Lookup(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn lookup_entity_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Lookup(error) => error.entity_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn lookup_key(&self) -> String {
        match self.0 {
            RocoErrorDetail::Lookup(error) => error.key(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn spirit_operation_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::SpiritOperation(error) => error.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn spirit_operation_spirit_id(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::SpiritOperation(error) => error.spirit_id(),
            _ => -1,
        }
    }

    pub(in crate::error::rhai) fn spirit_operation_catch_time(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::SpiritOperation(error) => error.catch_time(),
            _ => -1,
        }
    }
}
