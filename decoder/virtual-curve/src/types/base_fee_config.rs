use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BaseFeeConfig {
    pub cliff_fee_numerator: u64,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub number_of_period: u16,
    pub fee_scheduler_mode: u8,
    pub padding_0: [u8; 5],
}
