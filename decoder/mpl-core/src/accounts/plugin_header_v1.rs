use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}
