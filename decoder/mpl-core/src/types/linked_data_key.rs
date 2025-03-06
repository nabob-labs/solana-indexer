
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum LinkedDataKey {
    LinkedLifecycleHook
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
    LinkedAppData
                (
                    Authority,
                )
    ,
}


