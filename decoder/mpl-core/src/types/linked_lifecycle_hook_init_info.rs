use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LinkedLifecycleHookInitInfo {
    pub hooked_program: solana_pubkey::Pubkey,
    pub init_plugin_authority: Option<Authority>,
    pub lifecycle_checks: Vec<(HookableLifecycleEvent, ExternalCheckResult)>,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub data_authority: Option<Authority>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
