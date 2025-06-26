use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LifecycleHook {
    pub hooked_program: solana_pubkey::Pubkey,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub data_authority: Option<Authority>,
    pub schema: ExternalPluginAdapterSchema,
}
