use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x032c5eecdd46ac85")]
pub struct Multisig {
    pub m: u8,
    pub n: u8,
    pub is_initialized: bool,
    pub signers: [solana_pubkey::Pubkey; 11],
}
