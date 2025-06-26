use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0x17b7f83760d8ac60")]
pub struct BondingCurve {
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub sol_reserves: u64,
    pub token_reserves: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub status: BondingCurveStatus,
}
