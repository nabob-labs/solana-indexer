use super::*;

use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlatformParams {
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
}
