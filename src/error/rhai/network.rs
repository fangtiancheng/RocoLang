use ::rhai::Engine;

use super::super::*;
pub(super) fn register(engine: &mut Engine) {
    engine.register_get("code", |value: &mut RocoNetworkErrorKind| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut RocoBridgeErrorChannel| {
        value.code().to_string()
    });
    engine.register_type_with_name::<RocoHttpBridgeErrorKind>("RocoHttpBridgeErrorKind");
    engine.register_get("code", |value: &mut RocoHttpBridgeErrorKind| {
        value.code().to_string()
    });
    engine.register_type_with_name::<RocoNetBridgeErrorKind>("RocoNetBridgeErrorKind");
    engine.register_get("code", |value: &mut RocoNetBridgeErrorKind| {
        value.code().to_string()
    });
    engine.register_type_with_name::<RocoBridgeErrorKind>("RocoBridgeErrorKind");
    engine.register_get("code", |value: &mut RocoBridgeErrorKind| {
        value.code().to_string()
    });
    register_getters!(
        engine,
        RocoBridgeErrorInfo,
        channel,
        kind,
        operation_code,
        message
    );
    engine.register_get("kind_code", |value: &mut RocoBridgeErrorInfo| {
        value.kind_code()
    });
    engine.register_get("channel_code", |value: &mut RocoBridgeErrorInfo| {
        value.channel_code()
    });
    engine.register_get("operation_code", |value: &mut RocoBridgeErrorInfo| {
        value.operation_code()
    });
    engine.register_get("description", |value: &mut RocoBridgeErrorInfo| {
        value.description()
    });
    register_getters!(engine, RocoNetResponseParseFailure, target, source);
    engine.register_get("target_code", |value: &mut RocoNetResponseParseFailure| {
        value.target_code()
    });
    engine.register_get("target_label", |value: &mut RocoNetResponseParseFailure| {
        value.target_label()
    });
    engine.register_get(
        "source_kind_code",
        |value: &mut RocoNetResponseParseFailure| value.source_kind_code(),
    );
    engine.register_get(
        "protocol_message",
        |value: &mut RocoNetResponseParseFailure| value.protocol_message(),
    );
    engine.register_get(
        "protocol_reason",
        |value: &mut RocoNetResponseParseFailure| value.protocol_reason(),
    );
    engine.register_get(
        "protocol_reason_code",
        |value: &mut RocoNetResponseParseFailure| value.protocol_reason_code(),
    );
    engine.register_get(
        "protocol_error_type",
        |value: &mut RocoNetResponseParseFailure| value.protocol_error_type_code(),
    );
    engine.register_get(
        "protocol_layer",
        |value: &mut RocoNetResponseParseFailure| value.protocol_layer(),
    );
    engine.register_get(
        "protocol_layer_code",
        |value: &mut RocoNetResponseParseFailure| value.protocol_layer_code(),
    );
    engine.register_get(
        "protocol_kind_code",
        |value: &mut RocoNetResponseParseFailure| value.protocol_kind_code(),
    );
    engine.register_get(
        "unexpected_cmd_id",
        |value: &mut RocoNetResponseParseFailure| value.unexpected_cmd_id(),
    );
    engine.register_get("description", |value: &mut RocoNetResponseParseFailure| {
        value.description()
    });
    engine.register_get("kind_code", |value: &mut RocoNetResponseParseSource| {
        value.kind_code().to_string()
    });
    engine.register_get(
        "protocol_message",
        |value: &mut RocoNetResponseParseSource| value.protocol_message(),
    );
    engine.register_get(
        "protocol_reason",
        |value: &mut RocoNetResponseParseSource| value.protocol_reason(),
    );
    engine.register_get(
        "protocol_reason_code",
        |value: &mut RocoNetResponseParseSource| value.protocol_reason_code(),
    );
    engine.register_get(
        "protocol_error_type",
        |value: &mut RocoNetResponseParseSource| value.protocol_error_type_code(),
    );
    engine.register_get(
        "protocol_layer",
        |value: &mut RocoNetResponseParseSource| value.protocol_layer(),
    );
    engine.register_get(
        "protocol_layer_code",
        |value: &mut RocoNetResponseParseSource| value.protocol_layer_code(),
    );
    engine.register_get(
        "protocol_kind_code",
        |value: &mut RocoNetResponseParseSource| value.protocol_kind_code(),
    );
    engine.register_get(
        "unexpected_cmd_id",
        |value: &mut RocoNetResponseParseSource| value.unexpected_cmd_id(),
    );
    engine.register_get("code", |value: &mut RocoNetResponseParseTarget| {
        value.code().to_string()
    });
    engine.register_get("label", |value: &mut RocoNetResponseParseTarget| {
        value.label().to_string()
    });
    engine.register_get("code", |value: &mut RocoProtocolParseFailureKind| {
        value.code().to_string()
    });
    engine.register_get("code", |value: &mut RocoProtocolParseReason| {
        value.code().to_string()
    });
    engine.register_get("message", |value: &mut RocoProtocolParseReason| {
        value.message()
    });
    engine.register_get("context_code", |value: &mut RocoProtocolParseReason| {
        value.context_code()
    });
    engine.register_get("field_name", |value: &mut RocoProtocolParseReason| {
        value.field_name()
    });
    engine.register_get("field_code", |value: &mut RocoProtocolParseReason| {
        value.field_code()
    });
    engine.register_get(
        "spirit_storage_context_code",
        |value: &mut RocoProtocolParseReason| value.spirit_storage_context_code(),
    );
    engine.register_get(
        "spirit_storage_field",
        |value: &mut RocoProtocolParseReason| value.spirit_storage_field(),
    );
    engine.register_get(
        "spirit_storage_field_code",
        |value: &mut RocoProtocolParseReason| value.spirit_storage_field_code(),
    );
    engine.register_get("code", |value: &mut Option<RocoProtocolParseReason>| {
        value
            .as_ref()
            .map(RocoProtocolParseReason::code)
            .unwrap_or_default()
    });
    engine.register_get("message", |value: &mut Option<RocoProtocolParseReason>| {
        value
            .as_ref()
            .map(RocoProtocolParseReason::message)
            .unwrap_or_default()
    });
    engine.register_get(
        "context_code",
        |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::context_code)
                .unwrap_or_default()
        },
    );
    engine.register_get(
        "field_name",
        |value: &mut Option<RocoProtocolParseReason>| {
            value.as_ref().and_then(RocoProtocolParseReason::field_name)
        },
    );
    engine.register_get(
        "field_code",
        |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::field_code)
                .unwrap_or_default()
        },
    );
    engine.register_get(
        "spirit_storage_context_code",
        |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::spirit_storage_context_code)
                .unwrap_or_default()
        },
    );
    engine.register_get(
        "spirit_storage_field",
        |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .and_then(RocoProtocolParseReason::spirit_storage_field)
        },
    );
    engine.register_get(
        "spirit_storage_field_code",
        |value: &mut Option<RocoProtocolParseReason>| {
            value
                .as_ref()
                .map(RocoProtocolParseReason::spirit_storage_field_code)
                .unwrap_or_default()
        },
    );
    engine.register_get("code", |value: &mut RocoProtocolParseContext| {
        value.code().to_string()
    });
    engine.register_get("label", |value: &mut RocoProtocolParseContext| {
        value.label().to_string()
    });
    engine.register_get("code", |value: &mut RocoSpiritStorageProtoContext| {
        value.code().to_string()
    });
    engine.register_get("label", |value: &mut RocoSpiritStorageProtoContext| {
        value.label().to_string()
    });
    engine.register_get("code", |value: &mut RocoSpiritStorageProtoField| {
        value.code().to_string()
    });
    engine.register_get("label", |value: &mut RocoSpiritStorageProtoField| {
        value.label().to_string()
    });
    engine.register_get("code", |value: &mut RocoProtocolParseErrorType| {
        value.code().to_string()
    });
    engine.register_get("layer", |value: &mut RocoProtocolParseErrorType| {
        value.layer()
    });
    engine.register_get("layer_code", |value: &mut RocoProtocolParseErrorType| {
        value.layer().code().to_string()
    });
    engine.register_get("code", |value: &mut RocoProtocolParseLayer| {
        value.code().to_string()
    });
}
