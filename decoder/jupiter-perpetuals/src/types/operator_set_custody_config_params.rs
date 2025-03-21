use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OperatorSetCustodyConfigParams {
    pub pricing: PricingParams,
    pub hourly_funding_dbps: u64,
    pub target_ratio_bps: u64,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub max_position_size_usd: u64,
    pub jump_rate: JumpRateState,
}
