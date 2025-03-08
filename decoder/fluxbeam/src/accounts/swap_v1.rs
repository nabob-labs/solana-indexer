use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct SwapV1 {
    pub _padding: u8,
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee_account: solana_sdk::pubkey::Pubkey,
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}
