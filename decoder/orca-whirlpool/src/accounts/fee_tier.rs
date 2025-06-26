use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x384b9f4c8e44be69")]
pub struct FeeTier {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
