use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct AssetV1 {
    pub key: Key,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub update_authority: UpdateAuthority,
    pub name: String,
    pub uri: String,
    pub seq: Option<u64>,
}
