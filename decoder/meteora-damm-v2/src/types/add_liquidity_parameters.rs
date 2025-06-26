use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquidityParameters {
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}
