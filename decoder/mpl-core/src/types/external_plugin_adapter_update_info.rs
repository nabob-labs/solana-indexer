use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterUpdateInfo {
    LifecycleHook(LifecycleHookUpdateInfo),
    Oracle(OracleUpdateInfo),
    AppData(AppDataUpdateInfo),
    LinkedLifecycleHook(LinkedLifecycleHookUpdateInfo),
    LinkedAppData(LinkedAppDataUpdateInfo),
}
