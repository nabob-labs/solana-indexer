use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct PluginRegistryV1 {
    pub key: Key,
    pub registry: Vec<RegistryRecord>,
    pub external_registry: Vec<ExternalRegistryRecord>,
}
