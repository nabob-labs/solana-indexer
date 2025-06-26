use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d797f26885c370ef7")]
pub struct BootstrapLiquidityEvent {
    pub lp_mint_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub pool: solana_pubkey::Pubkey,
}
