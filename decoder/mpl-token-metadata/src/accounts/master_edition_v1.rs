use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct MasterEditionV1 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
    pub printing_mint: solana_pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_pubkey::Pubkey,
}
