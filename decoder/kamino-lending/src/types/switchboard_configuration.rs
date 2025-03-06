

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SwitchboardConfiguration {
    pub price_aggregator: solana_sdk::pubkey::Pubkey,
    pub twap_aggregator: solana_sdk::pubkey::Pubkey,
}
