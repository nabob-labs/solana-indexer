use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateDecreasePositionRequest2Params {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
}
