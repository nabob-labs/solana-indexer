use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xa916f6dce5e5a4cc")]
pub struct PluginRegistryV1 {
    pub key: Key,
    pub registry: Vec<RegistryRecord>,
    pub external_registry: Vec<ExternalRegistryRecord>,
}
