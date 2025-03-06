
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LinkedLifecycleHook {
    pub hooked_program: solana_sdk::pubkey::Pubkey,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub data_authority: Option<Authority>,
    pub schema: ExternalPluginAdapterSchema,
}
