use crate::prelude::*;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::timestamp::Timestamp;
use ibc::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;

pub const LOCALHOST_HEADER_TYPE_URL: &str = "/ibc.lightclients.localhost.v1.Header";

/// Header defines a solo machine consensus header
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Header {}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, " Header {{...}}")
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "Header {{  }}",)
    }
}

impl ibc::core::ics02_client::header::Header for Header {
    fn height(&self) -> Height {
        todo!()
    }

    fn timestamp(&self) -> Timestamp {
        todo!()
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = ClientError;

    fn try_from(_raw: Any) -> Result<Self, Self::Error> {
        Ok(Header {})
    }
}

impl From<Header> for Any {
    fn from(_header: Header) -> Self {
        Any {
            type_url: LOCALHOST_HEADER_TYPE_URL.to_string(),
            value: vec![],
        }
    }
}
