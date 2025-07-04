use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3ade563a44325538")]
pub struct DecreaseLiquidityEvent {
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub liquidity: u128,
    pub decrease_amount0: u64,
    pub decrease_amount1: u64,
    pub fee_amount0: u64,
    pub fee_amount1: u64,
    pub reward_amounts: [u64; 3],
    pub transfer_fee0: u64,
    pub transfer_fee1: u64,
}
