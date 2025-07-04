use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d8dc7b67b9f5ed766")]
pub struct RemoveLiquidityEvent {
    pub custody_key: solana_pubkey::Pubkey,
    pub pool_key: solana_pubkey::Pubkey,
    pub lp_amount_in: u64,
    pub remove_amount_usd: u64,
    pub fee_bps: u64,
    pub remove_token_amount: u64,
    pub token_amount_after_fee: u64,
    pub post_pool_amount_usd: u128,
}
