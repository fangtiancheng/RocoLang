use ::rhai::Engine;

use super::super::*;
pub(super) fn register(engine: &mut Engine) {
    register_getters!(
        engine,
        RocoInvalidParamInfo,
        kind,
        name,
        value,
        detail,
        message
    );
    engine.register_get("kind_code", |value: &mut RocoInvalidParamInfo| {
        value.kind_code()
    });
    engine.register_get("detail_kind_code", |value: &mut RocoInvalidParamInfo| {
        value.detail_kind_code()
    });
    engine.register_get("expected_text", |value: &mut RocoInvalidParamInfo| {
        value.expected_text()
    });
    engine.register_get("protocol_field", |value: &mut RocoInvalidParamInfo| {
        value.protocol_field()
    });
    engine.register_get("protocol_field_name", |value: &mut RocoInvalidParamInfo| {
        value.protocol_field_name()
    });
    engine.register_get(
        "rejected_source_code",
        |value: &mut RocoInvalidParamInfo| value.rejected_source_code(),
    );
    engine.register_get("code", |value: &mut RocoInvalidParamKind| {
        value.code().to_string()
    });
    engine.register_get("kind_code", |value: &mut RocoInvalidParamDetail| {
        value.kind_code()
    });
    engine.register_get("expected_text", |value: &mut RocoInvalidParamDetail| {
        value.expected_text()
    });
    engine.register_get("protocol_field", |value: &mut RocoInvalidParamDetail| {
        value.protocol_field()
    });
    engine.register_get(
        "protocol_field_name",
        |value: &mut RocoInvalidParamDetail| value.protocol_field_name(),
    );
    engine.register_get(
        "rejected_source_code",
        |value: &mut RocoInvalidParamDetail| value.rejected_source_code(),
    );
    engine.register_get("code", |value: &mut RocoProtocolFieldName| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut Option<RocoProtocolFieldName>| {
        value
            .as_ref()
            .map(|field| field.code().to_string())
            .unwrap_or_default()
    });
    engine.register_get(
        "system_operation_code",
        |value: &mut RocoInvalidParamDetail| value.system_operation_code(),
    );
    engine.register_get(
        "system_source_code",
        |value: &mut RocoInvalidParamDetail| value.system_source_code(),
    );
    engine.register_get("system_message", |value: &mut RocoInvalidParamDetail| {
        value.system_message()
    });
    engine.register_get("kind_code", |value: &mut RocoParamRange| {
        value.kind_code().to_string()
    });
    engine.register_get("min", |value: &mut RocoParamRange| value.min_value());
    engine.register_get("max", |value: &mut RocoParamRange| value.max_value());
    engine.register_get("type_name", |value: &mut RocoParamRange| value.type_name());
    engine.register_get("text", |value: &mut RocoParamRange| value.to_string());
    engine.register_get("kind_code", |value: &mut Option<RocoParamRange>| {
        value
            .as_ref()
            .map(|range| range.kind_code().to_string())
            .unwrap_or_default()
    });
    engine.register_get("min", |value: &mut Option<RocoParamRange>| {
        value
            .as_ref()
            .map(RocoParamRange::min_value)
            .unwrap_or_default()
    });
    engine.register_get("max", |value: &mut Option<RocoParamRange>| {
        value
            .as_ref()
            .map(RocoParamRange::max_value)
            .unwrap_or_default()
    });
    engine.register_get("type_name", |value: &mut Option<RocoParamRange>| {
        value
            .as_ref()
            .map(RocoParamRange::type_name)
            .unwrap_or_default()
    });
    engine.register_get("text", |value: &mut Option<RocoParamRange>| {
        value.as_ref().map(ToString::to_string).unwrap_or_default()
    });
    engine.register_get(
        "system_operation_code",
        |value: &mut RocoInvalidParamInfo| value.system_operation_code(),
    );
    engine.register_get("system_source_code", |value: &mut RocoInvalidParamInfo| {
        value.system_source_code()
    });
    engine.register_get("system_message", |value: &mut RocoInvalidParamInfo| {
        value.detail.system_message()
    });
}
