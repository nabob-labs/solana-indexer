use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0xabec9473a271deae")]
pub struct PresetParameter2 {
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub reduction_factor: u16,
    pub protocol_share: u16,
    pub index: u16,
    pub base_fee_power_factor: u8,
    pub padding0: u8,
    pub padding1: [u64; 20],
}
