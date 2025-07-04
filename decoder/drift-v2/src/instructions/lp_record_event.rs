use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d65163626b20d8e6f")]
pub struct LpRecordEvent {
    pub ts: i64,
    pub user: solana_pubkey::Pubkey,
    pub action: LPAction,
    pub n_shares: u64,
    pub market_index: u16,
    pub delta_base_asset_amount: i64,
    pub delta_quote_asset_amount: i64,
    pub pnl: i64,
}
