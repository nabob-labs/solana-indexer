
use super::*;
use solana_indexer_core::{borsh, IndexerDeserialize};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LiquidityParameter {
    pub amount_x: u64,
    pub amount_y: u64,
    pub bin_liquidity_dist: Vec<BinLiquidityDistribution>,
}

