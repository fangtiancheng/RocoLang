use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    Some(match type_name {
        "RocoNetworkErrorKind" => (
            "结构化网络错误子类型。",
            vec![field("code", "string", "稳定网络错误子类型代码。")],
        ),
        "RocoBridgeErrorChannel" => (
            "结构化 bridge 错误通道。",
            vec![field("code", "string", "稳定 bridge 通道代码。")],
        ),
        "RocoHttpBridgeErrorKind" => (
            "结构化 HTTP bridge 错误大类。",
            vec![field("code", "string", "稳定 HTTP bridge 错误大类代码。")],
        ),
        "RocoNetBridgeErrorKind" => (
            "结构化 Net bridge 错误大类。",
            vec![field("code", "string", "稳定 Net bridge 错误大类代码。")],
        ),
        "RocoBridgeErrorKind" => (
            "结构化 bridge 错误大类。",
            vec![field("code", "string", "稳定 bridge 错误大类代码。")],
        ),
        "RocoBridgeErrorInfo" => (
            "结构化 bridge 错误详情。",
            vec![
                field("channel", "RocoBridgeErrorChannel", "bridge 通道。"),
                field("channel_code", "string", "稳定 bridge 通道代码。"),
                field("kind", "RocoBridgeErrorKind", "结构化 bridge 错误大类。"),
                field("kind_code", "string", "稳定 bridge 错误大类代码。"),
                field("operation_code", "string", "稳定 bridge 操作代码。"),
                field("message", "string", "bridge 错误说明。"),
                field("description", "string", "格式化后的 bridge 错误说明。"),
            ],
        ),
        "RocoNetResponseParseSource" => (
            "结构化网络响应解析失败来源。",
            vec![
                field("kind_code", "string", "稳定解析失败来源代码。"),
                field(
                    "protocol_kind_code",
                    "string",
                    "协议解析失败分类代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_message",
                    "string",
                    "协议解析错误说明；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason",
                    "RocoProtocolParseReason?",
                    "结构化协议解析失败原因；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason_code",
                    "string",
                    "稳定协议解析失败原因代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_error_type",
                    "string",
                    "协议解析错误来源类型代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer",
                    "RocoProtocolParseLayer?",
                    "协议解析来源层级；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer_code",
                    "string",
                    "稳定协议解析来源层级代码；非 protocol 来源为空。",
                ),
                field(
                    "unexpected_cmd_id",
                    "int",
                    "意外命令号；非 unexpected_command 来源为 0。",
                ),
            ],
        ),
        "RocoNetResponseParseFailure" => (
            "结构化网络响应解析失败详情。",
            vec![
                field("target", "RocoNetResponseParseTarget", "结构化解析目标。"),
                field("target_code", "string", "稳定解析目标代码。"),
                field("target_label", "string", "解析目标显示名。"),
                field("source", "RocoNetResponseParseSource", "解析失败来源。"),
                field("source_kind_code", "string", "稳定解析失败来源代码。"),
                field(
                    "protocol_kind_code",
                    "string",
                    "协议解析失败分类代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_message",
                    "string",
                    "协议解析错误说明；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason",
                    "RocoProtocolParseReason?",
                    "结构化协议解析失败原因；非 protocol 来源为空。",
                ),
                field(
                    "protocol_reason_code",
                    "string",
                    "稳定协议解析失败原因代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_error_type",
                    "string",
                    "协议解析错误来源类型代码；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer",
                    "RocoProtocolParseLayer?",
                    "协议解析来源层级；非 protocol 来源为空。",
                ),
                field(
                    "protocol_layer_code",
                    "string",
                    "稳定协议解析来源层级代码；非 protocol 来源为空。",
                ),
                field(
                    "unexpected_cmd_id",
                    "int",
                    "意外命令号；非 unexpected_command 来源为 0。",
                ),
                field("description", "string", "格式化后的解析失败说明。"),
            ],
        ),
        "RocoNetResponseParseTarget" => (
            "网络响应解析目标。",
            vec![
                field("code", "string", "稳定解析目标代码。"),
                field("label", "string", "解析目标显示名。"),
            ],
        ),
        "RocoProtocolParseFailureKind" => (
            "协议解析失败分类。",
            vec![field("code", "string", "稳定协议解析失败分类代码。")],
        ),
        "RocoProtocolParseReason" => (
            "结构化协议解析失败原因。",
            vec![
                field("code", "string", "稳定原因代码。"),
                field("message", "string", "格式化失败说明。"),
                field(
                    "field_name",
                    "RocoProtocolFieldName?",
                    "结构化协议字段名；仅 missing_field 和 missing_indexed_field 原因为非空。",
                ),
                field("field_code", "string", "稳定协议字段代码；无字段时为空。"),
                field(
                    "spirit_storage_field",
                    "RocoSpiritStorageProtoField?",
                    "结构化仓库精灵字段；仅 spirit_storage_missing_field 原因为非空。",
                ),
                field(
                    "spirit_storage_field_code",
                    "string",
                    "稳定仓库精灵字段代码；无字段时为空。",
                ),
            ],
        ),
        "RocoSpiritStorageProtoField" => (
            "仓库精灵 protobuf 字段。",
            vec![
                field("code", "string", "稳定字段代码。"),
                field("label", "string", "字段显示名。"),
            ],
        ),
        "RocoProtocolParseErrorType" => (
            "协议解析错误来源类型。",
            vec![field("code", "string", "稳定协议解析错误来源类型代码。")],
        ),
        "RocoProtocolParseLayer" => (
            "协议解析来源层级。",
            vec![field("code", "string", "稳定协议解析来源层级代码。")],
        ),
        "ScriptBridgeOperation" => (
            "脚本 stdlib bridge 操作类型。",
            vec![field("code", "string", "稳定脚本 bridge 操作代码。")],
        ),
        "ScriptBridgeFailureReason" => (
            "脚本 stdlib bridge 失败原因。",
            vec![
                field("code", "string", "稳定 bridge 失败原因代码。"),
                field("message", "string", "bridge 失败原因说明。"),
            ],
        ),
        "ScriptBridgeFailure" => (
            "脚本 stdlib bridge 失败详情。",
            vec![
                field("operation", "ScriptBridgeOperation", "失败的 bridge 操作。"),
                field("operation_code", "string", "稳定 bridge 操作代码。"),
                field("reason", "ScriptBridgeFailureReason", "bridge 失败原因。"),
                field("reason_code", "string", "稳定 bridge 失败原因代码。"),
                field("message", "string", "bridge 失败说明。"),
                field("description", "string", "格式化后的 bridge 失败说明。"),
            ],
        ),
        "ScriptSystemOperation" => (
            "脚本 system 操作类型。",
            vec![field("code", "string", "稳定 system 操作代码。")],
        ),
        "ScriptSystemFailure" => (
            "脚本 system 操作失败详情。",
            vec![
                field("operation", "ScriptSystemOperation", "失败的 system 操作。"),
                field("operation_code", "string", "稳定 system 操作代码。"),
                field(
                    "source",
                    "ScriptSystemFailureSource",
                    "结构化 system 失败来源。",
                ),
                field("source_code", "string", "稳定 system 失败来源代码。"),
                field("message", "string", "由结构化来源派生的 system 失败说明。"),
                field("description", "string", "格式化后的 system 失败说明。"),
            ],
        ),
        "ScriptSystemFailureSource" => (
            "脚本 system 操作失败来源。",
            vec![field("code", "string", "稳定 system 失败来源代码。")],
        ),
        _ => return None,
    })
}
