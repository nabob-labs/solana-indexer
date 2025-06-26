use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x0c05648f7d5e1ad6")]
pub struct RestakingPool {
    pub lst_mint: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub bump: u8,
}
