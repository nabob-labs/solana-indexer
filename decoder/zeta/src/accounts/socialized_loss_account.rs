use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x41fe8deb3c546889")]
pub struct SocializedLossAccount {
    pub nonce: u8,
    pub overbankrupt_amount: u64,
}
