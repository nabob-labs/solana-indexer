use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xb617adf097ceb643")]
pub struct MarginfiGroup {
    pub admin: solana_pubkey::Pubkey,
    pub padding0: [u128; 32],
    pub padding1: [u128; 32],
}
