use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}
