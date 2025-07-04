use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x641e57f9c4df9ace")]
pub struct CreatePersonalPositionEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub minter: solana_pubkey::Pubkey,
    pub nft_owner: solana_pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub deposit_amount0: u64,
    pub deposit_amount1: u64,
    pub deposit_amount0_transfer_fee: u64,
    pub deposit_amount1_transfer_fee: u64,
}
