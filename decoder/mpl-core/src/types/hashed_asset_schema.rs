

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct HashedAssetSchema {
    pub asset_hash: [u8; 32],
    pub plugin_hashes: Vec<[u8; 32]>,
}
