use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x13ec3aed51deb7fc")]
pub struct OperationState {
    pub bump: u8,
    pub operation_owners: [solana_sdk::pubkey::Pubkey; 10],
    pub whitelist_mints: [solana_sdk::pubkey::Pubkey; 100],
}
