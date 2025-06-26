use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dacbd08f189af3b64")]
pub struct RaydiumLiquidityLockedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub lp_amount: u64,
}
