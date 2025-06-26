use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xc971463ece9dd1b2")]
pub struct Token {
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub amount: u64,
    pub delegate: Option<solana_pubkey::Pubkey>,
    pub state: AccountState,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<solana_pubkey::Pubkey>,
}
