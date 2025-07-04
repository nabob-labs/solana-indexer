use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x95089ccaa0fcb0d9")]
pub struct GlobalConfig {
    pub global_admin: solana_pubkey::Pubkey,
    pub treasury_fee_bps: u64,
    pub treasury_vaults_authority: solana_pubkey::Pubkey,
    pub treasury_vaults_authority_bump: u64,
    pub pending_global_admin: solana_pubkey::Pubkey,
    pub padding1: [u128; 126],
}
