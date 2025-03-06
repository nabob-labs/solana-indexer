use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x8bc283b38cb3e5f4")]
pub struct Oracle {
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
}
