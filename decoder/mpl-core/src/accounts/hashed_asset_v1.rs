use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct HashedAssetV1 {
    pub key: Key,
    pub hash: [u8; 32],
}
