

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Creator {
    pub address: solana_sdk::pubkey::Pubkey,
    pub verified: bool,
    pub share: u8,
}
