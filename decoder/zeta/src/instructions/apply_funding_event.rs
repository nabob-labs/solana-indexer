use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d7fca0fb7c8c0040c")]
pub struct ApplyFundingEvent {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub asset: Asset,
    pub balance_change: i64,
    pub remaining_balance: u64,
    pub funding_rate: i64,
    pub oracle_price: u64,
    pub position_size: i64,
}
