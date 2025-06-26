use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquidityIxParams {
    pub desired_base_amount_in: u64,
    pub desired_quote_amount_in: u64,
    pub initial_lp_shares: Option<u64>,
}
