use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x5ca21a433156fc05")]
pub struct CrossMarginAccountManager {
    pub nonce: u8,
    pub authority: solana_pubkey::Pubkey,
    pub accounts: [CrossMarginAccountInfo; 20],
    pub referrer: solana_pubkey::Pubkey,
    pub airdrop_community: u8,
    pub referred_timestamp: u64,
    pub padding: [u8; 14],
}
