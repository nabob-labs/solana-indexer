use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Route {
    pub dexes: Vec<Dex>,
    pub weights: Vec<u8>,
}
