

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ScopeConfiguration {
    pub price_feed: solana_sdk::pubkey::Pubkey,
    pub price_chain: [u16; 4],
    pub twap_chain: [u16; 4],
}
