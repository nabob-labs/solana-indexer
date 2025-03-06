use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct EditionMarkerV2 {
    pub key: Key,
    pub ledger: Vec<u8>,
}
