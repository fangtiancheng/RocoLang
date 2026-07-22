use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetResponseParseSource {
    Protocol {
        kind: RocoProtocolParseFailureKind,
        layer: RocoProtocolParseLayer,
        error_type: RocoProtocolParseErrorType,
        reason: RocoProtocolParseReason,
    },
    UnexpectedCommand {
        cmd_id: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseFailureKind {
    Decode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseLayer {
    Wire,
    DomainResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseReason {
    ByteArrayReadOverflow {
        offset: usize,
        bytes_needed: usize,
        bytes_available: usize,
    },
    ByteArrayWriteInvalidSourceOffset {
        src_idx: usize,
        src_len: usize,
    },
    ByteArrayWriteInvalidSourceLength {
        src_idx: usize,
        write_len: usize,
        src_len: usize,
    },
    ByteArrayLengthOutOfRange {
        context: String,
        length: usize,
        max: usize,
    },
    ProtoEncode,
    ProtoDecode,
    ProtoMissingRetInfo,
    ProtoMissingRetCode,
    MissingRetInfo {
        context: RocoProtocolParseContext,
    },
    MissingRetCode,
    MissingField {
        field: RocoProtocolFieldName,
    },
    InvalidValue {
        context: RocoProtocolParseContext,
        field: RocoProtocolFieldName,
        value: i64,
    },
    MissingIndexedField {
        context: RocoProtocolParseContext,
        index: usize,
        field: RocoProtocolFieldName,
    },
    TooManyItems {
        context: RocoProtocolParseContext,
        expected_at_most: usize,
        actual: usize,
    },
    IndexOutOfBounds {
        context: RocoProtocolParseContext,
        index: usize,
        len: usize,
    },
    IndexOverflow {
        context: RocoProtocolParseContext,
        index: usize,
    },
    InvalidSpiritSex {
        value: u8,
    },
    UnsupportedListCommand {
        cmd_id: u32,
    },
    SpiritStorageMissingField {
        field: RocoSpiritStorageProtoField,
    },
    SpiritStorageIncompleteVarint {
        context: RocoSpiritStorageProtoContext,
        offset: Option<usize>,
    },
    SpiritStorageLengthOutOfRange {
        context: RocoSpiritStorageProtoContext,
        offset: Option<usize>,
    },
    SpiritStorageTruncatedPayload {
        context: RocoSpiritStorageProtoContext,
    },
    SpiritStorageIncompleteKey {
        context: RocoSpiritStorageProtoContext,
        offset: usize,
    },
    SpiritStorageBadWireType {
        context: RocoSpiritStorageProtoContext,
        wire_type: u8,
        offset: usize,
    },
    MissingSpiritTalentTail,
    MissingSpiritSkinTail,
    CombatSpiritInvalidSex {
        value: u8,
    },
    UnmappedFightEventTag {
        tag: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RocoProtocolParseContext {
    code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoSpiritStorageProtoContext {
    Push,
    RetInfoLength,
    RetInfo,
    RetCode,
    RetMsgLength,
    RetMsg,
    AlreadyAddStorageNum,
    MaxAddStorageNum,
    ResponseFlag,
    SpiritInfoLength,
    SpiritInfo,
    SpiritId,
    CatchTime,
    StorageTime,
    Sex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoSpiritStorageProtoField {
    Sex,
    SpiritId,
    CatchTime,
    StorageTime,
}

impl RocoSpiritStorageProtoField {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Sex => "sex",
            Self::SpiritId => "spirit_id",
            Self::CatchTime => "catch_time",
            Self::StorageTime => "storage_time",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Sex => "sex",
            Self::SpiritId => "spiritId",
            Self::CatchTime => "catchTime",
            Self::StorageTime => "storageTime",
        }
    }
}

impl RocoSpiritStorageProtoContext {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::RetInfoLength => "ret_info_length",
            Self::RetInfo => "ret_info",
            Self::RetCode => "ret_code",
            Self::RetMsgLength => "ret_msg_length",
            Self::RetMsg => "ret_msg",
            Self::AlreadyAddStorageNum => "already_add_storage_num",
            Self::MaxAddStorageNum => "max_add_storage_num",
            Self::ResponseFlag => "response_flag",
            Self::SpiritInfoLength => "spirit_info_length",
            Self::SpiritInfo => "spirit_info",
            Self::SpiritId => "spirit_id",
            Self::CatchTime => "catch_time",
            Self::StorageTime => "storage_time",
            Self::Sex => "sex",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Push => "spirit storage push",
            Self::RetInfoLength => "spirit storage retInfo length",
            Self::RetInfo => "spirit storage retInfo",
            Self::RetCode => "spirit storage retCode",
            Self::RetMsgLength => "spirit storage retMsg length",
            Self::RetMsg => "spirit storage retMsg",
            Self::AlreadyAddStorageNum => "spirit storage alreadyAddStorageNum",
            Self::MaxAddStorageNum => "spirit storage maxAddStorageNum",
            Self::ResponseFlag => "spirit storage reponseFlag",
            Self::SpiritInfoLength => "spirit storage spiritInfo length",
            Self::SpiritInfo => "spirit storage spiritInfo",
            Self::SpiritId => "spirit storage spiritId",
            Self::CatchTime => "spirit storage catchTime",
            Self::StorageTime => "spirit storage storageTime",
            Self::Sex => "spirit storage sex",
        }
    }
}

impl RocoProtocolParseContext {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn label(&self) -> Cow<'_, str> {
        if self.code.contains('_') {
            Cow::Owned(self.code.replace('_', " "))
        } else {
            Cow::Borrowed(self.code.as_str())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoProtocolParseErrorType {
    ByteArray,
    ProtoBody,
    SocketProtocolBody,
    SocketProtocol,
    SpiritBag,
    SpiritStorage,
    SpiritStorageProtoRead,
    SpiritStorageProtoDecode,
    CombatStart,
    CombatFightPacket,
    CombatSpiritUpgrade,
}

impl RocoProtocolParseErrorType {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ByteArray => "byte_array",
            Self::ProtoBody => "proto_body",
            Self::SocketProtocolBody => "socket_protocol_body",
            Self::SocketProtocol => "socket_protocol",
            Self::SpiritBag => "spirit_bag",
            Self::SpiritStorage => "spirit_storage",
            Self::SpiritStorageProtoRead => "spirit_storage_proto_read",
            Self::SpiritStorageProtoDecode => "spirit_storage_proto_decode",
            Self::CombatStart => "combat_start",
            Self::CombatFightPacket => "combat_fight_packet",
            Self::CombatSpiritUpgrade => "combat_spirit_upgrade",
        }
    }

    pub const fn layer(&self) -> RocoProtocolParseLayer {
        match self {
            Self::ByteArray | Self::ProtoBody | Self::SocketProtocolBody | Self::SocketProtocol => {
                RocoProtocolParseLayer::Wire
            }
            Self::SpiritBag
            | Self::SpiritStorage
            | Self::SpiritStorageProtoRead
            | Self::SpiritStorageProtoDecode
            | Self::CombatStart
            | Self::CombatFightPacket
            | Self::CombatSpiritUpgrade => RocoProtocolParseLayer::DomainResponse,
        }
    }
}

impl RocoProtocolParseFailureKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Decode => "decode",
        }
    }
}

impl RocoProtocolParseLayer {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Wire => "wire",
            Self::DomainResponse => "domain_response",
        }
    }
}

