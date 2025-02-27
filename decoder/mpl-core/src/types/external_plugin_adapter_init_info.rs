use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterInitInfo {
    LifecycleHook(LifecycleHookInitInfo),
    Oracle(OracleInitInfo),
    AppData(AppDataInitInfo),
    LinkedLifecycleHook(LinkedLifecycleHookInitInfo),
    LinkedAppData(LinkedAppDataInitInfo),
    DataSection(DataSectionInitInfo),
}
