use super::*;

impl RocoErrorDetailScriptView<'_> {
    pub(in crate::error::rhai) fn bridge_channel_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Bridge(bridge) => bridge.channel_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn bridge_operation_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::Bridge(bridge) => bridge.operation_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn bridge_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::Bridge(bridge) => bridge.message.clone(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn net_response_parse_target(&self) -> String {
        self.net_response_parse_target_code()
    }

    pub(in crate::error::rhai) fn net_response_parse_target_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::NetResponseParse(failure) => failure.target_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn net_response_parse_source_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::NetResponseParse(failure) => failure.source_kind_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn net_response_parse_protocol_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::NetResponseParse(failure) => failure.protocol_message(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn net_response_parse_protocol_error_type(&self) -> String {
        self.net_response_parse_protocol_error_type_code()
    }

    pub(in crate::error::rhai) fn net_response_parse_protocol_error_type_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::NetResponseParse(failure) => failure.protocol_error_type_code(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn net_response_parse_unexpected_cmd_id(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::NetResponseParse(failure) => failure.unexpected_cmd_id(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn return_code_kind_code(&self) -> String {
        match self.0 {
            RocoErrorDetail::ReturnCode(rejection) => rejection.kind_code().to_string(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn return_code_value(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::ReturnCode(rejection) => i64::from(rejection.code()),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn return_code_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::ReturnCode(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn http_business_result_code(&self) -> i64 {
        match self.0 {
            RocoErrorDetail::HttpBusiness(rejection) => rejection.result_code(),
            _ => 0,
        }
    }

    pub(in crate::error::rhai) fn http_business_request_context(&self) -> String {
        match self.0 {
            RocoErrorDetail::HttpBusiness(rejection) => rejection.request_context(),
            _ => String::new(),
        }
    }

    pub(in crate::error::rhai) fn http_business_message(&self) -> String {
        match self.0 {
            RocoErrorDetail::HttpBusiness(rejection) => rejection.message.clone(),
            _ => String::new(),
        }
    }
}
