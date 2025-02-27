use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateZetaGroupExpiryArgs {
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
}
