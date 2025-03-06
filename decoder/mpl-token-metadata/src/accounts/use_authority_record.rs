use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct UseAuthorityRecord {
    pub key: Key,
    pub allowed_uses: u64,
    pub bump: u8,
}
