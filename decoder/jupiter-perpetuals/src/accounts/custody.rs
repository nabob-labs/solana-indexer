use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x01b830515d833f91")]
pub struct Custody {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub decimals: u8,
    pub is_stable: bool,
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub target_ratio_bps: u64,
    pub assets: Assets,
    pub funding_rate_state: FundingRateState,
    pub bump: u8,
    pub token_account_bump: u8,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub max_position_size_usd: u64,
    pub doves_oracle: solana_sdk::pubkey::Pubkey,
    pub jump_rate_state: JumpRateState,
}
