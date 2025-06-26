use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BaseFeeParameter {
    pub protocol_share: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
}
