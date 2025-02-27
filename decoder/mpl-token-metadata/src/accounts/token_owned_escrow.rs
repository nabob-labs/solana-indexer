use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct TokenOwnedEscrow {
    pub key: Key,
    pub base_token: solana_sdk::pubkey::Pubkey,
    pub authority: EscrowAuthority,
    pub bump: u8,
}
