use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x773b3d13a55439af")]
pub struct EventHeap {
    pub header: EventHeapHeader,
    pub nodes: [EventNode; 600],
    pub reserved: [u8; 64],
}
