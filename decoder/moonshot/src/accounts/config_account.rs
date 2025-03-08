use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug, PartialEq)]
#[indexer(discriminator = "0xbdff6146babd1866")]
pub struct ConfigAccount {
    pub migration_authority: solana_sdk::pubkey::Pubkey,
    pub backend_authority: solana_sdk::pubkey::Pubkey,
    pub config_authority: solana_sdk::pubkey::Pubkey,
    pub helio_fee: solana_sdk::pubkey::Pubkey,
    pub dex_fee: solana_sdk::pubkey::Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
    pub _reserved: [u8; 192],
}

impl Default for ConfigAccount {
    fn default() -> Self {
        Self {
            migration_authority: solana_sdk::pubkey::Pubkey::default(),
            backend_authority: solana_sdk::pubkey::Pubkey::default(),
            config_authority: solana_sdk::pubkey::Pubkey::default(),
            helio_fee: solana_sdk::pubkey::Pubkey::default(),
            dex_fee: solana_sdk::pubkey::Pubkey::default(),
            fee_bps: 0,
            dex_fee_share: 0,
            migration_fee: 0,
            marketcap_threshold: 0,
            marketcap_currency: Currency::default(),
            min_supported_decimal_places: 0,
            max_supported_decimal_places: 0,
            min_supported_token_supply: 0,
            max_supported_token_supply: 0,
            bump: 0,
            coef_b: 0,
            _reserved: [0; 192],
        }
    }
}
