use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dffca4c935be74916")]
pub struct MigrationEvent {
    pub tokens_migrated: u64,
    pub tokens_burned: u64,
    pub collateral_migrated: u64,
    pub fee: u64,
    pub label: String,
}
