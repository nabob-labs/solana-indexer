

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum AssetTier {
    Regular,
    IsolatedCollateral,
    IsolatedDebt,
}


