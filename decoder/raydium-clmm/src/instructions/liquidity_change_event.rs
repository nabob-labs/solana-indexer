use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7ef0afce9e58996b")]
pub struct LiquidityChangeEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub tick: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_before: u128,
    pub liquidity_after: u128,
}
