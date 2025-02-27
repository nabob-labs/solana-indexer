use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePerpParametersArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
}
