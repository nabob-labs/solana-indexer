
use super::*;
use solana_indexer_core::{borsh, IndexerDeserialize};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LiquidityOneSideParameter {
    pub amount: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}

