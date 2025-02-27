use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EventHeapHeader {
    pub free_head: u16,
    pub used_head: u16,
    pub count: u16,
    pub padd: u16,
    pub seq_num: u64,
}
