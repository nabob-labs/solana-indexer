use solana_indexer_core::{borsh, IndexerDeserialize};

use crate::types::{SwapParameters, SwapResult};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d1b3c15d58aaabb93")]
pub struct EvtSwapEvent {
    pub pool: solana_pubkey::Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub actual_amount_in: u64,
    pub current_timestamp: u64,
}
