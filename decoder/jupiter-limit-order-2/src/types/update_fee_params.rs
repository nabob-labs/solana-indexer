use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateFeeParams {
    pub normal_fee_bps: u16,
    pub stable_fee_bps: u16,
}
