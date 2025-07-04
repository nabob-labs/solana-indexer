use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xaabc8fe47a40f7d0")]
pub struct Position {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub side: Side,
    pub price: u64,
    pub size_usd: u64,
    pub collateral_usd: u64,
    pub realised_pnl_usd: i64,
    pub cumulative_interest_snapshot: u128,
    pub locked_amount: u64,
    pub bump: u8,
}
