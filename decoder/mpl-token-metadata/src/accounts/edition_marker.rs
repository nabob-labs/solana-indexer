use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
