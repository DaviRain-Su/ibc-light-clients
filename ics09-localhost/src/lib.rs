#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ibc::core::ics02_client::client_type::ClientType;

pub mod client_state;
pub mod error;
pub mod prelude;

pub(crate) const LOCALHOST_CLIENT_TYPE: &str = "09-localhost";

pub fn client_type() -> ClientType {
    ClientType::from(LOCALHOST_CLIENT_TYPE.to_string())
}
