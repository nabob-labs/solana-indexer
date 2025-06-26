use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}
