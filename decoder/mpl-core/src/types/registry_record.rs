use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegistryRecord {
    pub plugin_type: PluginType,
    pub authority: Authority,
    pub offset: u64,
}
