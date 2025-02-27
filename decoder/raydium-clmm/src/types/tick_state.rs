use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TickState {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside0_x64: u128,
    pub fee_growth_outside1_x64: u128,
    pub reward_growths_outside_x64: [u128; 3],
    pub padding: [u32; 13],
}
