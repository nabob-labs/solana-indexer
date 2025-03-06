
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddExternalPluginAdapterV1Args {
    pub init_info: ExternalPluginAdapterInitInfo,
}
