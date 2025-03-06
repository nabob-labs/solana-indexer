
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum UnlockArgs {
    V1
                {
                    authorization_data: Option<AuthorizationData>,
                }
    ,
}


