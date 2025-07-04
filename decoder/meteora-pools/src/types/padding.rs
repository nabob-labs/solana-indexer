use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Padding {
    pub padding0: [u8; 6],
    pub padding1: [u64; 21],
    pub padding2: [u64; 21],
}
