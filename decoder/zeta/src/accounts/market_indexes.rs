use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x6fcd6992db895217")]
pub struct MarketIndexes {
    pub nonce: u8,
    pub initialized: bool,
    pub indexes: [u8; 138],
}
