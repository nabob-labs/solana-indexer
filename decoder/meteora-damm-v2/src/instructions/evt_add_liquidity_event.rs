use solana_indexer_core::{borsh, IndexerDeserialize};

use crate::types::AddLiquidityParameters;

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1daff2089d1ef7b9a9")]
pub struct EvtAddLiquidityEvent {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub params: AddLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
}
