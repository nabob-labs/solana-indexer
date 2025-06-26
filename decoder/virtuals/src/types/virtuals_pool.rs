use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VirtualsPool {
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub virtual_y: u64,
    pub graduation_x: u64,
    pub state: PoolState,
    pub bump: u8,
}
