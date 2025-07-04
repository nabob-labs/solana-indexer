use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x9d1431e0d957c1fe")]
pub struct WhirlpoolsConfig {
    pub fee_authority: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub default_protocol_fee_rate: u16,
}
