

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum EscrowAuthority {
    TokenOwner,
    Creator
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
}


