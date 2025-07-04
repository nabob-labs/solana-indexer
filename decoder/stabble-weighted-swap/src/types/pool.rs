use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Pool {
    pub owner: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority_bump: u8,
    pub is_active: bool,
    pub invariant: u64,
    pub swap_fee: u64,
    pub tokens: Vec<PoolToken>,
    pub pending_owner: Option<solana_pubkey::Pubkey>,
    pub max_supply: u64,
}
