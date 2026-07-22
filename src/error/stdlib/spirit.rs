use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptSpiritOperationError {
    RecoverAllRequiresVip,
    StorageSpiritNotFound { spirit_id: u32, catch_time: u32 },
    MultipleStorageSpiritsMatch { spirit_id: u32 },
    StorageSpiritCatchTimeOutOfRange { catch_time: i64 },
}

impl ScriptSpiritOperationError {
    pub const fn kind_code(&self) -> &'static str {
        match self {
            Self::RecoverAllRequiresVip => "recover_all_requires_vip",
            Self::StorageSpiritNotFound { .. } => "storage_spirit_not_found",
            Self::MultipleStorageSpiritsMatch { .. } => "multiple_storage_spirits_match",
            Self::StorageSpiritCatchTimeOutOfRange { .. } => {
                "storage_spirit_catch_time_out_of_range"
            }
        }
    }

    pub const fn spirit_id(&self) -> i64 {
        match self {
            Self::StorageSpiritNotFound { spirit_id, .. }
            | Self::MultipleStorageSpiritsMatch { spirit_id } => *spirit_id as i64,
            _ => -1,
        }
    }

    pub const fn catch_time(&self) -> i64 {
        match self {
            Self::StorageSpiritNotFound { catch_time, .. } => *catch_time as i64,
            Self::StorageSpiritCatchTimeOutOfRange { catch_time } => *catch_time,
            _ => -1,
        }
    }
}

impl fmt::Display for ScriptSpiritOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecoverAllRequiresVip => f.write_str("recover_all_spirits requires VIP"),
            Self::StorageSpiritNotFound {
                spirit_id,
                catch_time,
            } => write!(
                f,
                "storage spirit not found: spirit_id={spirit_id} catch_time={catch_time}"
            ),
            Self::MultipleStorageSpiritsMatch { spirit_id } => write!(
                f,
                "multiple storage spirits match spirit_id={spirit_id}; pass catch_time to disambiguate"
            ),
            Self::StorageSpiritCatchTimeOutOfRange { catch_time } => {
                write!(f, "storage spirit catch_time out of u32 range: {catch_time}")
            }
        }
    }
}
