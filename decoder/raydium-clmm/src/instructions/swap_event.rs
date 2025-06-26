use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x40c6cde8260871e2")]
pub struct SwapEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_account0: solana_pubkey::Pubkey,
    pub token_account1: solana_pubkey::Pubkey,
    pub amount0: u64,
    pub transfer_fee0: u64,
    pub amount1: u64,
    pub transfer_fee1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}