impl RocoProtocolParseReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ByteArrayReadOverflow { .. } => "byte_array_read_overflow",
            Self::ByteArrayWriteInvalidSourceOffset { .. } => {
                "byte_array_write_invalid_source_offset"
            }
            Self::ByteArrayWriteInvalidSourceLength { .. } => {
                "byte_array_write_invalid_source_length"
            }
            Self::ByteArrayLengthOutOfRange { .. } => "byte_array_length_out_of_range",
            Self::ProtoEncode => "proto_encode",
            Self::ProtoDecode => "proto_decode",
            Self::ProtoMissingRetInfo => "proto_missing_ret_info",
            Self::ProtoMissingRetCode => "proto_missing_ret_code",
            Self::MissingRetInfo { .. } => "missing_ret_info",
            Self::MissingRetCode => "missing_ret_code",
            Self::MissingField { .. } => "missing_field",
            Self::InvalidValue { .. } => "invalid_value",
            Self::MissingIndexedField { .. } => "missing_indexed_field",
            Self::TooManyItems { .. } => "too_many_items",
            Self::IndexOutOfBounds { .. } => "index_out_of_bounds",
            Self::IndexOverflow { .. } => "index_overflow",
            Self::InvalidSpiritSex { .. } => "invalid_spirit_sex",
            Self::UnsupportedListCommand { .. } => "unsupported_list_command",
            Self::SpiritStorageMissingField { .. } => "spirit_storage_missing_field",
            Self::SpiritStorageIncompleteVarint { .. } => "spirit_storage_incomplete_varint",
            Self::SpiritStorageLengthOutOfRange { .. } => "spirit_storage_length_out_of_range",
            Self::SpiritStorageTruncatedPayload { .. } => "spirit_storage_truncated_payload",
            Self::SpiritStorageIncompleteKey { .. } => "spirit_storage_incomplete_key",
            Self::SpiritStorageBadWireType { .. } => "spirit_storage_bad_wire_type",
            Self::MissingSpiritTalentTail => "missing_spirit_talent_tail",
            Self::MissingSpiritSkinTail => "missing_spirit_skin_tail",
            Self::CombatSpiritInvalidSex { .. } => "combat_spirit_invalid_sex",
            Self::UnmappedFightEventTag { .. } => "unmapped_fight_event_tag",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ByteArrayReadOverflow {
                offset,
                bytes_needed,
                bytes_available,
            } => format!(
                "ADF read overflow at offset {offset}: need {bytes_needed} bytes, available {bytes_available}"
            ),
            Self::ByteArrayWriteInvalidSourceOffset { src_idx, src_len } => {
                format!("ADF write overflow: source offset {src_idx} exceeds source length {src_len}")
            }
            Self::ByteArrayWriteInvalidSourceLength {
                src_idx,
                write_len,
                src_len,
            } => format!(
                "ADF write overflow: source range {src_idx}..{} exceeds source length {src_len}",
                src_idx.saturating_add(*write_len)
            ),
            Self::ByteArrayLengthOutOfRange {
                context,
                length,
                max,
            } => format!("ADF {context} length {length} exceeds maximum {max}"),
            Self::ProtoEncode => "proto encode failed".to_string(),
            Self::ProtoDecode => "proto decode failed".to_string(),
            Self::ProtoMissingRetInfo => "proto response missing ret_info".to_string(),
            Self::ProtoMissingRetCode => "proto response ret_info missing ret_code".to_string(),
            Self::MissingRetInfo { context } => {
                format!("{} response missing ret_info", context.label())
            }
            Self::MissingRetCode => "ret_info missing ret_code field".to_string(),
            Self::MissingField { field } => format!("response missing {field} field"),
            Self::InvalidValue {
                context,
                field,
                value,
            } => format!("{} response has invalid {field} value: {value}", context.label()),
            Self::MissingIndexedField {
                context,
                index,
                field,
            } => format!("{} item {index} missing {field} field", context.label()),
            Self::TooManyItems {
                context,
                expected_at_most,
                actual,
            } => format!(
                "{} response has too many items: expected at most {expected_at_most}, got {actual}",
                context.label()
            ),
            Self::IndexOutOfBounds {
                context,
                index,
                len,
            } => format!(
                "{} item index out of bounds: index {index}, len {len}",
                context.label()
            ),
            Self::IndexOverflow { context, index } => {
                format!("{} item index overflow: {index}", context.label())
            }
            Self::InvalidSpiritSex { value } => format!("unknown spirit sex: {value}"),
            Self::UnsupportedListCommand { cmd_id } => {
                format!("unsupported spirit storage list cmd_id 0x{cmd_id:x}")
            }
            Self::SpiritStorageMissingField { field } => {
                format!("storage spirit missing {} field", field.label())
            }
            Self::SpiritStorageIncompleteVarint { context, offset } => match offset {
                Some(offset) => {
                    format!(
                        "failed to parse {} at {offset}: incomplete varint",
                        context.label()
                    )
                }
                None => format!("failed to parse {}: incomplete varint", context.label()),
            },
            Self::SpiritStorageLengthOutOfRange { context, offset } => match offset {
                Some(offset) => {
                    format!(
                        "failed to parse {} at {offset}: length out of range",
                        context.label()
                    )
                }
                None => format!("failed to parse {}: value out of range", context.label()),
            },
            Self::SpiritStorageTruncatedPayload { context } => {
                format!("failed to parse {}: truncated payload", context.label())
            }
            Self::SpiritStorageIncompleteKey { context, offset } => {
                format!(
                    "failed to parse {}: incomplete key at {offset}",
                    context.label()
                )
            }
            Self::SpiritStorageBadWireType {
                context,
                wire_type,
                offset,
            } => format!(
                "failed to parse {}: bad wire type {wire_type} at {offset}",
                context.label()
            ),
            Self::MissingSpiritTalentTail => "combat start spirit talent tail missing".to_string(),
            Self::MissingSpiritSkinTail => "combat start spirit skin tail missing".to_string(),
            Self::CombatSpiritInvalidSex { value } => {
                format!("combat spirit sex parse failed: unknown spirit sex: {value}")
            }
            Self::UnmappedFightEventTag { tag } => format!("unmapped fight result tag {tag}"),
        }
    }

    pub fn context(&self) -> Option<RocoProtocolParseContext> {
        match self {
            Self::MissingRetInfo { context }
            | Self::InvalidValue { context, .. }
            | Self::MissingIndexedField { context, .. }
            | Self::TooManyItems { context, .. }
            | Self::IndexOutOfBounds { context, .. }
            | Self::IndexOverflow { context, .. } => Some(context.clone()),
            _ => None,
        }
    }

    pub fn context_code(&self) -> String {
        self.context()
            .map(|context| context.code().to_string())
            .unwrap_or_default()
    }

    pub fn field_name(&self) -> Option<RocoProtocolFieldName> {
        match self {
            Self::MissingField { field }
            | Self::InvalidValue { field, .. }
            | Self::MissingIndexedField { field, .. } => Some(field.clone()),
            _ => None,
        }
    }

    pub fn field_code(&self) -> String {
        self.field_name()
            .map(|field| field.code().to_string())
            .unwrap_or_default()
    }

    pub fn spirit_storage_context(&self) -> Option<RocoSpiritStorageProtoContext> {
        match self {
            Self::SpiritStorageIncompleteVarint { context, .. }
            | Self::SpiritStorageLengthOutOfRange { context, .. }
            | Self::SpiritStorageTruncatedPayload { context }
            | Self::SpiritStorageIncompleteKey { context, .. }
            | Self::SpiritStorageBadWireType { context, .. } => Some(*context),
            _ => None,
        }
    }

    pub fn spirit_storage_context_code(&self) -> String {
        self.spirit_storage_context()
            .map(|context| context.code().to_string())
            .unwrap_or_default()
    }

    pub fn spirit_storage_field(&self) -> Option<RocoSpiritStorageProtoField> {
        match self {
            Self::SpiritStorageMissingField { field } => Some(*field),
            _ => None,
        }
    }

    pub fn spirit_storage_field_code(&self) -> String {
        self.spirit_storage_field()
            .map(|field| field.code().to_string())
            .unwrap_or_default()
    }
}

impl fmt::Display for RocoProtocolParseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}
