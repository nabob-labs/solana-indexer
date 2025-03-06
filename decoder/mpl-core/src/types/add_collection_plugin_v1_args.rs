
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddCollectionPluginV1Args {
    pub plugin: Plugin,
    pub init_authority: Option<Authority>,
}
