use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Depeg {
    pub base_virtual_price: u64,
    pub base_cache_updated: u64,
    pub depeg_type: DepegType,
}
