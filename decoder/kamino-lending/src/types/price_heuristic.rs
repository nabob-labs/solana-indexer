

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PriceHeuristic {
    pub lower: u64,
    pub upper: u64,
    pub exp: u64,
}
