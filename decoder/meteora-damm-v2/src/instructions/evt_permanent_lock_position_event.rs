use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d918fa2dada50430b")]
pub struct EvtPermanentLockPositionEvent {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub lock_liquidity_amount: u128,
    pub total_permanent_locked_liquidity: u128,
}
