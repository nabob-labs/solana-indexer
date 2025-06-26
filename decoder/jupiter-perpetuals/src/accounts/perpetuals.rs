use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x1ca762bf68526cc4")]
pub struct Perpetuals {
    pub permissions: Permissions,
    pub pools: Vec<solana_pubkey::Pubkey>,
    pub admin: solana_pubkey::Pubkey,
    pub transfer_authority_bump: u8,
    pub perpetuals_bump: u8,
    pub inception_time: i64,
}
