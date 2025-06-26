use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct HolderDelegateRecord {
    pub key: Key,
    pub bump: u8,
    pub mint: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}
