
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LinkedAppDataInitInfo {
    pub data_authority: Authority,
    pub init_plugin_authority: Option<Authority>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
