use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xed32a5d026fd9998")]
pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}
