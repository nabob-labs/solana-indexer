
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateMetadataAccountArgsV3 {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>,
}
