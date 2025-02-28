use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0xe445a52e51cb9a1d40c6cde8260871e2")]
pub struct SwapEvent{
    pub dex: Dex,
    pub amount_in: u64,
    pub amount_out: u64,
}
