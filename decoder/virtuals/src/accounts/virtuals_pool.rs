use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x477605cb05628774")]
pub struct VirtualsPool {
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub virtual_y: u64,
    pub graduation_x: u64,
    pub state: PoolState,
    pub bump: u8,
}
