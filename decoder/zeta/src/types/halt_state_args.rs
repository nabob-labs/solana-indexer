use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct HaltStateArgs {
    pub asset: Asset,
    pub spot_price: u64,
    pub timestamp: u64,
}
