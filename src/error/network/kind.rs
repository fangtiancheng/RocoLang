use super::super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RocoNetworkErrorKind {
    ChannelClosed,
    HttpRequestFailed,
    HttpBridgeRequestFailed,
    NetBridgeRequestFailed,
    NetResponseParseFailed,
}

impl RocoNetworkErrorKind {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ChannelClosed => "channel_closed",
            Self::HttpRequestFailed => "http_request_failed",
            Self::HttpBridgeRequestFailed => "http_bridge_request_failed",
            Self::NetBridgeRequestFailed => "net_bridge_request_failed",
            Self::NetResponseParseFailed => "net_response_parse_failed",
        }
    }
}

impl fmt::Display for RocoNetworkErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
