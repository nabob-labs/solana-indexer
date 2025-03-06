
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ApproveCollectionPluginAuthorityV1Args {
    pub plugin_type: PluginType,
    pub new_authority: Authority,
}
