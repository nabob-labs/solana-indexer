use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionMetrics {
    pub total_claimed_a_fee: u64,
    pub total_claimed_b_fee: u64,
}
