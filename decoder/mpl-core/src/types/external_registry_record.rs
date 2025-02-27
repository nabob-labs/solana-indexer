use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ExternalRegistryRecord {
    pub plugin_type: ExternalPluginAdapterType,
    pub authority: Authority,
    pub lifecycle_checks: Option<Vec<(HookableLifecycleEvent, ExternalCheckResult)>>,
    pub offset: u64,
    pub data_offset: Option<u64>,
    pub data_len: Option<u64>,
}
