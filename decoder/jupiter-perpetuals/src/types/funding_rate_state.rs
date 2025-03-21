use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FundingRateState {
    pub cumulative_interest_rate: u128,
    pub last_update: i64,
    pub hourly_funding_dbps: u64,
}
