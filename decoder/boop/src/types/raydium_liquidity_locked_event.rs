use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RaydiumLiquidityLockedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub lp_amount: u64,
}
