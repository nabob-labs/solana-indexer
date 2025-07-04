use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dec32611bc665f814")]
pub struct LiquidityDepositedIntoRaydiumEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub lp_token_amount: u64,
    pub tokens_deposited: u64,
    pub wsol_deposited: u64,
}
