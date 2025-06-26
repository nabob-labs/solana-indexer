use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x0263d7a3f01a993a")]
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub config_extension_authority: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
}
