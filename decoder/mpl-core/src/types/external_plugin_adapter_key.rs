use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterKey {
    LifecycleHook(solana_sdk::pubkey::Pubkey),
    Oracle(solana_sdk::pubkey::Pubkey),
    AppData(Authority),
    LinkedLifecycleHook(solana_sdk::pubkey::Pubkey),
    LinkedAppData(Authority),
    DataSection(LinkedDataKey),
}
