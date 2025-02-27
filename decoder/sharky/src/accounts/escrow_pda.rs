use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xa8d91e0ef8c57680")]
pub struct EscrowPda {
    pub bump: u8,
}
