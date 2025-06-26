use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xd308e82b02987577")]
pub struct Vault {
    pub admin: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub withdraw_authority_bump: u8,
    pub authority_bump: u8,
    pub is_active: bool,
    pub beneficiary: solana_pubkey::Pubkey,
    pub beneficiary_fee: u64,
    pub pending_admin: Option<solana_pubkey::Pubkey>,
}
