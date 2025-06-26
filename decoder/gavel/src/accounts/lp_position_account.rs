use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0x65b11a2ca1f25788")]
pub struct LpPositionAccount {
    pub authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub status: u64,
    pub lp_position: LpPosition,
}
