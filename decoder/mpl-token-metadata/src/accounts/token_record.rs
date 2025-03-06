use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct TokenRecord {
    pub key: Key,
    pub bump: u8,
    pub state: TokenState,
    pub rule_set_revision: Option<u64>,
    pub delegate: Option<solana_sdk::pubkey::Pubkey>,
    pub delegate_role: Option<TokenDelegateRole>,
    pub locked_transfer: Option<solana_sdk::pubkey::Pubkey>,
}
