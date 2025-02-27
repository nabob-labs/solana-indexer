use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapter {
    LifecycleHook(LifecycleHook),
    Oracle(Oracle),
    AppData(AppData),
    LinkedLifecycleHook(LinkedLifecycleHook),
    LinkedAppData(LinkedAppData),
    DataSection(DataSection),
}
