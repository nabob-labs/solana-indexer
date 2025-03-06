use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Observation {
    pub cumulative_active_bin_id: i128,
    pub created_at: i64,
    pub last_updated_at: i64,
}
