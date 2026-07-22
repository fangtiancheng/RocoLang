use super::*;

mod bridge;
mod kind;
mod protocol_parse;
mod response_parse;

pub use bridge::*;
pub use kind::*;
pub use protocol_parse::*;
pub use response_parse::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetworkError {
    ChannelClosed,
    HttpRequestFailed {
        message: String,
    },
    HttpBridgeRequestFailed {
        bridge: RocoBridgeErrorInfo,
    },
    NetBridgeRequestFailed {
        bridge: RocoBridgeErrorInfo,
    },
    NetResponseParseFailed {
        target: RocoNetResponseParseTarget,
        source: RocoNetResponseParseSource,
    },
}
