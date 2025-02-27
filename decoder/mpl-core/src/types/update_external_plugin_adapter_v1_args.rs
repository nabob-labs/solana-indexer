use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateExternalPluginAdapterV1Args {
    pub key: ExternalPluginAdapterKey,
    pub update_info: ExternalPluginAdapterUpdateInfo,
}
