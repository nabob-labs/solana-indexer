use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x512a686f7b5992b4")]
pub struct SettlementAccount {
    pub settlement_price: u64,
    pub strikes: [u64; 23],
}
