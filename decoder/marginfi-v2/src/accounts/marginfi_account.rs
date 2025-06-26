use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x43b2826d7e721c2a")]
pub struct MarginfiAccount {
    pub group: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lending_account: LendingAccount,
    pub account_flags: u64,
    pub padding: [u64; 63],
}
