use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x6985aa6e342a1cb6")]
pub struct ReferrerName {
    pub authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub name: [u8; 32],
}
