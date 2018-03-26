#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod decoder;
pub mod encoder;
pub mod types;
pub mod defines;

use self::serde::ser::Serialize;
use self::serde::de::Deserialize;

pub trait RLPSerialize: Sized {
    fn encode(&self) -> Result<types::EncodedRLP, types::RLPError>;
    fn decode(rlp: &types::RLP) -> Result<Self, types::RLPError>;
}