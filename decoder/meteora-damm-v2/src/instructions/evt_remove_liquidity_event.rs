use solana_indexer_core::{borsh, IndexerDeserialize};

use crate::types::RemoveLiquidityParameters;

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d572e5862af60225b")]
pub struct EvtRemoveLiquidityEvent {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub params: RemoveLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
