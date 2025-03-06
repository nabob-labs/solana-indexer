

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdateDelegate {
    pub additional_delegates: Vec<solana_sdk::pubkey::Pubkey>,
}
