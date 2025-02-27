use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateGreeksArgs {
    pub index: u8,
    pub theo: u64,
    pub delta: u32,
    pub gamma: u32,
    pub volatility: u32,
}
