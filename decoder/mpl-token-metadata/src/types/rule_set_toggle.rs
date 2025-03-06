

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum RuleSetToggle {
    None,
    Clear,
    Set
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
}


