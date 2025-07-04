use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x314f69d420221e54")]
pub struct IncreaseLiquidityEvent {
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub liquidity: u128,
    pub amount0: u64,
    pub amount1: u64,
    pub amount0_transfer_fee: u64,
    pub amount1_transfer_fee: u64,
}
