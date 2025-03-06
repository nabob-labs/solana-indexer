
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct HashablePluginSchema {
    pub index: u64,
    pub authority: Authority,
    pub plugin: Plugin,
}
