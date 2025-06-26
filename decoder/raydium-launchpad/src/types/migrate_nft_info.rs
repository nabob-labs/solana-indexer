use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MigrateNftInfo {
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
}
