use crate::prelude::*;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::core::timestamp::Timestamp;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;

pub const LOCALHOST_CONSENSUS_STATE_TYPE_URL: &str =
    "/ibc.lightclients.localhost.v1.ConsensusState";

/// ConsensusState defines a solo machine consensus state. The sequence of a
/// consensus state is contained in the "height" key used in storing the
/// consensus state.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug, Eq)]
pub struct ConsensusState {}

impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    fn root(&self) -> &CommitmentRoot {
        todo!()
    }

    fn timestamp(&self) -> Timestamp {
        todo!()
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = ClientError;

    fn try_from(_raw: Any) -> Result<Self, Self::Error> {
        Ok(ConsensusState {})
    }
}

impl From<ConsensusState> for Any {
    fn from(_consensus_state: ConsensusState) -> Self {
        Any {
            type_url: LOCALHOST_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: vec![],
        }
    }
}
