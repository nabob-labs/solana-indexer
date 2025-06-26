use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct CollectionAuthorityRecord {
    pub key: Key,
    pub bump: u8,
    pub update_authority: Option<solana_pubkey::Pubkey>,
}
