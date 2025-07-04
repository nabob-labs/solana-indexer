use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquiditySingleSidePreciseParameter2 {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
    pub max_amount: u64,
}
